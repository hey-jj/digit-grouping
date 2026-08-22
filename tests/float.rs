//! The optional float feature: fixed precision, core::fmt rounding, grouped
//! integer part, locale decimal symbol and digit set.
#![cfg(feature = "float")]

use digit_grouping::{to_formatted_string_float, Buffer, CustomFormat, Locale, WriteFormatted};

#[test]
fn grouped_integer_part_and_decimal_symbol() {
    let mut buffer = Buffer::new();
    assert_eq!(
        buffer.write_formatted_float(1234567.891f64, 2, &Locale::en),
        Ok("1,234,567.89")
    );
    assert_eq!(
        buffer.write_formatted_float(-1234.5f64, 1, &Locale::de),
        Ok("-1.234,5")
    );
    assert_eq!(
        buffer.write_formatted_float(1234567.0f64, 0, &Locale::hi),
        Ok("12,34,567")
    );
    // es suppresses grouping below five integer digits.
    assert_eq!(
        buffer.write_formatted_float(1000.5f64, 1, &Locale::es),
        Ok("1000,5")
    );
}

#[test]
fn rounding_matches_core_fmt() {
    let mut buffer = Buffer::new();
    for (value, precision) in [(0.125f64, 2), (2.675, 2), (1.0 / 3.0, 5), (9.9999, 3)] {
        let raw = format!("{value:.precision$}");
        let ours = buffer
            .write_formatted_float(value, precision, &CustomFormat::default())
            .unwrap()
            .replace(',', "");
        assert_eq!(ours, raw);
    }
}

#[test]
fn locale_digit_set_covers_fraction() {
    let mut buffer = Buffer::new();
    let text = buffer
        .write_formatted_float(-1234.56f64, 2, &Locale::ar_EG)
        .unwrap()
        .to_owned();
    assert_eq!(
        text,
        "\u{61c}-\u{661}\u{66c}\u{662}\u{663}\u{664}\u{66b}\u{665}\u{666}"
    );
}

#[test]
fn specials_use_table_symbols() {
    let mut buffer = Buffer::new();
    assert_eq!(
        buffer.write_formatted_float(f64::INFINITY, 2, &Locale::en),
        Ok("\u{221e}")
    );
    assert_eq!(
        buffer.write_formatted_float(f64::NEG_INFINITY, 2, &Locale::sv),
        Ok("\u{2212}\u{221e}")
    );
    assert_eq!(
        buffer.write_formatted_float(f64::NAN, 2, &Locale::en),
        Ok("NaN")
    );
    assert_eq!(
        buffer.write_formatted_float(f64::NAN, 2, &Locale::fa),
        Ok("\u{646}\u{627}\u{639}\u{62f}\u{62f}")
    );
}

#[test]
fn overflow_returns_error_and_clears() {
    let mut buffer = Buffer::new();
    buffer.write_formatted(5, &Locale::en);
    let result = buffer.write_formatted_float(1.0f64, 500, &Locale::en);
    assert!(result.is_err());
    assert!(buffer.is_empty());
    assert!(buffer
        .write_formatted_float(f64::MAX, 0, &Locale::en)
        .is_err());
}

fn assert_uncapped_paths(value: f64, precision: usize, expected: &str) {
    let allocated = to_formatted_string_float(value, precision, &Locale::en).unwrap();
    assert_eq!(allocated, expected);

    let mut written = String::new();
    written
        .write_formatted_float(value, precision, &Locale::en)
        .unwrap();
    assert_eq!(written, expected);
}

#[test]
fn f64_max_precision_zero_is_uncapped() {
    let expected = concat!(
        "179,769,313,486,231,570,814,527,423,731,704,356,798,070,567,525,844,996,",
        "598,917,476,803,157,260,780,028,538,760,589,558,632,766,878,171,540,458,",
        "953,514,382,464,234,321,326,889,464,182,768,467,546,703,537,516,986,049,",
        "910,576,551,282,076,245,490,090,389,328,944,075,868,508,455,133,942,304,",
        "583,236,903,222,948,165,808,559,332,123,348,274,797,826,204,144,723,168,",
        "738,177,180,919,299,881,250,404,026,184,124,858,368"
    );
    assert_uncapped_paths(f64::MAX, 0, expected);
}

#[test]
fn precision_193_is_uncapped() {
    let expected = format!("1.{}", "0".repeat(193));
    assert_uncapped_paths(1.0, 193, &expected);
}

#[test]
fn one_e_200_is_uncapped() {
    let expected = concat!(
        "99,999,999,999,999,996,973,312,221,251,036,165,947,450,327,545,502,362,",
        "648,241,750,950,346,848,435,554,075,534,196,338,404,706,251,868,027,512,",
        "415,973,882,408,182,135,734,368,278,484,639,385,041,047,239,877,871,023,",
        "591,066,789,981,811,181,813,306,167,128,854,888,448"
    );
    assert_uncapped_paths(1e200, 0, expected);
}

#[test]
fn write_and_string_paths_agree() {
    let mut buffer = Buffer::new();
    let expected = buffer
        .write_formatted_float(98765.5f32, 1, &Locale::fr)
        .unwrap()
        .to_owned();
    let mut sink = String::new();
    sink.write_formatted_float(98765.5f32, 1, &Locale::fr)
        .unwrap();
    assert_eq!(sink, expected);
    assert_eq!(
        to_formatted_string_float(98765.5f32, 1, &Locale::fr).unwrap(),
        expected
    );
}
