pub fn kind(e: &std::io::Error) -> &'static str {
    match e.kind() {
        std::io::ErrorKind::NotFound => "missing",
        _ => "other",
    }
}
