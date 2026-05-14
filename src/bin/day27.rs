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
//   - Decisions to make: pick a stable output format (e.g., `path:line:text`) and stick to it. Lines are conventionally 1-indexed in grep-style tools.
//   - No `regex` yet.
//   - Continuity: Day 37 replaces your manual arg parsing here with `clap`, and Day 38 replaces substring search with `regex`. Keep the search function pure (signature like `fn search(pattern, text) -> impl Iterator<Item = Match>`) so the matcher can be swapped without touching IO.
// - [ ] Done when:
//   - Handles missing args, missing files, invalid UTF-8-ish situations as gracefully as you can at this stage.
//   - Has tests for the search logic independent of file IO.

fn main() {}
