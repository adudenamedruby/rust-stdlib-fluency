fn normalize_whitespace(s: &str) -> String {
    s.trim().to_string()
}

fn count_chars_words_bytes(s: &str) -> (usize, usize, usize) {
    (
        s.chars().count(),
        s.split_whitespace().count(),
        s.as_bytes().iter().count(),
    )
}

fn main() {}
