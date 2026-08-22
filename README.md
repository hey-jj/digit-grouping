# digit-grouping

Locale-correct digit grouping for integers, with strict parse-back.

`digit-grouping` formats any primitive integer with the digit set, group
separator, minus sign and grouping rule of a CLDR locale, or of a format you
define. It parses such strings back into the same integer type and rejects
everything its formatter would not emit, with a typed error naming the byte
that failed.

```rust
use digit_grouping::{Buffer, Locale, ToFormattedString, parse_formatted};

// Allocation-free, on the stack.
let mut buf = Buffer::new();
assert_eq!(buf.write_formatted(1234567, &Locale::en), "1,234,567");
assert_eq!(buf.write_formatted(1234567890, &Locale::hi), "1,23,45,67,890");
assert_eq!(buf.write_formatted(-1234567, &Locale::de), "-1.234.567");

// Into a String.
assert_eq!(1000.to_formatted_string(&Locale::es), "1000"); // es groups from 5 digits
assert_eq!(10000.to_formatted_string(&Locale::es), "10.000");

// Strict parse-back.
assert_eq!(parse_formatted::<u32, _>("1,234,567", &Locale::en), Ok(1234567));
assert!(parse_formatted::<u32, _>("1234567", &Locale::en).is_err());
assert!(parse_formatted::<u8, _>("1,234", &Locale::en).is_err()); // overflow, no panic
```

## What you get

- 54 CLDR locales plus 9 default-content aliases (`en_US`, `pt_BR`, ...),
  generated from cldr-json 48.2.1 and checked into the crate. The pin is
  recorded in `CLDR_VERSION`, `CLDR_SOURCE_URL` and `CLDR_INPUT_SHA256`, and
  a CI job regenerates the table and fails on any diff.
- Standard, Indian and Posix grouping, minimumGroupingDigits honored, digit
  sets beyond ASCII (`ar_EG`, `fa`, `bn`, `mr`), multi-scalar minus signs.
- `Buffer`: a 192-byte stack buffer, `Copy`, zero heap allocations. A test
  with a counting allocator holds that at zero across every locale.
- `CustomFormat`: your own separator, minus sign, decimal symbol, grouping
  and minimum grouping digits, validated at construction.
- Parsing with `ParseError` variants for every rejection: misplaced or
  missing separators, wrong digit set, leading zeros, negative zero,
  unsigned minus, overflow, zero into `NonZeroU*`.
- All 12 primitive integer types plus the 6 `NonZeroU*` types. `i128::MIN`
  and `u128::MAX` format and parse correctly.
- An optional `float` feature with fixed-precision `f32`/`f64` formatting
  that rounds exactly as `format!("{:.2}", x)` does.

## Why parsing is strict

For each format and type, the parser accepts exactly the strings the
formatter emits. Under `en` that means `"1,000"` parses, while `"1 000"`,
`"1,0,0,0"` and bare `"1000"` each return an error naming the failing byte.
Round trips hold in both directions: `parse(format(v)) == Ok(v)`, and
`format(parse(s)) == s` for every accepted `s`. A proptest suite and two
fuzz targets back this.

## no_std

The crate is `no_std` with no runtime dependencies, no build script and
`#![forbid(unsafe_code)]` at the root. `Buffer`, `WriteFormatted` and
parsing work with `core` alone. The `alloc` feature (default, through `std`)
adds `ToFormattedString`.

## Performance

The formatter writes digits from the right in one pass, two at a time from a
lookup table, straight into the output buffer. The benchmark gate in
`benches/throughput.rs` requires `Buffer` formatting of `u64` under `en` to
beat a `to_string`-plus-`insert_str` baseline by roughly 5x.

## Feature flags

| feature | default | adds |
|---|---|---|
| `std` | yes | implies `alloc` |
| `alloc` | via `std` | `ToFormattedString` |
| `float` | no | fixed-precision `f32`/`f64` formatting |

## Minimum supported Rust version

1.81, so the error types implement `core::error::Error` without `std`. CI
builds the library and runs the doctests at 1.81.

## License

MIT
