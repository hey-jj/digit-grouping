//! Named regression for the ignored minimumGroupingDigits defect: replay of
//! all format vectors, which include es, hu, it, pl and pt-PT at 999, 1000,
//! 9999 and 10000.

mod common;

use digit_grouping::Buffer;

#[test]
fn minimum_grouping_digits_honored() {
    let vectors = common::load();
    let mut minimum_grouping_cases = 0;
    for vector in &vectors.format {
        macro_rules! run {
            ($ty:ty) => {{
                let value: $ty = vector.value.parse().expect("vector value fits type");
                crate::with_format!(&vector.locale, &vector.custom, f => {
                    let mut buffer = Buffer::new();
                    assert_eq!(
                        buffer.write_formatted(value, &f),
                        vector.expected,
                        "{}",
                        vector.id
                    );
                })
            }};
        }
        crate::with_type!(vector.type_name.as_str(), run);
        if matches!(
            vector.locale.as_deref(),
            Some("es" | "hu" | "it" | "pl" | "pt-PT")
        ) && matches!(vector.value.as_str(), "999" | "1000" | "9999" | "10000")
        {
            minimum_grouping_cases += 1;
        }
    }
    assert_eq!(vectors.format.len(), 1865);
    assert_eq!(minimum_grouping_cases, 20);
}
