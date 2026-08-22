//! Locale-correct digit grouping for integers, with strict parse-back.
//!
//! Formats any primitive integer with the digit set, group separator, minus
//! sign and grouping rule of a CLDR locale, or of a caller-defined
//! [`CustomFormat`]. Parses such a string back into the same integer type,
//! rejecting everything the formatter would not emit.
//!
//! The locale table is generated from cldr-json 48.2.1 and checked into the
//! crate, so there are no runtime dependencies and no build script. The crate
//! is `no_std` by default and every line is safe Rust, under a crate-root
//! forbid.
//!
//! # Formatting
//!
//! ```
//! use digit_grouping::{Buffer, Locale, ToFormattedString};
//!
//! // Allocation-free, on the stack:
//! let mut buf = Buffer::new();
//! assert_eq!(buf.write_formatted(1234567, &Locale::en), "1,234,567");
//! assert_eq!(buf.write_formatted(1234567890, &Locale::hi), "1,23,45,67,890");
//!
//! // Into a String (feature `alloc`, on by default):
//! assert_eq!((-1234567).to_formatted_string(&Locale::de), "-1.234.567");
//! ```
//!
//! # Parsing
//!
//! Parsing accepts exactly the strings the formatter emits for the given
//! format and integer type. Anything else returns a typed [`ParseError`].
//!
//! ```
//! use digit_grouping::{parse_formatted, Locale, ParseError};
//!
//! assert_eq!(parse_formatted::<u32, _>("1,234,567", &Locale::en), Ok(1234567));
//! assert_eq!(
//!     parse_formatted::<u32, _>("1234567", &Locale::en),
//!     Err(ParseError::MissingSeparator { byte_index: 1 }),
//! );
//! ```
//!
//! # Features
//!
//! - `std` (default): implies `alloc`.
//! - `alloc`: adds [`ToFormattedString`].
//! - `float`: adds fixed-precision `f32` and `f64` formatting.
//!
//! # CLDR pin
//!
//! [`CLDR_VERSION`], [`CLDR_SOURCE_URL`] and [`CLDR_INPUT_SHA256`] record the
//! pinned cldr-json tag, its source, and a hash of the generator inputs.
#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

mod buffer;
#[cfg(feature = "float")]
mod float;
mod format;
mod grouping;
mod locale;
mod parse;
mod write;

pub use buffer::Buffer;
#[cfg(all(feature = "float", feature = "alloc"))]
pub use float::to_formatted_string_float;
#[cfg(feature = "float")]
pub use float::{BufferOverflow, ToFormattedFloat};
pub use format::{CustomFormat, CustomFormatBuilder, Format, FormatError};
pub use grouping::Grouping;
pub use locale::{Locale, UnknownLocale, CLDR_INPUT_SHA256, CLDR_SOURCE_URL, CLDR_VERSION};
pub use parse::{parse_formatted, FromFormattedStr, ParseError};
#[cfg(feature = "alloc")]
pub use write::ToFormattedString;
pub use write::{formatted, Formatted, ToFormattedStr, WriteFormatted};

mod sealed {
    /// Seals [`crate::Format`], [`crate::ToFormattedStr`] and the float
    /// trait of the `float` feature.
    pub trait Sealed {}

    /// Formatting internals of the sealed integer types.
    pub trait Int: Sealed + Copy {
        /// Splits into sign and magnitude. `i128::MIN` splits without
        /// overflow because the magnitude is unsigned.
        fn split(self) -> (bool, u128);
    }
}
