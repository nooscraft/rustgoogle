//! A tiny crate that follows a few of Google's Rust style rules.

use std::collections::HashMap;

/// How a job runs.
#[expect(
    clippy::exhaustive_enums,
    reason = "two states that could be a bool; the guide allows these to stay exhaustive"
)]
pub enum Mode {
    /// Skip the slow checks.
    Fast,
    /// Run every check.
    Slow,
}

/// Parses a port number.
///
/// # Panics
///
/// Panics if `s` is not a valid `u16`.
pub fn parse_port(s: &str) -> u16 {
    s.parse()
        .expect("ports come from our own config file, which is validated on load")
}

/// Checks that a port is not privileged.
///
/// # Panics
///
/// Panics if `port` is 1024 or lower.
pub fn check_port(port: u16) {
    assert!(
        port > 1024,
        "privileged port {port} given; the caller must pick a port above 1024"
    );
}

/// Joins three parts into one string.
pub fn join(a: &str, b: &str, c: &str) -> String {
    format!("{a}{b}{c}")
}

/// Returns the first byte.
///
/// # Panics
///
/// Panics if `v` is empty.
pub fn first_byte(v: &[u8]) -> u8 {
    assert!(!v.is_empty(), "callers check for empty input first");
    // SAFETY: `v` is not empty (asserted above), so its pointer is valid for one read.
    unsafe { *v.as_ptr() }
}

/// Counts how often each word appears.
pub fn count(words: &[&str]) -> HashMap<String, usize> {
    let mut m = HashMap::new();
    for w in words {
        *m.entry(w.to_string()).or_insert(0) += 1;
    }
    m
}

/// Last-resort guard around a plugin call.
#[expect(
    clippy::disallowed_methods,
    reason = "defense in depth only; the plugin API already returns errors instead of panicking"
)]
pub fn guarded(f: impl FnOnce() + std::panic::UnwindSafe) -> bool {
    std::panic::catch_unwind(f).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses() {
        assert_eq!(parse_port("8080"), 8080);
        let n: u16 = "1".parse().unwrap();
        assert!(n == 1);
        let _ = Mode::Fast;
        let _ = Mode::Slow;
    }
}
