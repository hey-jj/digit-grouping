/// How digits are grouped.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Grouping {
    /// Groups of three, counted from the right: `1,234,567`.
    Standard,
    /// A rightmost group of three, then groups of two: `12,34,567`.
    Indian,
    /// No separators at all: `1234567`.
    Posix,
}
