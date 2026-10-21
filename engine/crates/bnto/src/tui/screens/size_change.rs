// How an output's size compares to its input — shrank, grew, or stayed put.
//
// The results screen labels both the run total and each file with this, so a
// conversion that makes a file bigger says so instead of "0% smaller".

use std::cmp::Ordering;
use std::fmt;

/// The change from an input size to an output size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeChange {
    /// Output is smaller by this many percent of the input (rounded down).
    Smaller(u64),
    /// Output is larger by this many percent of the input (rounded up, so a
    /// file that grew never reads "0% larger").
    Larger(u64),
    /// Output is exactly the input's size.
    Same,
}

impl SizeChange {
    /// Compare `before` to `after`. `None` when there's no input size to compare.
    pub fn between(before: u64, after: u64) -> Option<Self> {
        if before == 0 {
            return None;
        }
        let change = match after.cmp(&before) {
            Ordering::Less => Self::Smaller((before - after) * 100 / before),
            Ordering::Greater => Self::Larger(((after - before) * 100).div_ceil(before)),
            Ordering::Equal => Self::Same,
        };
        Some(change)
    }

    /// Heading for the run total: "Savings" only when the output shrank.
    pub fn heading(self) -> &'static str {
        match self {
            Self::Smaller(_) => "Savings",
            Self::Larger(_) | Self::Same => "Size",
        }
    }
}

impl fmt::Display for SizeChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Smaller(pct) => write!(f, "{pct}% smaller"),
            Self::Larger(pct) => write!(f, "{pct}% larger"),
            Self::Same => f.write_str("same size"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shrinking_output_is_smaller_rounded_down() {
        // 780 KB -> 290 KB is 62.8% smaller.
        assert_eq!(
            SizeChange::between(780_000, 290_000),
            Some(SizeChange::Smaller(62))
        );
    }

    #[test]
    fn growing_output_is_larger() {
        // gradient.jpg 5 KB -> 10 KB WebP doubles.
        assert_eq!(
            SizeChange::between(5_000, 10_000),
            Some(SizeChange::Larger(100))
        );
    }

    #[test]
    fn slight_growth_is_never_zero_percent_larger() {
        assert_eq!(
            SizeChange::between(1_000, 1_001),
            Some(SizeChange::Larger(1))
        );
    }

    #[test]
    fn equal_sizes_are_the_same() {
        assert_eq!(SizeChange::between(500, 500), Some(SizeChange::Same));
    }

    #[test]
    fn zero_byte_input_has_no_comparison() {
        assert_eq!(SizeChange::between(0, 50), None);
        assert_eq!(SizeChange::between(0, 0), None);
    }

    #[test]
    fn labels_read_as_smaller_larger_or_same() {
        assert_eq!(SizeChange::Smaller(62).to_string(), "62% smaller");
        assert_eq!(SizeChange::Larger(100).to_string(), "100% larger");
        assert_eq!(SizeChange::Same.to_string(), "same size");
    }

    #[test]
    fn heading_says_savings_only_when_smaller() {
        assert_eq!(SizeChange::Smaller(10).heading(), "Savings");
        assert_eq!(SizeChange::Larger(10).heading(), "Size");
        assert_eq!(SizeChange::Same.heading(), "Size");
    }
}
