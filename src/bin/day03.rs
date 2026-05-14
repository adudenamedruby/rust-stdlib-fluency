// ## Day 3 - `String`, `str`, slices, and UTF-8 reality
//
// roux
//
// - [ ] Focus: owned versus borrowed text.
//   - Detail: Text in Rust forces you to separate ownership from borrowing and bytes from human-readable characters. This focus prevents a lot of early pain around indexing, slicing, and unnecessary allocation.
// - [ ] Read/inspect:
//   - What to look for: Look for the difference between `String` methods and `str` methods, and notice how many methods return iterators. Pay special attention to docs that mention UTF-8 boundaries.
//   - `std::string::String`
//   - primitive `str`
// - [ ] Exercise:
//   - Goal: The point is to write text APIs that borrow by default and allocate only when producing new text. The UTF-8 slicing experiment should make Rust's string design feel protective rather than arbitrary.
//   - Write functions that accept `&str`, not `String`, wherever possible.
//   - Implement:
//     - `normalize_whitespace(s: &str) -> String`
//     - `count_chars_words_bytes(s: &str) -> (usize, usize, usize)`
//     - `first_n_chars(s: &str, n: usize) -> String`
//   - Try slicing a string at a non-character boundary and observe what happens.
//   - Decisions to make: define what "normalize" means for whitespace — collapse internal runs? trim ends? both? — and stay consistent in your tests. For `count_chars_words_bytes`, name how you're counting "words" (whitespace split is fine — just commit to it). For `first_n_chars`, decide what happens when `n` exceeds the string's length.
// - [ ] Done when:
//   - You can explain the difference between bytes, chars, and grapheme clusters at a high level.
//   - You stop assuming string indexing works like JavaScript/Python.

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
