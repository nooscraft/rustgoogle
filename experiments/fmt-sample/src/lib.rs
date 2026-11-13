//! Formatting check.

use std::{collections::HashMap, fmt::Write};

/// Mask for the low byte.
pub const MASK: u32 = 0xFF;

/// Squares a number.
#[macro_export]
macro_rules! square {
    ($x : expr) => {
        $x * $x
    };
}

/// Builds a report line per entry.
pub fn report(counts: &HashMap<String, usize>) -> String {
    let mut out = String::new();
    for (k, v) in counts {
        writeln!(out, "{k}: {v}").expect("writing to a String does not fail");
    }
    out
}
