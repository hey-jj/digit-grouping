use core::fmt;

use crate::buffer::CAPACITY;
use crate::{sealed, Format, Grouping};

/// Decimal digit pairs 00 through 99, for two-digits-at-a-time emission.
static LUT: [u8; 200] = *b"0001020304050607080910111213141516171819\
2021222324252627282930313233343536373839\
4041424344454647484950515253545556575859\
6061626364656667686970717273747576777879\
8081828384858687888990919293949596979899";

/// Formats `magnitude` with `format` into the tail of `out`, right to left,
/// and returns the start index of the rendered bytes.
///
/// One pass, no intermediate string: digits are pulled two at a time from
/// the lookup table and written from the right, with the separator inserted
/// on a per-group countdown as the loop goes. The digit stride is selected
/// once, before the loop: an ASCII digit set takes the single-byte path, any
/// other set writes its pre-encoded UTF-8 digit strings.
///
/// The worst case is 191 bytes (39 digits under an Indian format with an
/// eight-byte separator and an eight-byte minus sign), so the result always
/// fits `CAPACITY`.
pub(crate) fn write_num<F: Format>(
    out: &mut [u8; CAPACITY],
    negative: bool,
    magnitude: u128,
    format: &F,
) -> usize {
    let digits = format.digits();
    let separator = format.separator().as_bytes();
    let grouping = format.grouping();
    // n >= 3 + mgd digits means magnitude >= 10^(mgd + 2). Past index 38
    // no u128 qualifies.
    let active = !matches!(grouping, Grouping::Posix)
        && !separator.is_empty()
        && POW10
            .get(usize::from(format.minimum_grouping_digits()) + 2)
            .is_some_and(|threshold| magnitude >= *threshold);
    // Group size left of the rightmost group of three.
    let step: u32 = match grouping {
        Grouping::Indian => 2,
        Grouping::Standard | Grouping::Posix => 3,
    };
    let ascii = digits[0].as_bytes() == b"0";

    let mut pos = out.len();
    // Digits left before the next separator. Grouping off never reaches 0.
    let mut until: u32 = if active { 3 } else { u32::MAX };

    macro_rules! put_bytes {
        ($bytes:expr) => {{
            let bytes = $bytes;
            pos -= bytes.len();
            out[pos..pos + bytes.len()].copy_from_slice(bytes);
        }};
    }
    macro_rules! put_separator {
        () => {{
            if until == 0 {
                if let [byte] = separator {
                    pos -= 1;
                    out[pos] = *byte;
                } else {
                    put_bytes!(separator);
                }
                until = step;
            }
            until -= 1;
        }};
    }
    // `$byte` is the digit as an ASCII byte from the LUT.
    macro_rules! put_ascii {
        ($byte:expr) => {{
            put_separator!();
            pos -= 1;
            out[pos] = $byte;
        }};
    }
    macro_rules! put_table {
        ($byte:expr) => {{
            put_separator!();
            put_bytes!(digits[usize::from($byte - b'0')].as_bytes());
        }};
    }
    macro_rules! digit_loop {
        ($ty:ty, $value:expr, $put:ident) => {{
            let mut m: $ty = $value;
            loop {
                if m >= 100 {
                    let r = ((m % 100) as usize) << 1;
                    m /= 100;
                    $put!(LUT[r + 1]);
                    $put!(LUT[r]);
                } else if m >= 10 {
                    let r = (m as usize) << 1;
                    $put!(LUT[r + 1]);
                    $put!(LUT[r]);
                    break;
                } else {
                    $put!(b'0' + m as u8);
                    break;
                }
            }
        }};
    }
    // Exactly eight digits, zero padded. Splitting the value into fixed
    // chunks keeps the division chains short and independent.
    macro_rules! put_chunk8 {
        ($value:expr, $put:ident) => {{
            let chunk = $value;
            let r0 = ((chunk % 100) as usize) << 1;
            let chunk = chunk / 100;
            let r1 = ((chunk % 100) as usize) << 1;
            let chunk = chunk / 100;
            let r2 = ((chunk % 100) as usize) << 1;
            let r3 = ((chunk / 100) as usize) << 1;
            $put!(LUT[r0 + 1]);
            $put!(LUT[r0]);
            $put!(LUT[r1 + 1]);
            $put!(LUT[r1]);
            $put!(LUT[r2 + 1]);
            $put!(LUT[r2]);
            $put!(LUT[r3 + 1]);
            $put!(LUT[r3]);
        }};
    }
    macro_rules! digit_loop_u64 {
        ($value:expr, $put:ident) => {{
            let mut m: u64 = $value;
            if m >= 100_000_000 {
                let low = m % 100_000_000;
                m /= 100_000_000;
                if m >= 100_000_000 {
                    let middle = m % 100_000_000;
                    m /= 100_000_000;
                    put_chunk8!(low, $put);
                    put_chunk8!(middle, $put);
                } else {
                    put_chunk8!(low, $put);
                }
            }
            digit_loop!(u64, m, $put);
        }};
    }

    match (ascii, u64::try_from(magnitude)) {
        (true, Ok(small)) => {
            if let (true, 3, [sep]) = (active, step, separator) {
                // Groups of three under a one-byte separator: each group
                // lands as one four-byte block, LUT pair plus leading digit.
                macro_rules! put_group {
                    ($group:expr) => {{
                        let group = $group as usize;
                        let r = (group % 100) << 1;
                        pos -= 4;
                        let block = [*sep, b'0' + (group / 100) as u8, LUT[r], LUT[r + 1]];
                        out[pos..pos + 4].copy_from_slice(&block);
                    }};
                }
                let mut m = small;
                // Nine digits per round keeps the division chains short and
                // group-aligned.
                while m >= 1_000_000_000 {
                    let bottom = (m % 1_000_000_000) as u32;
                    m /= 1_000_000_000;
                    let rest = bottom / 1000;
                    put_group!(bottom % 1000);
                    put_group!(rest % 1000);
                    put_group!(rest / 1000);
                }
                while m >= 1000 {
                    put_group!(m % 1000);
                    m /= 1000;
                }
                // The head group: one to three digits, no separator before
                // it. m stays nonzero because the loop entered at >= 1000.
                if m >= 100 {
                    let r = ((m % 100) as usize) << 1;
                    pos -= 3;
                    let block = [b'0' + (m / 100) as u8, LUT[r], LUT[r + 1]];
                    out[pos..pos + 3].copy_from_slice(&block);
                } else if m >= 10 {
                    let r = (m as usize) << 1;
                    pos -= 2;
                    let block = [LUT[r], LUT[r + 1]];
                    out[pos..pos + 2].copy_from_slice(&block);
                } else {
                    pos -= 1;
                    out[pos] = b'0' + m as u8;
                }
            } else {
                digit_loop_u64!(small, put_ascii);
            }
        }
        (true, Err(_)) => digit_loop!(u128, magnitude, put_ascii),
        (false, Ok(small)) => digit_loop_u64!(small, put_table),
        (false, Err(_)) => digit_loop!(u128, magnitude, put_table),
    }
    // The countdown ends wherever the leftmost group ends.
    let _ = until;
    if negative {
        put_bytes!(format.minus_sign().as_bytes());
    }
    pos
}

/// Powers of ten through the largest below `u128::MAX`.
const POW10: [u128; 39] = {
    let mut table = [1u128; 39];
    let mut i = 1;
    while i < 39 {
        table[i] = table[i - 1] * 10;
        i += 1;
    }
    table
};

/// Renders into `buf` and hands back the `&str` view.
pub(crate) fn render<'a, N: ToFormattedStr, F: Format>(
    buf: &'a mut [u8; CAPACITY],
    n: N,
    format: &F,
) -> &'a str {
    let (negative, magnitude) = n.split();
    let start = write_num(buf, negative, magnitude, format);
    match core::str::from_utf8(&buf[start..]) {
        Ok(s) => s,
        // Every byte comes from a `&str` in the format, so the content is
        // valid UTF-8 by construction.
        Err(_) => unreachable!("formatted bytes come from &str components"),
    }
}

/// An integer this crate formats: the twelve primitive integer types and the
/// six `NonZeroU*` types. Sealed.
pub trait ToFormattedStr: sealed::Int + Copy {}

macro_rules! impl_int {
    ($($ty:ty),*) => {$(
        impl sealed::Sealed for $ty {}
        impl sealed::Int for $ty {
            fn split(self) -> (bool, u128) {
                (self < 0, self.unsigned_abs() as u128)
            }
        }
        impl ToFormattedStr for $ty {}
    )*};
}

macro_rules! impl_uint {
    ($($ty:ty),*) => {$(
        impl sealed::Sealed for $ty {}
        impl sealed::Int for $ty {
            fn split(self) -> (bool, u128) {
                (false, self as u128)
            }
        }
        impl ToFormattedStr for $ty {}
    )*};
}

macro_rules! impl_nonzero {
    ($($ty:ty),*) => {$(
        impl sealed::Sealed for $ty {}
        impl sealed::Int for $ty {
            fn split(self) -> (bool, u128) {
                (false, self.get() as u128)
            }
        }
        impl ToFormattedStr for $ty {}
    )*};
}

impl_int!(i8, i16, i32, i64, i128, isize);
impl_uint!(u8, u16, u32, u64, u128, usize);
impl_nonzero!(
    core::num::NonZeroU8,
    core::num::NonZeroU16,
    core::num::NonZeroU32,
    core::num::NonZeroU64,
    core::num::NonZeroU128,
    core::num::NonZeroUsize
);

/// Formats into any [`core::fmt::Write`] sink.
///
/// Implemented for every `W: core::fmt::Write`, so a `String` or a
/// `core::fmt::Formatter` works directly.
pub trait WriteFormatted: fmt::Write {
    /// Writes `n` formatted with `format`.
    fn write_formatted<N: ToFormattedStr, F: Format>(&mut self, n: N, format: &F) -> fmt::Result;

    /// Writes `x` with `precision` fraction digits, formatted with `format`.
    ///
    /// Requires the `float` feature. Returns the sink's error if it rejects
    /// a write.
    #[cfg(feature = "float")]
    fn write_formatted_float<X: crate::ToFormattedFloat, F: Format>(
        &mut self,
        x: X,
        precision: usize,
        format: &F,
    ) -> fmt::Result;
}

impl<W: fmt::Write + ?Sized> WriteFormatted for W {
    fn write_formatted<N: ToFormattedStr, F: Format>(&mut self, n: N, format: &F) -> fmt::Result {
        let mut buf = [0u8; CAPACITY];
        self.write_str(render(&mut buf, n, format))
    }

    #[cfg(feature = "float")]
    fn write_formatted_float<X: crate::ToFormattedFloat, F: Format>(
        &mut self,
        x: X,
        precision: usize,
        format: &F,
    ) -> fmt::Result {
        crate::float::write_formatted_float_into(self, x, precision, format)
    }
}

/// Formats into a new `String`. Requires the `alloc` feature.
#[cfg(feature = "alloc")]
pub trait ToFormattedString: ToFormattedStr {
    /// Returns `self` formatted with `format`.
    fn to_formatted_string<F: Format>(self, format: &F) -> alloc::string::String;
}

#[cfg(feature = "alloc")]
impl<N: ToFormattedStr> ToFormattedString for N {
    fn to_formatted_string<F: Format>(self, format: &F) -> alloc::string::String {
        let mut buf = [0u8; CAPACITY];
        alloc::string::String::from(render(&mut buf, self, format))
    }
}

/// A [`Display`](core::fmt::Display) adapter, so
/// `format!("{}", formatted(5000, &Locale::en))` works without a buffer.
///
/// ```
/// use digit_grouping::{formatted, Locale};
///
/// assert_eq!(
///     std::format!("{}", formatted(5000, &Locale::en)),
///     "5,000",
/// );
/// ```
pub fn formatted<N: ToFormattedStr, F: Format>(n: N, format: &F) -> Formatted<'_, N, F> {
    Formatted { value: n, format }
}

/// The adapter returned by [`formatted`].
pub struct Formatted<'a, N, F> {
    value: N,
    format: &'a F,
}

impl<N: ToFormattedStr, F: Format> fmt::Display for Formatted<'_, N, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = [0u8; CAPACITY];
        f.write_str(render(&mut buf, self.value, self.format))
    }
}
