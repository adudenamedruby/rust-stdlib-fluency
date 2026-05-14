// ## Day 27 - Weekend mini-project: `grep_lite`
//
// - [ ] Focus: paths, files, buffered IO, errors.
//   - Detail: This project ties together path handling, buffered IO, search logic, and user-facing errors. It is intentionally pre-regex so you understand the simple standard-library version first.
// - [ ] Project:
//   - Goal: The point is to build a complete file-processing tool with only standard-library tools. Separating search logic from IO gives you testable code and a cleaner path to later adding `regex` or `clap`.
//   - Build `grep_lite` using only `std`.
//   - Inputs: search term and file path.
//   - Output matching lines with line numbers.
//   - Add optional case-insensitive mode.
//   - No `regex` yet.
// - [ ] Done when:
//   - Handles missing args, missing files, invalid UTF-8-ish situations as gracefully as you can at this stage.
//   - Has tests for the search logic independent of file IO.

fn main() {}
