//! Fixed-precision float formatting, behind the `float` feature.
//!
//! Digits come from `core::fmt` with `{:.precision$}`, so rounding matches
//! `format!("{:.2}", x)` exactly. The integer part is grouped by the same
//! rules as the integer path, the `.` becomes the format's decimal symbol,
//! and fraction digits use the format's digit set.

use core::fmt;

use crate::buffer::CAPACITY;
use crate::{sealed, Buffer, Format, Grouping};

/// The formatted float does not fit the 192-byte [`Buffer`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BufferOverflow;

impl fmt::Display for BufferOverflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("formatted float does not fit the buffer")
    }
}

impl core::error::Error for BufferOverflow {}

/// A float this crate formats: `f32` and `f64`. Sealed.
pub trait ToFormattedFloat: sealed::Sealed + Copy {
    /// Writes `{:.precision$}` of `self`.
    #[doc(hidden)]
    fn write_raw<W: fmt::Write>(self, precision: usize, sink: &mut W) -> fmt::Result;

    /// Not-a-number, or infinity with its sign.
    #[doc(hidden)]
    fn special(self) -> Option<Special>;
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum Special {
    Nan,
    Infinite { negative: bool },
}

macro_rules! impl_float {
    ($($ty:ty),*) => {$(
        impl sealed::Sealed for $ty {}
        impl ToFormattedFloat for $ty {
            fn write_raw<W: fmt::Write>(
                self,
                precision: usize,
                sink: &mut W,
            ) -> fmt::Result {
                write!(sink, "{self:.precision$}")
            }

            fn special(self) -> Option<Special> {
                if self.is_nan() {
                    Some(Special::Nan)
                } else if self.is_infinite() {
                    Some(Special::Infinite { negative: self < 0.0 })
                } else {
                    None
                }
            }
        }
    )*};
}

impl_float!(f32, f64);

/// A bounded sink for the raw `{:.precision$}` digits. Anything the buffer
/// cannot hold cannot fit the buffer after grouping either, because grouping
/// never shrinks the byte count.
struct Raw {
    bytes: [u8; CAPACITY],
    len: usize,
}

impl fmt::Write for Raw {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let end = self.len.checked_add(s.len()).ok_or(fmt::Error)?;
        if end > CAPACITY {
            return Err(fmt::Error);
        }
        self.bytes[self.len..end].copy_from_slice(s.as_bytes());
        self.len = end;
        Ok(())
    }
}

/// Writes the grouped rendering of the raw `{:.precision$}` string.
fn group_raw<W: fmt::Write + ?Sized, F: Format>(
    sink: &mut W,
    raw: &str,
    format: &F,
) -> fmt::Result {
    let (negative, magnitude) = match raw.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, raw),
    };
    let (integer, fraction) = match magnitude.split_once('.') {
        Some((i, f)) => (i, Some(f)),
        None => (magnitude, None),
    };
    let digits = format.digits();
    let n = integer.len() as u32;
    let indian = matches!(format.grouping(), Grouping::Indian);
    let active = !matches!(format.grouping(), Grouping::Posix)
        && !format.separator().is_empty()
        && n >= 3 + u32::from(format.minimum_grouping_digits());

    if negative {
        sink.write_str(format.minus_sign())?;
    }
    for (index, byte) in integer.bytes().enumerate() {
        sink.write_str(digits[usize::from(byte - b'0')])?;
        let remaining = n - index as u32 - 1;
        let boundary = if indian {
            remaining >= 3 && (remaining - 3) % 2 == 0
        } else {
            remaining % 3 == 0
        };
        if active && remaining > 0 && boundary {
            sink.write_str(format.separator())?;
        }
    }
    if let Some(fraction) = fraction {
        sink.write_str(format.decimal())?;
        for byte in fraction.bytes() {
            sink.write_str(digits[usize::from(byte - b'0')])?;
        }
    }
    Ok(())
}

/// Counts the integer digits in a raw fixed-precision rendering.
struct RawShape {
    integer_digits: u32,
    fraction: bool,
}

impl fmt::Write for RawShape {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            match byte {
                b'-' => {}
                b'.' => self.fraction = true,
                b'0'..=b'9' if !self.fraction => {
                    self.integer_digits = self.integer_digits.checked_add(1).ok_or(fmt::Error)?;
                }
                b'0'..=b'9' => {}
                _ => return Err(fmt::Error),
            }
        }
        Ok(())
    }
}

/// Groups a raw fixed-precision rendering as `core::fmt` writes it.
struct GroupedRaw<'a, W: fmt::Write + ?Sized, F: Format> {
    sink: &'a mut W,
    format: &'a F,
    integer_digits: u32,
    integer_index: u32,
    fraction: bool,
}

impl<W: fmt::Write + ?Sized, F: Format> fmt::Write for GroupedRaw<'_, W, F> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let digits = self.format.digits();
        let indian = matches!(self.format.grouping(), Grouping::Indian);
        let active = !matches!(self.format.grouping(), Grouping::Posix)
            && !self.format.separator().is_empty()
            && self.integer_digits >= 3 + u32::from(self.format.minimum_grouping_digits());

        for byte in s.bytes() {
            match byte {
                b'-' => self.sink.write_str(self.format.minus_sign())?,
                b'.' => {
                    self.fraction = true;
                    self.sink.write_str(self.format.decimal())?;
                }
                b'0'..=b'9' if self.fraction => {
                    self.sink.write_str(digits[usize::from(byte - b'0')])?;
                }
                b'0'..=b'9' => {
                    self.sink.write_str(digits[usize::from(byte - b'0')])?;
                    self.integer_index += 1;
                    let remaining = self.integer_digits - self.integer_index;
                    let boundary = if indian {
                        remaining >= 3 && (remaining - 3) % 2 == 0
                    } else {
                        remaining % 3 == 0
                    };
                    if active && remaining > 0 && boundary {
                        self.sink.write_str(self.format.separator())?;
                    }
                }
                _ => return Err(fmt::Error),
            }
        }
        Ok(())
    }
}

/// Writes NaN or infinity symbols.
fn write_special<W: fmt::Write + ?Sized, F: Format>(
    sink: &mut W,
    special: Special,
    format: &F,
    nan: &str,
    infinity: &str,
) -> fmt::Result {
    match special {
        Special::Nan => sink.write_str(nan),
        Special::Infinite { negative } => {
            if negative {
                sink.write_str(format.minus_sign())?;
            }
            sink.write_str(infinity)
        }
    }
}

/// The `nan` and `infinity` symbols for a format. `CustomFormat` has no
/// fields for them, so it falls back to `NaN` and U+221E.
fn symbols<F: Format>(format: &F) -> (&str, &str) {
    format.float_symbols().unwrap_or(("NaN", "\u{221e}"))
}

impl Buffer {
    /// Formats `x` with `precision` fraction digits, replacing the previous
    /// content, and returns the formatted view.
    ///
    /// Returns `Err(BufferOverflow)` and empties the buffer when the result
    /// would exceed [`Buffer::CAPACITY`] bytes. The magnitude of `x` and
    /// `precision` decide that.
    pub fn write_formatted_float<X: ToFormattedFloat, F: Format>(
        &mut self,
        x: X,
        precision: usize,
        format: &F,
    ) -> Result<&str, BufferOverflow> {
        self.clear();
        let mut out = Raw {
            bytes: [0; CAPACITY],
            len: 0,
        };
        let result = match x.special() {
            Some(special) => {
                let (nan, infinity) = symbols(format);
                write_special(&mut out, special, format, nan, infinity)
            }
            None => {
                let mut raw = Raw {
                    bytes: [0; CAPACITY],
                    len: 0,
                };
                match x.write_raw(precision, &mut raw) {
                    Ok(()) => {
                        let raw = match core::str::from_utf8(&raw.bytes[..raw.len]) {
                            Ok(s) => s,
                            Err(_) => unreachable!("core::fmt writes UTF-8"),
                        };
                        group_raw(&mut out, raw, format)
                    }
                    Err(fmt::Error) => Err(fmt::Error),
                }
            }
        };
        match result {
            Ok(()) => {
                let bytes = out.bytes;
                self.set_content(&bytes[..out.len]);
                Ok(self.as_str())
            }
            Err(fmt::Error) => Err(BufferOverflow),
        }
    }
}

/// Extends [`WriteFormatted`] sinks with the float path.
pub(crate) fn write_formatted_float_into<W, X, F>(
    sink: &mut W,
    x: X,
    precision: usize,
    format: &F,
) -> fmt::Result
where
    W: fmt::Write + ?Sized,
    X: ToFormattedFloat,
    F: Format,
{
    match x.special() {
        Some(special) => {
            let (nan, infinity) = symbols(format);
            write_special(sink, special, format, nan, infinity)
        }
        None => {
            let mut shape = RawShape {
                integer_digits: 0,
                fraction: false,
            };
            x.write_raw(precision, &mut shape)?;
            let mut grouped = GroupedRaw {
                sink,
                format,
                integer_digits: shape.integer_digits,
                integer_index: 0,
                fraction: false,
            };
            x.write_raw(precision, &mut grouped)
        }
    }
}

/// Formats `x` into a new `String` with `precision` fraction digits.
///
/// Requires the `alloc` feature next to `float`. The `String` grows to hold
/// every digit produced by `core::fmt`.
#[cfg(feature = "alloc")]
pub fn to_formatted_string_float<X: ToFormattedFloat, F: Format>(
    x: X,
    precision: usize,
    format: &F,
) -> Result<alloc::string::String, BufferOverflow> {
    let mut out = alloc::string::String::new();
    match write_formatted_float_into(&mut out, x, precision, format) {
        Ok(()) => Ok(out),
        Err(fmt::Error) => Err(BufferOverflow),
    }
}
