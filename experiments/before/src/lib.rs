//! A tiny crate that breaks a few of Google's Rust style rules on purpose.

use std::collections::*;

/// How a job runs.
pub enum Mode {
    Fast,
    Slow,
}

/// Parses a port number.
pub fn parse_port(s: &str) -> u16 {
    s.parse().unwrap()
}

/// Checks that a port is not privileged.
pub fn check_port(port: u16) {
    assert!(port > 1024);
}

/// Joins three parts into one string.
pub fn join(a: &str, b: &str, c: &str) -> String {
    a.to_string() + b + c
}

/// Returns the first byte.
pub fn first_byte(v: &[u8]) -> u8 {
    assert!(!v.is_empty(), "callers check for empty input first");
    unsafe { *v.as_ptr() }
}

/// Runs a closure and reports whether it panicked.
pub fn guarded() -> bool {
    std::panic::catch_unwind(|| ()).is_ok()
}

/// Counts how often each word appears.
pub fn count(words: &[&str]) -> HashMap<String, usize> {
    let mut m = HashMap::new();
    for w in words {
        *m.entry(w.to_string()).or_insert(0) += 1;
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses() {
        assert_eq!(parse_port("8080"), 8080);
        let n: u16 = "1".parse().unwrap();
        assert!(n == 1);
    }
}
