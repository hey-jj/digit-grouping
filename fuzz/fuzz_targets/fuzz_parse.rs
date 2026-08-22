//! Gate 4: parsing arbitrary text under an arbitrary locale never panics,
//! for every integer type.
#![no_main]

use libfuzzer_sys::fuzz_target;

use digit_grouping::{parse_formatted, Locale};

fuzz_target!(|input: (u8, &str)| {
    let (locale_index, text) = input;
    let names = Locale::available_names();
    let name = names[usize::from(locale_index) % names.len()];
    let locale = Locale::from_name(name).unwrap();

    macro_rules! run {
        ($($ty:ty),*) => {$(
            let _ = parse_formatted::<$ty, _>(text, &locale);
        )*};
    }
    run!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
    run!(
        core::num::NonZeroU8,
        core::num::NonZeroU16,
        core::num::NonZeroU32,
        core::num::NonZeroU64,
        core::num::NonZeroU128,
        core::num::NonZeroUsize
    );
});
