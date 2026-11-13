pub fn parse_port(s: &str) -> u16 {
    s.parse().expect("failed")
}

pub fn check_port(port: u16) {
    assert!(port > 1024, "port > 1024");
}

pub fn first_byte(v: &[u8]) -> u8 {
    // SAFETY: trust me
    unsafe { *v.as_ptr() }
}
