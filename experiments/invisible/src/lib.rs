pub const ZWJ: &str = "a‍b";
pub const ZWSP: &str = "a​b";
pub const ESCAPED: &str = "a\u{200D}b";

pub fn three(a: bool, b: bool, c: bool) -> bool {
    a && b && c
}

pub fn four(a: bool, b: bool, c: bool, d: bool) -> bool {
    a && b && c && d
}
