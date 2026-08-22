//! Named regression for the buffer bound: every table row and every
//! custom-format limit case lands within the capacity and holds valid
//! UTF-8, checked through the safe `from_utf8`.

mod common;

use digit_grouping::{Buffer, CustomFormat, Grouping, Locale};

fn assert_fits(buffer: &Buffer) {
    assert!(buffer.len() <= Buffer::CAPACITY);
    assert!(core::str::from_utf8(buffer.as_bytes()).is_ok());
    assert_eq!(buffer.as_str().len(), buffer.len());
}

#[test]
fn every_locale_extreme_fits_and_is_utf8() {
    let mut buffer = Buffer::new();
    for name in Locale::available_names() {
        let locale = Locale::from_name(name).unwrap();
        buffer.write_formatted(i128::MIN, &locale);
        assert_fits(&buffer);
        buffer.write_formatted(u128::MAX, &locale);
        assert_fits(&buffer);
    }

    // The custom-format limit: Indian grouping, eight-byte separator,
    // eight-byte minus sign. 39 digits, 18 separators, one sign: 191 bytes.
    let widest = CustomFormat::builder()
        .grouping(Grouping::Indian)
        .separator("--------")
        .minus_sign("minus:--")
        .build()
        .unwrap();
    let longest = buffer.write_formatted(i128::MIN, &widest).len();
    assert_eq!(longest, Buffer::CAPACITY - 1);
    assert_fits(&buffer);

    for grouping in [Grouping::Standard, Grouping::Indian, Grouping::Posix] {
        for separator in ["", ",", "\u{1f642}", "--------"] {
            let format = CustomFormat::builder()
                .grouping(grouping)
                .separator(separator)
                .minus_sign("minus:--")
                .build()
                .unwrap();
            buffer.write_formatted(i128::MIN, &format);
            assert_fits(&buffer);
            buffer.write_formatted(u128::MAX, &format);
            assert_fits(&buffer);
        }
    }
}

#[test]
fn buffer_views_agree() {
    let mut buffer = Buffer::new();
    assert!(buffer.is_empty());
    assert_eq!(buffer.as_str(), "");
    let text = buffer.write_formatted(-5000, &Locale::en).to_owned();
    assert_eq!(text, "-5,000");
    assert_eq!(&*buffer, "-5,000");
    assert_eq!(buffer.as_ref() as &str, "-5,000");
    assert_eq!(buffer, *"-5,000");
    assert_eq!(buffer, "-5,000");
    assert_eq!(std::format!("{buffer}"), "-5,000");
    assert_eq!(std::format!("{buffer:?}"), "\"-5,000\"");
    let copy = buffer;
    assert_eq!(copy.as_str(), buffer.as_str());
    buffer.clear();
    assert!(buffer.is_empty());
    assert_eq!(Buffer::default().len(), 0);
}
