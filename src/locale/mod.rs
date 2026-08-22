use core::fmt;

use crate::{sealed, Format, Grouping};

mod table;

pub use table::{CLDR_INPUT_SHA256, CLDR_SOURCE_URL, CLDR_VERSION};

/// One locale's symbols, digit set and grouping rule.
pub(crate) struct Entry {
    pub grouping: Grouping,
    pub separator: &'static str,
    pub minus_sign: &'static str,
    pub decimal: &'static str,
    pub minimum_grouping_digits: u8,
    pub numbering_system: &'static str,
    pub digits: [&'static str; 10],
    pub infinity: &'static str,
    pub nan: &'static str,
}

/// A CLDR locale from the pinned table.
///
/// One variant per curated CLDR id, named with `_` where the id has `-`. The
/// last nine variants are CLDR default-content aliases and share their base
/// locale's data: `en_US` formats exactly like `en`.
///
/// ```
/// use digit_grouping::Locale;
///
/// assert_eq!(Locale::from_name("de-CH"), Ok(Locale::de_CH));
/// assert_eq!(Locale::de_CH.separator(), "'");
/// assert_eq!(Locale::hi.grouping(), digit_grouping::Grouping::Indian);
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
#[rustfmt::skip]
pub enum Locale {
    am, ar, ar_EG, ar_SA, bn, bn_IN, cs, da, de, de_AT, de_CH, el, en, en_CH,
    en_GB, en_IN, en_ZA, es, es_AR, es_MX, fa, fi, fr, fr_CA, fr_CH, gu, he,
    hi, hi_Latn, hu, id, it, ja, ko, mr, nb, nl, pl, pt, pt_PT, ro, ru, sv,
    sw, ta, th, tr, uk, und, ur, vi, zh, zh_Hant, zh_Hant_HK,
    en_US, pt_BR, de_DE, fr_FR, es_ES, zh_Hans_CN, zh_Hant_TW, hi_IN, ja_JP,
}

impl Locale {
    /// Looks up a locale by CLDR id.
    ///
    /// Accepts the exact id with `-` or `_`, case-insensitively, for every
    /// name in [`Locale::available_names`]. Anything else returns
    /// `Err(UnknownLocale)`.
    pub fn from_name(name: &str) -> Result<Locale, UnknownLocale> {
        for (i, candidate) in table::NAMES.iter().enumerate() {
            if eq_folded(name, candidate) {
                return Ok(table::VARIANTS[i]);
            }
        }
        Err(UnknownLocale)
    }

    /// The CLDR id, with `-`: `Locale::zh_Hant_HK.name()` is `"zh-Hant-HK"`.
    pub const fn name(self) -> &'static str {
        table::name_of(self)
    }

    /// Every accepted CLDR id, in variant declaration order.
    pub const fn available_names() -> &'static [&'static str] {
        &table::NAMES
    }

    /// The grouping rule from the locale's standard decimal pattern.
    pub const fn grouping(self) -> Grouping {
        table::entry(self).grouping
    }

    /// The group separator.
    pub const fn separator(self) -> &'static str {
        table::entry(self).separator
    }

    /// The minus sign. Can be more than one scalar: `ar` uses U+200E then
    /// `-`.
    pub const fn minus_sign(self) -> &'static str {
        table::entry(self).minus_sign
    }

    /// The decimal symbol. Used only by the `float` feature.
    pub const fn decimal(self) -> &'static str {
        table::entry(self).decimal
    }

    /// Separators appear only when the digit count is at least three plus
    /// this value: `es` has 2, so 1000 prints as `1000`.
    pub const fn minimum_grouping_digits(self) -> u8 {
        table::entry(self).minimum_grouping_digits
    }

    /// The locale's default numbering system: `"latn"`, `"arab"`,
    /// `"arabext"`, `"beng"` or `"deva"`.
    pub const fn numbering_system(self) -> &'static str {
        table::entry(self).numbering_system
    }

    /// The digit set, indexed by decimal digit value.
    pub const fn digits(self) -> &'static [&'static str; 10] {
        &table::entry(self).digits
    }

    /// The infinity symbol. Used only by the `float` feature.
    pub const fn infinity(self) -> &'static str {
        table::entry(self).infinity
    }

    /// The not-a-number symbol. Used only by the `float` feature.
    pub const fn nan(self) -> &'static str {
        table::entry(self).nan
    }
}

/// Compares a caller-supplied name with a table name, folding ASCII case and
/// treating `_` as `-`.
fn eq_folded(input: &str, name: &str) -> bool {
    if input.len() != name.len() {
        return false;
    }
    input.bytes().zip(name.bytes()).all(|(a, b)| {
        let a = if a == b'_' { b'-' } else { a };
        a.eq_ignore_ascii_case(&b)
    })
}

impl core::str::FromStr for Locale {
    type Err = UnknownLocale;

    fn from_str(s: &str) -> Result<Locale, UnknownLocale> {
        Locale::from_name(s)
    }
}

/// Prints [`Locale::name`].
impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl sealed::Sealed for Locale {}

impl Format for Locale {
    fn grouping(&self) -> Grouping {
        Locale::grouping(*self)
    }
    fn separator(&self) -> &str {
        Locale::separator(*self)
    }
    fn minus_sign(&self) -> &str {
        Locale::minus_sign(*self)
    }
    fn decimal(&self) -> &str {
        Locale::decimal(*self)
    }
    fn minimum_grouping_digits(&self) -> u8 {
        Locale::minimum_grouping_digits(*self)
    }
    fn digits(&self) -> &[&str; 10] {
        Locale::digits(*self)
    }
    #[cfg(feature = "float")]
    fn float_symbols(&self) -> Option<(&str, &str)> {
        Some((self.nan(), self.infinity()))
    }
}

/// The name passed to [`Locale::from_name`] is not in the table.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct UnknownLocale;

impl fmt::Display for UnknownLocale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unknown locale name")
    }
}

impl core::error::Error for UnknownLocale {}
