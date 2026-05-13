fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<&str>>().join(" ")
}

fn count_chars_words_bytes(s: &str) -> (usize, usize, usize) {
    (s.chars().count(), s.split_whitespace().count(), s.len())
}

fn first_n_chars(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitespace_test_normal() {
        assert_eq!(
            "hello world".to_string(),
            normalize_whitespace("hello world")
        );
    }

    #[test]
    fn whitespace_test_external() {
        assert_eq!(
            "hello world".to_string(),
            normalize_whitespace("  hello world  ")
        );
    }

    #[test]
    fn whitespace_test_internal_whitespace() {
        assert_eq!(
            "hello world".to_string(),
            normalize_whitespace("hello    world"),
            "failed on normalizing inter word whitespace"
        );
    }

    #[test]
    fn whitespace_test_internal_tabs() {
        assert_eq!(
            "hello world".to_string(),
            normalize_whitespace("hello\tworld")
        );
    }

    #[test]
    fn chars_normal() {
        assert_eq!((5, 1, 5), count_chars_words_bytes("hello"))
    }

    #[test]
    fn chars_unicode() {
        assert_eq!((5, 1, 6), count_chars_words_bytes("hellö"))
    }
}
