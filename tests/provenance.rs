//! Verifies that the shipped vector set uses the same pinned CLDR tag as the
//! crate's `CLDR_VERSION` and `CLDR_SOURCE_URL` constants.

mod common;

/// The vector file was generated from the same cldr-json tag as the table.
#[test]
fn vectors_use_same_pin() {
    let vectors = common::load();
    assert_eq!(
        vectors.meta.cldr.cldr_json_tag,
        digit_grouping::CLDR_VERSION
    );
    assert!(digit_grouping::CLDR_SOURCE_URL.contains(digit_grouping::CLDR_VERSION));
}
