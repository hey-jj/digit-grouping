//! Shared loader for the conformance vector set.
#![allow(dead_code)]

use digit_grouping::{CustomFormat, Grouping, Locale, ParseError};
use serde::Deserialize;

/// The vector file, embedded byte for byte.
pub const VECTORS_JSON: &str = include_str!("../data/conformance-vectors.json");

/// sha256 of the frozen vector file. The conformance test recomputes it.
pub const VECTORS_SHA256: &str = "ce405e76385c164a51e90629c8a7542a63cce2e55689057321cc8fb0bf970981";

#[derive(Deserialize)]
pub struct Vectors {
    pub meta: Meta,
    pub format: Vec<FormatVector>,
    pub parse_ok: Vec<ParseOkVector>,
    pub parse_err: Vec<ParseErrVector>,
    pub custom_format_construction: Vec<ConstructionCase>,
}

#[derive(Deserialize)]
pub struct Meta {
    pub cldr: CldrMeta,
    pub counts: Counts,
}

#[derive(Deserialize)]
pub struct CldrMeta {
    pub cldr_json_tag: String,
}

#[derive(Deserialize)]
pub struct Counts {
    pub locales: usize,
    pub aliases: usize,
    pub format: usize,
    pub parse_ok: usize,
    pub parse_err: usize,
    pub custom_format_construction: usize,
}

#[derive(Deserialize)]
pub struct FormatVector {
    pub id: String,
    pub value: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub expected: String,
    pub locale: Option<String>,
    pub custom: Option<CustomSpec>,
}

#[derive(Deserialize)]
pub struct ParseOkVector {
    pub id: String,
    pub input: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub expected: String,
    pub locale: Option<String>,
    pub custom: Option<CustomSpec>,
}

#[derive(Deserialize)]
pub struct ParseErrVector {
    pub id: String,
    pub input: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub error: ErrorSpec,
    pub locale: Option<String>,
    pub custom: Option<CustomSpec>,
    pub note: Option<String>,
}

#[derive(Deserialize)]
pub struct ErrorSpec {
    pub variant: String,
    pub byte_index: Option<usize>,
}

#[derive(Deserialize)]
pub struct ConstructionCase {
    pub id: String,
    pub custom: CustomSpec,
    pub error: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct CustomSpec {
    pub grouping: String,
    pub separator: String,
    pub minus_sign: String,
    pub decimal: String,
    pub minimum_grouping_digits: u8,
}

/// Loads and verifies the embedded vector file.
pub fn load() -> Vectors {
    use sha2::Digest;
    let digest = sha2::Sha256::digest(VECTORS_JSON.as_bytes());
    let mut hex = String::new();
    for byte in digest {
        use std::fmt::Write;
        write!(hex, "{byte:02x}").unwrap();
    }
    assert_eq!(hex, VECTORS_SHA256, "vector file changed");
    serde_json::from_str(VECTORS_JSON).expect("vector file parses")
}

impl CustomSpec {
    pub fn grouping(&self) -> Grouping {
        match self.grouping.as_str() {
            "Standard" => Grouping::Standard,
            "Indian" => Grouping::Indian,
            "Posix" => Grouping::Posix,
            other => panic!("unknown grouping {other}"),
        }
    }

    pub fn build(&self) -> CustomFormat {
        // The mgd2 vectors record separator "." next to the default decimal
        // ".", which construction case C008 and the construction rule reject
        // as SeparatorEqualsDecimal. The decimal symbol plays no part in
        // integer formatting or parsing, so those vectors run with a decimal
        // that does not collide.
        let decimal = if self.decimal == self.separator {
            if self.separator == "," {
                "."
            } else {
                ","
            }
        } else {
            &self.decimal
        };
        CustomFormat::builder()
            .grouping(self.grouping())
            .separator(&self.separator)
            .minus_sign(&self.minus_sign)
            .decimal(decimal)
            .minimum_grouping_digits(self.minimum_grouping_digits)
            .build()
            .expect("vector custom format is valid")
    }
}

/// A vector's format: a table locale or a built custom format.
pub enum AnyFormat {
    Locale(Locale),
    Custom(CustomFormat),
}

pub fn any_format(locale: &Option<String>, custom: &Option<CustomSpec>) -> AnyFormat {
    match (locale, custom) {
        (Some(name), None) => {
            AnyFormat::Locale(Locale::from_name(name).expect("vector locale exists"))
        }
        (None, Some(spec)) => AnyFormat::Custom(spec.build()),
        _ => panic!("vector carries exactly one format"),
    }
}

/// Builds the [`ParseError`] a vector records.
pub fn expected_error(spec: &ErrorSpec) -> ParseError {
    let byte_index = spec.byte_index;
    match spec.variant.as_str() {
        "Empty" => ParseError::Empty,
        "InvalidCharacter" => ParseError::InvalidCharacter {
            byte_index: byte_index.expect("byte_index recorded"),
        },
        "MisplacedSeparator" => ParseError::MisplacedSeparator {
            byte_index: byte_index.expect("byte_index recorded"),
        },
        "MissingSeparator" => ParseError::MissingSeparator {
            byte_index: byte_index.expect("byte_index recorded"),
        },
        "LeadingZero" => ParseError::LeadingZero,
        "NegativeZero" => ParseError::NegativeZero,
        "NegativeUnsigned" => ParseError::NegativeUnsigned,
        "Overflow" => ParseError::Overflow,
        "Zero" => ParseError::Zero,
        other => panic!("unknown error variant {other}"),
    }
}

/// Runs a macro taking a concrete integer type for a vector's type name.
#[macro_export]
macro_rules! with_type {
    ($name:expr, $mac:ident $(, $arg:expr)*) => {
        match $name {
            "i8" => $mac!(i8 $(, $arg)*),
            "i16" => $mac!(i16 $(, $arg)*),
            "i32" => $mac!(i32 $(, $arg)*),
            "i64" => $mac!(i64 $(, $arg)*),
            "i128" => $mac!(i128 $(, $arg)*),
            "u8" => $mac!(u8 $(, $arg)*),
            "u16" => $mac!(u16 $(, $arg)*),
            "u32" => $mac!(u32 $(, $arg)*),
            "u64" => $mac!(u64 $(, $arg)*),
            "u128" => $mac!(u128 $(, $arg)*),
            "NonZeroU8" => $mac!(core::num::NonZeroU8 $(, $arg)*),
            "NonZeroU16" => $mac!(core::num::NonZeroU16 $(, $arg)*),
            "NonZeroU32" => $mac!(core::num::NonZeroU32 $(, $arg)*),
            "NonZeroU64" => $mac!(core::num::NonZeroU64 $(, $arg)*),
            "NonZeroU128" => $mac!(core::num::NonZeroU128 $(, $arg)*),
            "NonZeroUsize" => $mac!(core::num::NonZeroUsize $(, $arg)*),
            other => panic!("unknown type name {other}"),
        }
    };
}

/// Runs a generic body once with the vector's format bound to `$f`.
#[macro_export]
macro_rules! with_format {
    ($locale:expr, $custom:expr, $f:ident => $body:expr) => {
        match common::any_format($locale, $custom) {
            common::AnyFormat::Locale($f) => $body,
            common::AnyFormat::Custom($f) => $body,
        }
    };
}
