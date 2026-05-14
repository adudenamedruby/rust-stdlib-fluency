// ## Day 38 - `regex` basics
//
// - [ ] Focus: regular expressions in Rust.
//   - Detail: Rust keeps regex outside `std`, but the `regex` crate is a common tool for real parsing and search tasks. This focus is about compiling patterns, using captures, and handling invalid patterns safely.
// - [ ] Read/inspect:
//   - What to look for: Look for `Regex::new`, matching APIs, captures, and error handling for invalid patterns. Also notice the crate's guarantees and limitations compared with backtracking regex engines.
//   - `regex` crate docs
// - [ ] Exercise:
//   - Goal: The point is to add pattern matching without making parsing reckless. Compiling once, reusing the regex, and handling invalid patterns are the habits that matter.
//   - Upgrade `grep_lite` to support regex search.
//   - Extract named captures from log lines.
//   - Handle invalid regex patterns gracefully.
//   - Decisions to make: name your captures after the log fields you defined in Week 2 (date, level, user, action) so the parsing pipeline composes cleanly later.
// - [ ] Done when:
//   - You can compile a `Regex` once and reuse it.

fn main() {}
