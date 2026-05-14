// ## Day 32 - `anyhow` for application boundaries
//
// - [ ] Focus: context-rich app errors.
//   - Detail: Applications often need helpful context more than inspectable error types. `anyhow` is useful at the outer edge of a program where you want failures to explain what the program was trying to do.
// - [ ] Read/inspect:
//   - What to look for: Look for `Context` and how it adds information at the call site. Pay attention to the difference between preserving the underlying error and improving the top-level message.
//   - `anyhow` docs
// - [ ] Exercise:
//   - Goal: The point is to add human context at the application boundary without polluting lower-level code. This is where error messages become useful to the person running the program.
//   - Write a binary that calls your library-like code.
//   - Use `anyhow::Result` and `.context(...)` / `.with_context(...)` at the app boundary.
// - [ ] Done when:
//   - You can explain `thiserror` inside libraries, `anyhow` near application edges.

fn main() {}
