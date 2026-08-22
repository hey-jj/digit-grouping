use core::fmt;

use crate::{Format, Grouping, ToFormattedStr};

/// Why parsing rejected an input.
///
/// `byte_index` values point into the input string.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ParseError {
    /// The input is empty, or is only the minus sign.
    Empty,
    /// A scalar that is not a digit of the format's digit set, and not the
    /// separator or minus sign where the grammar allows them.
    InvalidCharacter {
        /// Where the scalar starts.
        byte_index: usize,
    },
    /// A separator in a position the formatter never puts one.
    MisplacedSeparator {
        /// Where the separator starts.
        byte_index: usize,
    },
    /// A digit run long enough to require a separator that has none.
    MissingSeparator {
        /// Where the separator belongs.
        byte_index: usize,
    },
    /// A magnitude longer than one digit that starts with zero.
    LeadingZero,
    /// The minus sign followed by zero.
    NegativeZero,
    /// The minus sign present for an unsigned target type.
    NegativeUnsigned,
    /// The magnitude does not fit the target type.
    Overflow,
    /// Zero parsed into a `NonZeroU*` type.
    Zero,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Empty => f.write_str("empty input"),
            ParseError::InvalidCharacter { byte_index } => {
                write!(f, "invalid character at byte {byte_index}")
            }
            ParseError::MisplacedSeparator { byte_index } => {
                write!(f, "misplaced separator at byte {byte_index}")
            }
            ParseError::MissingSeparator { byte_index } => {
                write!(f, "missing separator at byte {byte_index}")
            }
            ParseError::LeadingZero => f.write_str("leading zero"),
            ParseError::NegativeZero => f.write_str("negative zero"),
            ParseError::NegativeUnsigned => f.write_str("negative value for an unsigned type"),
            ParseError::Overflow => f.write_str("magnitude does not fit the type"),
            ParseError::Zero => f.write_str("zero for a non-zero type"),
        }
    }
}

impl core::error::Error for ParseError {}

/// An integer this crate parses back from its own output. Sealed.
pub trait FromFormattedStr: ToFormattedStr {
    /// Parses a string the formatter emitted for `format` and `Self`.
    ///
    /// Accepts exactly that set of strings. Everything else returns a
    /// [`ParseError`].
    fn from_formatted_str<F: Format>(s: &str, format: &F) -> Result<Self, ParseError>;
}

/// Parses a string the formatter emitted for `format` and `N`.
///
/// The free-function form of [`FromFormattedStr::from_formatted_str`].
///
/// ```
/// use digit_grouping::{parse_formatted, Locale};
///
/// assert_eq!(parse_formatted::<i64, _>("1,23,45,67,890", &Locale::hi), Ok(1234567890));
/// assert!(parse_formatted::<i64, _>("1,234,567,890", &Locale::hi).is_err());
/// ```
pub fn parse_formatted<N: FromFormattedStr, F: Format>(
    s: &str,
    format: &F,
) -> Result<N, ParseError> {
    N::from_formatted_str(s, format)
}

/// Accumulator for the twelve primitive integer types.
pub(crate) trait Accumulate: Sized + Copy {
    const SIGNED: bool;
    const ZERO: Self;
    /// `self * 10 - digit` when negative, `self * 10 + digit` otherwise,
    /// `None` on overflow. Negative values accumulate downward so that
    /// `i128::MIN` parses without overflow.
    fn step(self, digit: u8, negative: bool) -> Option<Self>;
}

macro_rules! impl_accumulate {
    ($signed:literal: $($ty:ty),*) => {$(
        impl Accumulate for $ty {
            const SIGNED: bool = $signed;
            const ZERO: Self = 0;
            fn step(self, digit: u8, negative: bool) -> Option<Self> {
                let acc = self.checked_mul(10)?;
                if negative {
                    acc.checked_sub(digit as $ty)
                } else {
                    acc.checked_add(digit as $ty)
                }
            }
        }
    )*};
}

impl_accumulate!(true: i8, i16, i32, i64, i128, isize);
impl_accumulate!(false: u8, u16, u32, u64, u128, usize);

macro_rules! impl_from_formatted {
    ($($ty:ty),*) => {$(
        impl FromFormattedStr for $ty {
            fn from_formatted_str<F: Format>(
                s: &str,
                format: &F,
            ) -> Result<Self, ParseError> {
                parse_int(s, format)
            }
        }
    )*};
}

impl_from_formatted!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

macro_rules! impl_from_formatted_nonzero {
    ($($nz:ty => $prim:ty),*) => {$(
        impl FromFormattedStr for $nz {
            fn from_formatted_str<F: Format>(
                s: &str,
                format: &F,
            ) -> Result<Self, ParseError> {
                let value: $prim = parse_int(s, format)?;
                <$nz>::new(value).ok_or(ParseError::Zero)
            }
        }
    )*};
}

impl_from_formatted_nonzero!(
    core::num::NonZeroU8 => u8,
    core::num::NonZeroU16 => u16,
    core::num::NonZeroU32 => u32,
    core::num::NonZeroU64 => u64,
    core::num::NonZeroU128 => u128,
    core::num::NonZeroUsize => usize
);

/// One lexed piece of the magnitude.
#[derive(Copy, Clone)]
struct Token {
    /// Digit value, or `SEP`.
    kind: u8,
    /// Absolute byte index of the token's first byte.
    start: usize,
}

const SEP: u8 = 10;

/// Lexes the magnitude part into digit and separator tokens.
#[derive(Copy, Clone)]
struct Lexer<'a> {
    rest: &'a str,
    digits: &'a [&'a str; 10],
    separator: &'a str,
    /// First scalar of the separator, or empty when there is no separator.
    separator_head: &'a str,
    /// Byte offset of `rest` inside the full input.
    base: usize,
    pos: usize,
}

impl<'a> Lexer<'a> {
    fn new<F: Format>(rest: &'a str, format: &'a F, base: usize) -> Lexer<'a> {
        let separator = format.separator();
        let separator_head = match separator.chars().next() {
            Some(c) => &separator[..c.len_utf8()],
            None => "",
        };
        Lexer {
            rest,
            digits: format.digits(),
            separator,
            separator_head,
            base,
            pos: 0,
        }
    }

    /// The next token, `Ok(None)` at the end of input.
    fn next(&mut self) -> Result<Option<Token>, ParseError> {
        let here = &self.rest[self.pos..];
        if here.is_empty() {
            return Ok(None);
        }
        let start = self.base + self.pos;
        for (value, digit) in self.digits.iter().enumerate() {
            if here.starts_with(digit) {
                self.pos += digit.len();
                return Ok(Some(Token {
                    kind: value as u8,
                    start,
                }));
            }
        }
        if !self.separator.is_empty() {
            if here.starts_with(self.separator) {
                self.pos += self.separator.len();
                return Ok(Some(Token { kind: SEP, start }));
            }
            // A partial match of a multi-scalar separator reads as a
            // misplaced separator, not as an unrelated character.
            if here.starts_with(self.separator_head) {
                return Err(ParseError::MisplacedSeparator { byte_index: start });
            }
        }
        Err(ParseError::InvalidCharacter { byte_index: start })
    }

    fn rewind(&mut self) {
        self.pos = 0;
    }
}

/// Grouping geometry for a magnitude of `n` digits.
struct Geometry {
    /// Digit count of the leftmost group.
    head: u32,
    /// Digit count of every group between the first and the last.
    step: u32,
    /// Largest digit count, from the left, after which a separator sits.
    last: u32,
}

impl Geometry {
    fn new(indian: bool, n: u32) -> Geometry {
        if indian {
            let rest = n - 3;
            let head = if rest % 2 == 0 { 2 } else { 1 };
            Geometry {
                head,
                step: 2,
                last: n - 3,
            }
        } else {
            let head = if n % 3 == 0 { 3 } else { n % 3 };
            Geometry {
                head,
                step: 3,
                last: n - 3,
            }
        }
    }

    /// True when the formatter puts a separator after `count` digits.
    fn expects(&self, count: u32) -> bool {
        count >= self.head && count <= self.last && (count - self.head) % self.step == 0
    }
}

/// The parser. Section numbers refer to the grammar: the accepted set is
/// exactly what the formatter emits for this format and type.
fn parse_int<T: Accumulate, F: Format>(s: &str, format: &F) -> Result<T, ParseError> {
    if s.is_empty() {
        return Err(ParseError::Empty);
    }
    let minus = format.minus_sign();
    let (negative, rest, base) = match s.strip_prefix(minus) {
        Some(rest) => (true, rest, minus.len()),
        None => (false, s, 0),
    };
    if rest.is_empty() {
        return Err(ParseError::Empty);
    }
    if negative && !T::SIGNED {
        return Err(ParseError::NegativeUnsigned);
    }

    // Pass 1: lex the whole magnitude, counting digits and separators and
    // accumulating the value. Lexing errors win over everything later.
    let mut lexer = Lexer::new(rest, format, base);
    let mut n: u32 = 0;
    let mut separators: u32 = 0;
    let mut first_separator: usize = 0;
    let mut leading_zero = false;
    let mut acc = Some(T::ZERO);
    while let Some(token) = lexer.next()? {
        if token.kind == SEP {
            if separators == 0 {
                first_separator = token.start;
            }
            separators += 1;
        } else {
            if n == 0 && token.kind == 0 {
                leading_zero = true;
            }
            n += 1;
            acc = acc.and_then(|a| a.step(token.kind, negative));
        }
    }
    if leading_zero && n > 1 {
        return Err(ParseError::LeadingZero);
    }
    if negative && n == 1 && leading_zero {
        return Err(ParseError::NegativeZero);
    }
    // Overflow outranks separator placement: `1234` into u8 is Overflow.
    let value = acc.ok_or(ParseError::Overflow)?;

    let active = !matches!(format.grouping(), Grouping::Posix)
        && !format.separator().is_empty()
        && n >= 3 + u32::from(format.minimum_grouping_digits());
    if separators > 0 && !active {
        return Err(ParseError::MisplacedSeparator {
            byte_index: first_separator,
        });
    }
    if active {
        check_structure(lexer, format.grouping(), n, separators)?;
    }
    Ok(value)
}

/// Separator placement, three passes over the tokens.
///
/// Pass 2 checks local shape: every separator follows a digit, and every
/// group it opens has the group size, with the last group at least three.
/// Pass 3 checks each separator against the positions the formatter uses for
/// this digit count. Pass 4 reports the first such position with no
/// separator. Error indexes point at the leftmost separator of the group
/// that breaks the rule.
fn check_structure(
    mut lexer: Lexer<'_>,
    grouping: Grouping,
    n: u32,
    separators: u32,
) -> Result<(), ParseError> {
    let indian = matches!(grouping, Grouping::Indian);
    let step = if indian { 2 } else { 3 };

    // Pass 2: local shape.
    lexer.rewind();
    let mut previous_is_digit = false;
    let mut group_len: u32 = 0;
    let mut group_index: u32 = 0;
    let mut previous_separator: usize = 0;
    let mut last_separator: usize = 0;
    while let Some(token) = next_lexed(&mut lexer) {
        if token.kind == SEP {
            if !previous_is_digit {
                return Err(ParseError::MisplacedSeparator {
                    byte_index: token.start,
                });
            }
            if group_index > 0 && group_len != step {
                return Err(ParseError::MisplacedSeparator {
                    byte_index: previous_separator,
                });
            }
            previous_is_digit = false;
            previous_separator = token.start;
            last_separator = token.start;
            group_len = 0;
            group_index += 1;
        } else {
            previous_is_digit = true;
            group_len += 1;
        }
    }
    if separators > 0 && group_len < 3 {
        // Trailing separator, or a last group shorter than three digits.
        return Err(ParseError::MisplacedSeparator {
            byte_index: last_separator,
        });
    }

    // Pass 3: every separator sits where the formatter puts one for n digits.
    let geometry = Geometry::new(indian, n);
    lexer.rewind();
    let mut digits_seen: u32 = 0;
    while let Some(token) = next_lexed(&mut lexer) {
        if token.kind == SEP {
            if !geometry.expects(digits_seen) {
                return Err(ParseError::MisplacedSeparator {
                    byte_index: token.start,
                });
            }
        } else {
            digits_seen += 1;
        }
    }

    // Pass 4: every expected position has a separator.
    lexer.rewind();
    let mut digits_seen: u32 = 0;
    let mut expected = geometry.head;
    while let Some(token) = next_lexed(&mut lexer) {
        if token.kind == SEP {
            // Pass 3 proved this position is expected; consume it.
            expected += geometry.step;
        } else {
            if digits_seen == expected && expected <= geometry.last {
                return Err(ParseError::MissingSeparator {
                    byte_index: token.start,
                });
            }
            digits_seen += 1;
        }
    }
    Ok(())
}

/// Pass 1 validated the input, so later passes cannot hit a lexing error.
fn next_lexed(lexer: &mut Lexer<'_>) -> Option<Token> {
    match lexer.next() {
        Ok(token) => token,
        Err(_) => unreachable!("token stream was validated in pass 1"),
    }
}
