pub fn two(a: &str, b: &str) -> String {
    a.to_string() + b
}

mod helpers {
    use super::*;

    pub fn shout(a: &str) -> String {
        two(a, "!")
    }
}

pub fn loud(a: &str) -> String {
    helpers::shout(a)
}
