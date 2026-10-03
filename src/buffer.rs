use core::borrow::Borrow;
use core::fmt;
use core::ops::Deref;

use crate::write::write_num;
use crate::{Format, ToFormattedStr};

pub(crate) const CAPACITY: usize = 192;

/// A fixed-capacity stack buffer. No heap, `Copy`.
///
/// The longest integer output has 39 digits. Formatting those digits
/// under an Indian-grouping custom format with eight-byte separators and an
/// eight-byte minus sign is 191 bytes.
///
/// ```
/// use digit_grouping::{Buffer, Locale};
///
/// let mut buf = Buffer::new();
/// assert_eq!(buf.write_formatted(-1234567, &Locale::en), "-1,234,567");
/// assert_eq!(buf.as_str(), "-1,234,567");
/// assert_eq!(buf.len(), 10);
/// ```
#[derive(Copy, Clone)]
pub struct Buffer {
    bytes: [u8; CAPACITY],
    len: u8,
}

impl Buffer {
    /// The output capacity in bytes.
    pub const CAPACITY: usize = CAPACITY;

    /// An empty buffer.
    pub const fn new() -> Buffer {
        Buffer {
            bytes: [0; CAPACITY],
            len: 0,
        }
    }

    /// Formats `n` with `format`, replacing the previous content, and
    /// returns the formatted view without touching the heap.
    ///
    /// The formatter fills the array from the right, so the content lives at
    /// the tail and needs no move afterwards.
    pub fn write_formatted<N: ToFormattedStr, F: Format>(&mut self, n: N, format: &F) -> &str {
        let (negative, magnitude) = crate::sealed::Int::split(n);
        let start = write_num(&mut self.bytes, negative, magnitude, format);
        self.len = (CAPACITY - start) as u8;
        self.as_str()
    }

    /// The current content.
    pub fn as_str(&self) -> &str {
        match core::str::from_utf8(self.as_bytes()) {
            Ok(s) => s,
            // The buffer is only written by the formatter from `&str`
            // components, so the content is valid UTF-8 by construction.
            Err(_) => unreachable!("buffer content comes from &str components"),
        }
    }

    /// The current content as bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[CAPACITY - self.len as usize..]
    }

    /// The content length in bytes.
    pub const fn len(&self) -> usize {
        self.len as usize
    }

    /// True when the buffer holds nothing.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Empties the buffer.
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Stores `bytes` as the content, at the tail like the formatter does.
    #[cfg(feature = "float")]
    pub(crate) fn set_content(&mut self, bytes: &[u8]) {
        self.bytes[CAPACITY - bytes.len()..].copy_from_slice(bytes);
        self.len = bytes.len() as u8;
    }
}

impl Default for Buffer {
    fn default() -> Buffer {
        Buffer::new()
    }
}

impl Deref for Buffer {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for Buffer {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for Buffer {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl PartialEq<str> for Buffer {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for Buffer {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
