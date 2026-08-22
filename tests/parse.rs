//! Named parser regressions: invalid-scalar rejection, overflow without
//! panics, and rejection of everything the formatter does not emit.

mod common;

use digit_grouping::{parse_formatted, ParseError};
use proptest::prelude::*;

/// Every reject vector whose input carries U+1D7D8, U+2460, U+00BD or
/// U+0663 under a latn format (E001 through E018) returns
/// `InvalidCharacter` with the recorded byte index, for u8 through u128 and
/// i8 through i128. Those scalars pass `char::is_numeric`, so a parser that
/// filters on that keeps them.
#[test]
fn issue_52_inputs_are_invalid_character() {
    let vectors = common::load();
    let mut seen = 0;
    for vector in &vectors.parse_err {
        let n = vector
            .id
            .strip_prefix('E')
            .and_then(|s| s.parse::<u32>().ok());
        if !matches!(n, Some(1..=18)) {
            continue;
        }
        seen += 1;
        let expected = common::expected_error(&vector.error);
        assert!(matches!(expected, ParseError::InvalidCharacter { .. }));
        macro_rules! run {
            ($ty:ty) => {
                crate::with_format!(&vector.locale, &vector.custom, f => {
                    assert_eq!(
                        parse_formatted::<$ty, _>(&vector.input, &f),
                        Err(expected),
                        "{}",
                        vector.id
                    );
                })
            };
        }
        run!(u8);
        run!(u16);
        run!(u32);
        run!(u64);
        run!(u128);
        run!(i8);
        run!(i16);
        run!(i32);
        run!(i64);
        run!(i128);
    }
    assert_eq!(seen, 18);
}

/// The overflow vectors return `Err(Overflow)`: any input whose magnitude
/// exceeds the type, including inputs one digit longer than the type's
/// maximum, errors instead of panicking.
#[test]
fn overflow_vectors_return_err() {
    let vectors = common::load();
    let mut seen = 0;
    for vector in &vectors.parse_err {
        if vector.error.variant != "Overflow" {
            continue;
        }
        seen += 1;
        macro_rules! run {
            ($ty:ty) => {
                crate::with_format!(&vector.locale, &vector.custom, f => {
                    assert_eq!(
                        parse_formatted::<$ty, _>(&vector.input, &f),
                        Err(ParseError::Overflow),
                        "{}",
                        vector.id
                    );
                })
            };
        }
        crate::with_type!(vector.type_name.as_str(), run);
    }
    assert_eq!(seen, 16);
}

proptest! {
    /// Random digit strings of length 1 through 60 never panic for any
    /// type, and anything longer than 39 digits is an error for every type.
    #[test]
    fn overflow_never_panics(digits in "[0-9]{1,60}") {
        let locale_en = digit_grouping::Locale::en;
        macro_rules! run {
            ($ty:ty) => {{
                let result = parse_formatted::<$ty, _>(&digits, &locale_en);
                if digits.len() > 39 {
                    prop_assert!(result.is_err());
                }
                if result.is_ok() {
                    // Only ungrouped short magnitudes can parse under en.
                    prop_assert!(digits.len() <= 3);
                }
            }};
        }
        run!(i8); run!(i16); run!(i32); run!(i64); run!(i128); run!(isize);
        run!(u8); run!(u16); run!(u32); run!(u64); run!(u128); run!(usize);
        run!(core::num::NonZeroU8);
        run!(core::num::NonZeroU16);
        run!(core::num::NonZeroU32);
        run!(core::num::NonZeroU64);
        run!(core::num::NonZeroU128);
        run!(core::num::NonZeroUsize);
    }
}

/// All 97 reject vectors error, two of them under the adjudication written
/// in tests/conformance.rs: E051 is still rejected, with the variant the
/// grammar gives that shape, and E096 is the one entry whose input is
/// exactly what the formatter emits, contradicted by P1811 inside the same
/// frozen file.
#[test]
fn rejects_everything_the_formatter_does_not_emit() {
    let vectors = common::load();
    for vector in &vectors.parse_err {
        // Both adjudicated vectors declare type u32.
        let adjudicated: Option<Result<u32, ParseError>> = match vector.id.as_str() {
            "E051" => Some(Err(ParseError::InvalidCharacter { byte_index: 1 })),
            "E096" => Some(Ok(100000)),
            _ => None,
        };
        if let Some(expected) = adjudicated {
            crate::with_format!(&vector.locale, &vector.custom, f => {
                let parsed = parse_formatted::<u32, _>(&vector.input, &f);
                assert_eq!(parsed, expected, "{} adjudicated", vector.id);
            });
            continue;
        }
        let expected = common::expected_error(&vector.error);
        macro_rules! run {
            ($ty:ty) => {
                crate::with_format!(&vector.locale, &vector.custom, f => {
                    let parsed = parse_formatted::<$ty, _>(&vector.input, &f);
                    assert_eq!(parsed, Err(expected), "{}", vector.id);
                })
            };
        }
        crate::with_type!(vector.type_name.as_str(), run);
    }
    assert_eq!(vectors.parse_err.len(), 97);
}
