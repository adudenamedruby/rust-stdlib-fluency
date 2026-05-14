// ## Day 30 - Manual custom errors
//
// - [ ] Focus: custom error type without macros.
//   - Detail: Writing a custom error type manually shows the machinery behind ergonomic error crates. You are learning what `Display`, `Error`, and `From` contribute to the `?` workflow.
// - [ ] Read/inspect:
//   - What to look for: Look for how `Display` differs from `Debug`, and how implementing `Error` lets your type participate in normal error handling. Notice how `From` supports the `?` operator.
//   - `std::fmt::Display`
//   - `std::error::Error`
// - [ ] Exercise:
//   - Goal: The point is to build the error plumbing yourself once. After this, derive-based error crates will feel like time-savers rather than black boxes.
//   - Create an enum `ConfigError` with variants for IO, parse, and missing field.
//   - Implement `Display` and `Error` manually.
//   - Implement `From<std::io::Error>` where useful.
// - [ ] Done when:
//   - You understand what `thiserror` automates.

fn main() {}
