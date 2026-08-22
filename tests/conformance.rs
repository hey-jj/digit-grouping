//! Gate 2: exact replay of every conformance vector.
//!
//! `tests/data/conformance-vectors.json` is the frozen contract, checked by
//! sha256. Every format, parse-accept, parse-reject and custom-format
//! construction entry is asserted. Two parse-reject entries contradict the
//! rest of the file and are asserted against the file's own grammar instead;
//! see `adjudicated_expectation`.

mod common;

use digit_grouping::{Buffer, FormatError, ParseError};

#[test]
fn vector_file_is_frozen() {
    common::load();
}

#[test]
fn counts_match_meta() {
    let vectors = common::load();
    assert_eq!(vectors.format.len(), vectors.meta.counts.format);
    assert_eq!(vectors.parse_ok.len(), vectors.meta.counts.parse_ok);
    assert_eq!(vectors.parse_err.len(), vectors.meta.counts.parse_err);
    assert_eq!(
        vectors.custom_format_construction.len(),
        vectors.meta.counts.custom_format_construction
    );
}

/// Every format vector, through every output path. The paths must agree
/// with each other and with the expected string.
#[test]
fn format_vectors() {
    let vectors = common::load();
    for vector in &vectors.format {
        macro_rules! run {
            ($ty:ty) => {{
                let value: $ty = vector.value.parse().expect("vector value fits type");
                crate::with_format!(&vector.locale, &vector.custom, f => {
                    let mut buffer = Buffer::new();
                    let from_buffer = buffer.write_formatted(value, &f);
                    assert_eq!(from_buffer, vector.expected, "{} buffer", vector.id);
                    #[cfg(feature = "alloc")]
                    {
                        use digit_grouping::{ToFormattedString, WriteFormatted};
                        let from_string = value.to_formatted_string(&f);
                        assert_eq!(from_string, vector.expected, "{} string", vector.id);
                        let mut sink = String::new();
                        sink.write_formatted(value, &f).unwrap();
                        assert_eq!(sink, vector.expected, "{} write", vector.id);
                    }
                })
            }};
        }
        crate::with_type!(vector.type_name.as_str(), run);
    }
}

/// Every accepted string parses to its value.
#[test]
fn parse_ok_vectors() {
    let vectors = common::load();
    for vector in &vectors.parse_ok {
        macro_rules! run {
            ($ty:ty) => {{
                let expected: $ty = vector.expected.parse().expect("expected fits type");
                crate::with_format!(&vector.locale, &vector.custom, f => {
                    let parsed = digit_grouping::parse_formatted::<$ty, _>(&vector.input, &f);
                    assert_eq!(parsed, Ok(expected), "{}", vector.id);
                })
            }};
        }
        crate::with_type!(vector.type_name.as_str(), run);
    }
}

/// Two vectors contradict the rest of the frozen file, so the grammar wins.
///
/// E051 expects `MisplacedSeparator` for `,` under `es`, whose separator is
/// `.` and whose formatter never emits `,`. E054 and E081 pin the same shape
/// (a scalar that is not a digit, separator or minus sign) to
/// `InvalidCharacter` under `de` and `en`, whose symbol tables match `es`.
///
/// E096's input is byte for byte the string format vector F1811 emits and
/// parse vector P1811 accepts for 100000 under the same separator and
/// grouping, so it parses.
fn adjudicated_expectation(id: &str) -> Option<Result<u32, ParseError>> {
    match id {
        "E051" => Some(Err(ParseError::InvalidCharacter { byte_index: 1 })),
        "E096" => Some(Ok(100000)),
        _ => None,
    }
}

/// Every rejected string returns its recorded error.
#[test]
fn parse_err_vectors() {
    let vectors = common::load();
    for vector in &vectors.parse_err {
        if let Some(expected) = adjudicated_expectation(&vector.id) {
            crate::with_format!(&vector.locale, &vector.custom, f => {
                let parsed = digit_grouping::parse_formatted::<u32, _>(&vector.input, &f);
                assert_eq!(parsed, expected, "{} adjudicated", vector.id);
            });
            continue;
        }
        let expected = common::expected_error(&vector.error);
        macro_rules! run {
            ($ty:ty) => {{
                crate::with_format!(&vector.locale, &vector.custom, f => {
                    let parsed = digit_grouping::parse_formatted::<$ty, _>(&vector.input, &f);
                    assert_eq!(parsed, Err(expected), "{}", vector.id);
                })
            }};
        }
        crate::with_type!(vector.type_name.as_str(), run);
    }
}

/// Every custom-format construction case builds or fails as recorded.
#[test]
fn custom_format_construction() {
    let vectors = common::load();
    for case in &vectors.custom_format_construction {
        let spec = &case.custom;
        let result = digit_grouping::CustomFormat::builder()
            .grouping(spec.grouping())
            .separator(&spec.separator)
            .minus_sign(&spec.minus_sign)
            .decimal(&spec.decimal)
            .minimum_grouping_digits(spec.minimum_grouping_digits)
            .build();
        match case.error.as_deref() {
            None => assert!(result.is_ok(), "{} builds", case.id),
            Some(name) => {
                let expected = match name {
                    "SeparatorTooLong" => FormatError::SeparatorTooLong,
                    "MinusSignEmpty" => FormatError::MinusSignEmpty,
                    "MinusSignTooLong" => FormatError::MinusSignTooLong,
                    "DecimalEmpty" => FormatError::DecimalEmpty,
                    "DecimalTooLong" => FormatError::DecimalTooLong,
                    "ContainsDigit" => FormatError::ContainsDigit,
                    "SeparatorEqualsDecimal" => FormatError::SeparatorEqualsDecimal,
                    "MinimumGroupingZero" => FormatError::MinimumGroupingZero,
                    other => panic!("unknown error {other}"),
                };
                assert_eq!(result, Err(expected), "{}", case.id);
            }
        }
    }
}
