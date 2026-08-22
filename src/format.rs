use core::fmt;

use crate::{sealed, Grouping, Locale};

/// ASCII digits used by every [`CustomFormat`].
pub(crate) static ASCII_DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

/// A format's symbols and grouping rule. Sealed.
///
/// Implemented by [`Locale`] and [`CustomFormat`].
pub trait Format: sealed::Sealed {
    /// The grouping rule.
    fn grouping(&self) -> Grouping;
    /// The group separator. Empty means no separators.
    fn separator(&self) -> &str;
    /// The minus sign. Emitted before the digits of a negative value.
    fn minus_sign(&self) -> &str;
    /// The decimal symbol. Used only by the `float` feature.
    fn decimal(&self) -> &str;
    /// Separators appear only when the digit count is at least three plus
    /// this value.
    fn minimum_grouping_digits(&self) -> u8;
    /// The digit set, indexed by decimal digit value. ASCII for
    /// [`CustomFormat`].
    fn digits(&self) -> &[&str; 10];

    /// Float-only symbols `(nan, infinity)`. `None` falls back to `NaN` and
    /// U+221E.
    #[cfg(feature = "float")]
    #[doc(hidden)]
    fn float_symbols(&self) -> Option<(&str, &str)> {
        None
    }
}

/// A symbol stored inline: at most eight bytes of UTF-8.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct Sym {
    bytes: [u8; 8],
    len: u8,
}

impl Sym {
    const fn empty() -> Sym {
        Sym {
            bytes: [0; 8],
            len: 0,
        }
    }

    /// Copies from a `&str` of at most eight bytes.
    fn new(s: &str) -> Sym {
        debug_assert!(s.len() <= 8);
        let mut bytes = [0u8; 8];
        bytes[..s.len()].copy_from_slice(s.as_bytes());
        Sym {
            bytes,
            len: s.len() as u8,
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        match core::str::from_utf8(&self.bytes[..self.len as usize]) {
            Ok(s) => s,
            // A Sym is only ever built from a &str, so the bytes are valid
            // UTF-8 by construction.
            Err(_) => unreachable!("Sym bytes come from a &str"),
        }
    }
}

/// A caller-defined format with ASCII digits. Built by
/// [`CustomFormat::builder`], which validates every field.
///
/// ```
/// use digit_grouping::{Buffer, CustomFormat, Grouping};
///
/// let underscores = CustomFormat::builder().separator("_").build().unwrap();
/// let mut buf = digit_grouping::Buffer::new();
/// assert_eq!(buf.write_formatted(1234567, &underscores), "1_234_567");
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct CustomFormat {
    grouping: Grouping,
    separator: Sym,
    minus_sign: Sym,
    decimal: Sym,
    minimum_grouping_digits: u8,
}

impl CustomFormat {
    /// Starts a builder holding the [`Default`] values.
    pub fn builder() -> CustomFormatBuilder {
        CustomFormatBuilder {
            grouping: Grouping::Standard,
            separator: Field::new(","),
            minus_sign: Field::new("-"),
            decimal: Field::new("."),
            minimum_grouping_digits: 1,
        }
    }

    /// The grouping rule.
    pub fn grouping(&self) -> Grouping {
        self.grouping
    }

    /// The group separator. Empty means no separators.
    pub fn separator(&self) -> &str {
        self.separator.as_str()
    }

    /// The minus sign.
    pub fn minus_sign(&self) -> &str {
        self.minus_sign.as_str()
    }

    /// The decimal symbol. Used only by the `float` feature.
    pub fn decimal(&self) -> &str {
        self.decimal.as_str()
    }

    /// Separators appear only when the digit count is at least three plus
    /// this value.
    pub fn minimum_grouping_digits(&self) -> u8 {
        self.minimum_grouping_digits
    }
}

/// [`Grouping::Standard`], separator `,`, minus sign `-`, decimal `.`,
/// minimum grouping digits 1.
impl Default for CustomFormat {
    fn default() -> CustomFormat {
        CustomFormat {
            grouping: Grouping::Standard,
            separator: Sym::new(","),
            minus_sign: Sym::new("-"),
            decimal: Sym::new("."),
            minimum_grouping_digits: 1,
        }
    }
}

/// Copies the locale's grouping rule, separator, minus sign, decimal symbol
/// and minimum grouping digits. The digit set becomes ASCII.
impl From<Locale> for CustomFormat {
    fn from(locale: Locale) -> CustomFormat {
        CustomFormat {
            grouping: locale.grouping(),
            separator: Sym::new(locale.separator()),
            minus_sign: Sym::new(locale.minus_sign()),
            decimal: Sym::new(locale.decimal()),
            minimum_grouping_digits: locale.minimum_grouping_digits(),
        }
    }
}

impl sealed::Sealed for CustomFormat {}

impl Format for CustomFormat {
    fn grouping(&self) -> Grouping {
        self.grouping
    }
    fn separator(&self) -> &str {
        self.separator.as_str()
    }
    fn minus_sign(&self) -> &str {
        self.minus_sign.as_str()
    }
    fn decimal(&self) -> &str {
        self.decimal.as_str()
    }
    fn minimum_grouping_digits(&self) -> u8 {
        self.minimum_grouping_digits
    }
    fn digits(&self) -> &[&str; 10] {
        &ASCII_DIGITS
    }
}

/// A candidate symbol for the builder. Values over eight bytes keep only the
/// over-length flag, which [`CustomFormatBuilder::build`] turns into an error.
#[derive(Copy, Clone, Debug)]
struct Field {
    sym: Sym,
    too_long: bool,
}

impl Field {
    fn new(s: &str) -> Field {
        if s.len() > 8 {
            Field {
                sym: Sym::empty(),
                too_long: true,
            }
        } else {
            Field {
                sym: Sym::new(s),
                too_long: false,
            }
        }
    }

    fn contains_ascii_digit(&self) -> bool {
        self.sym.as_str().bytes().any(|b| b.is_ascii_digit())
    }
}

/// Builds a [`CustomFormat`], validating on [`build`](Self::build).
#[derive(Clone, Debug)]
pub struct CustomFormatBuilder {
    grouping: Grouping,
    separator: Field,
    minus_sign: Field,
    decimal: Field,
    minimum_grouping_digits: u8,
}

impl CustomFormatBuilder {
    /// Sets the grouping rule.
    pub fn grouping(mut self, grouping: Grouping) -> Self {
        self.grouping = grouping;
        self
    }

    /// Sets the group separator: at most eight bytes, free of ASCII digits,
    /// and distinct from the decimal symbol. Empty means no separators.
    pub fn separator(mut self, s: &str) -> Self {
        self.separator = Field::new(s);
        self
    }

    /// Sets the minus sign: one to eight bytes, no ASCII digit.
    pub fn minus_sign(mut self, s: &str) -> Self {
        self.minus_sign = Field::new(s);
        self
    }

    /// Sets the decimal symbol: one to eight bytes, no ASCII digit.
    pub fn decimal(mut self, s: &str) -> Self {
        self.decimal = Field::new(s);
        self
    }

    /// Sets the minimum grouping digits: at least 1.
    pub fn minimum_grouping_digits(mut self, n: u8) -> Self {
        self.minimum_grouping_digits = n;
        self
    }

    /// Validates the fields and builds the format.
    pub fn build(self) -> Result<CustomFormat, FormatError> {
        if self.separator.too_long {
            return Err(FormatError::SeparatorTooLong);
        }
        if self.minus_sign.too_long {
            return Err(FormatError::MinusSignTooLong);
        }
        if self.minus_sign.sym.len == 0 {
            return Err(FormatError::MinusSignEmpty);
        }
        if self.decimal.too_long {
            return Err(FormatError::DecimalTooLong);
        }
        if self.decimal.sym.len == 0 {
            return Err(FormatError::DecimalEmpty);
        }
        if self.separator.contains_ascii_digit()
            || self.minus_sign.contains_ascii_digit()
            || self.decimal.contains_ascii_digit()
        {
            return Err(FormatError::ContainsDigit);
        }
        if self.separator.sym == self.decimal.sym {
            return Err(FormatError::SeparatorEqualsDecimal);
        }
        if self.minimum_grouping_digits == 0 {
            return Err(FormatError::MinimumGroupingZero);
        }
        Ok(CustomFormat {
            grouping: self.grouping,
            separator: self.separator.sym,
            minus_sign: self.minus_sign.sym,
            decimal: self.decimal.sym,
            minimum_grouping_digits: self.minimum_grouping_digits,
        })
    }
}

/// Why [`CustomFormatBuilder::build`] rejected its input.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FormatError {
    /// The separator is longer than eight bytes.
    SeparatorTooLong,
    /// The minus sign is empty.
    MinusSignEmpty,
    /// The minus sign is longer than eight bytes.
    MinusSignTooLong,
    /// The decimal symbol is empty.
    DecimalEmpty,
    /// The decimal symbol is longer than eight bytes.
    DecimalTooLong,
    /// The separator, minus sign or decimal symbol contains an ASCII digit.
    ContainsDigit,
    /// The separator equals the decimal symbol.
    SeparatorEqualsDecimal,
    /// The minimum grouping digits value is zero.
    MinimumGroupingZero,
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            FormatError::SeparatorTooLong => "separator is longer than 8 bytes",
            FormatError::MinusSignEmpty => "minus sign is empty",
            FormatError::MinusSignTooLong => "minus sign is longer than 8 bytes",
            FormatError::DecimalEmpty => "decimal symbol is empty",
            FormatError::DecimalTooLong => "decimal symbol is longer than 8 bytes",
            FormatError::ContainsDigit => "symbol contains an ASCII digit",
            FormatError::SeparatorEqualsDecimal => "separator equals the decimal symbol",
            FormatError::MinimumGroupingZero => "minimum grouping digits is zero",
        };
        f.write_str(msg)
    }
}

impl core::error::Error for FormatError {}
