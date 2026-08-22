//! Gate 4: building a custom format from arbitrary inputs never panics, and
//! every format that builds round-trips 100 values.
#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

use digit_grouping::{parse_formatted, Buffer, CustomFormat, Grouping};

#[derive(Arbitrary, Debug)]
struct Input<'a> {
    grouping: u8,
    separator: &'a str,
    minus_sign: &'a str,
    decimal: &'a str,
    minimum_grouping_digits: u8,
    seed: u64,
}

fuzz_target!(|input: Input<'_>| {
    let grouping = match input.grouping % 3 {
        0 => Grouping::Standard,
        1 => Grouping::Indian,
        _ => Grouping::Posix,
    };
    let built = CustomFormat::builder()
        .grouping(grouping)
        .separator(input.separator)
        .minus_sign(input.minus_sign)
        .decimal(input.decimal)
        .minimum_grouping_digits(input.minimum_grouping_digits)
        .build();
    let Ok(format) = built else { return };

    let mut buffer = Buffer::new();
    let mut state = input.seed;
    for _ in 0..100 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let value = state as i64;
        let text = buffer.write_formatted(value, &format);
        assert_eq!(parse_formatted::<i64, _>(text, &format), Ok(value));
    }
});
