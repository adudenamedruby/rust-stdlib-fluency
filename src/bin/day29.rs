// # Week 5: Error Handling, Testing, and Library Boundaries
//
// Objective: learn Rust's error stack from `std` first, then add the common application/library crates.
//
// ## Day 29 - `std::error::Error` and `Box<dyn Error>`
//
// - [ ] Focus: standard error trait.
//   - Detail: Rust's standard error trait is the foundation underneath both standard and ecosystem error handling. This day teaches the tradeoff between convenient erased errors and more precise typed errors.
// - [ ] Read/inspect:
//   - What to look for: Look for the minimum contract of an error: display, debug, and optional source chaining. Notice how `io::Error` carries a kind plus more detailed platform information.
//   - `std::error::Error`
//   - `std::io::Error`
// - [ ] Exercise:
//   - Goal: The point is to see how one return type can carry multiple underlying error kinds. This is convenient for application code, but the exercise should also reveal what specificity you lose.
//   - Write a function that reads a file and parses numbers from it.
//   - Return `Result<Vec<i32>, Box<dyn std::error::Error>>`.
//   - Use `?` across both IO and parse errors.
// - [ ] Done when:
//   - You understand why boxed dynamic errors are convenient but less specific.

fn main() {}
