//! Gate 3: round trips. `parse(format(v)) == Ok(v)` for random values under
//! every format in the vector set, and `format(parse(s)) == s` for every
//! accepted string.

mod common;

use std::sync::OnceLock;

use common::AnyFormat;
use digit_grouping::{Buffer, FromFormattedStr, Locale, ToFormattedStr};
use proptest::prelude::*;

/// Every locale variant plus every distinct custom format in the vectors.
fn all_formats() -> &'static Vec<AnyFormat> {
    static FORMATS: OnceLock<Vec<AnyFormat>> = OnceLock::new();
    FORMATS.get_or_init(|| {
        let vectors = common::load();
        let mut formats: Vec<AnyFormat> = Locale::available_names()
            .iter()
            .map(|name| AnyFormat::Locale(Locale::from_name(name).unwrap()))
            .collect();
        let mut seen = Vec::new();
        for vector in &vectors.format {
            if let Some(spec) = &vector.custom {
                let built = spec.build();
                if !seen.contains(&built) {
                    seen.push(built.clone());
                    formats.push(AnyFormat::Custom(built));
                }
            }
        }
        assert_eq!(formats.len(), Locale::available_names().len() + 15);
        formats
    })
}

fn roundtrip<N>(value: N) -> Result<(), TestCaseError>
where
    N: ToFormattedStr + FromFormattedStr + PartialEq + std::fmt::Debug,
{
    let mut buffer = Buffer::new();
    for format in all_formats() {
        crate::with_any!(format, f => {
            let text = buffer.write_formatted(value, f);
            let back = digit_grouping::parse_formatted::<N, _>(text, f);
            prop_assert_eq!(back, Ok(value), "{}", text);
        });
    }
    Ok(())
}

/// Binds a format reference out of an [`AnyFormat`].
#[macro_export]
macro_rules! with_any {
    ($any:expr, $f:ident => $body:expr) => {
        match $any {
            common::AnyFormat::Locale($f) => $body,
            common::AnyFormat::Custom($f) => $body,
        }
    };
}

macro_rules! roundtrip_test {
    ($($name:ident: $ty:ty),* $(,)?) => {
        proptest! {
            #![proptest_config(ProptestConfig::with_cases(2000))]
            $(
                #[test]
                fn $name(value: $ty) {
                    roundtrip::<$ty>(value)?;
                }
            )*
        }
    };
}

roundtrip_test!(
    roundtrip_i8: i8,
    roundtrip_i16: i16,
    roundtrip_i32: i32,
    roundtrip_i64: i64,
    roundtrip_i128: i128,
    roundtrip_isize: isize,
    roundtrip_u8: u8,
    roundtrip_u16: u16,
    roundtrip_u32: u32,
    roundtrip_u64: u64,
    roundtrip_u128: u128,
    roundtrip_usize: usize,
);

/// The accepted strings re-emit byte for byte.
#[test]
fn accepted_strings_reformat_exactly() {
    let vectors = common::load();
    let mut buffer = Buffer::new();
    for vector in &vectors.parse_ok {
        macro_rules! run {
            ($ty:ty) => {
                crate::with_format!(&vector.locale, &vector.custom, f => {
                    let parsed: $ty =
                        digit_grouping::parse_formatted(&vector.input, &f).expect(&vector.id);
                    assert_eq!(
                        buffer.write_formatted(parsed, &f),
                        vector.input,
                        "{}",
                        vector.id
                    );
                })
            };
        }
        crate::with_type!(vector.type_name.as_str(), run);
    }
}

/// NonZero types format and parse like their primitive counterparts.
#[test]
fn nonzero_roundtrip() {
    let mut buffer = Buffer::new();
    let value = core::num::NonZeroU64::new(9_876_543_210).unwrap();
    let text = buffer.write_formatted(value, &Locale::en);
    assert_eq!(text, "9,876,543,210");
    assert_eq!(
        digit_grouping::parse_formatted::<core::num::NonZeroU64, _>(text, &Locale::en),
        Ok(value)
    );
}

/// On this crate's 64-bit CI targets, usize and isize behave as u64 and i64.
#[test]
#[cfg(target_pointer_width = "64")]
fn usize_matches_u64() {
    let mut a = Buffer::new();
    let mut b = Buffer::new();
    for value in [0usize, 999, 1000, usize::MAX] {
        assert_eq!(
            a.write_formatted(value, &Locale::hi),
            b.write_formatted(value as u64, &Locale::hi)
        );
    }
    assert_eq!(
        a.write_formatted(isize::MIN, &Locale::en),
        b.write_formatted(i64::MIN, &Locale::en)
    );
}
