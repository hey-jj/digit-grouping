# Changelog

## [0.1.0] - 2026-08-21

First release.

- Integer formatting with locale digit grouping for 54 CLDR locales plus 9
  default-content aliases, generated from cldr-json 48.2.1 and checked in.
  Standard, Indian and Posix grouping, minimumGroupingDigits, non-ASCII
  digit sets, multi-scalar minus signs.
- `Buffer`: 192-byte stack formatting with zero heap allocations, `Copy`,
  `Deref<Target = str>`.
- `ToFormattedString`, `WriteFormatted` and a `formatted()` Display adapter.
- `CustomFormat` with a validating builder: separator, minus sign, decimal
  symbol, grouping rule, minimum grouping digits.
- Strict parse-back for the 12 primitive integer types and the 6 `NonZeroU*`
  types: the parser accepts exactly what the formatter emits and returns a
  typed `ParseError` for everything else. Oversized magnitudes return
  `Overflow`, and inputs outside the format's digit set return
  `InvalidCharacter` with the byte index.
- `no_std` by default, zero runtime dependencies, no build script,
  `#![forbid(unsafe_code)]`, rust-version 1.81.
- Optional `float` feature: fixed-precision `f32`/`f64` with `core::fmt`
  rounding, grouped integer part, locale decimal symbol, infinity and NaN
  symbols from the locale table.
- The crate exposes CLDR pin constants `CLDR_VERSION`, `CLDR_SOURCE_URL` and
  `CLDR_INPUT_SHA256`. A shipped test asserts that the vector set uses the
  same pinned tag.
