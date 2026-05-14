// ## Day 24 - `Path`, `PathBuf`, `OsStr`, and platform reality
//
// - [ ] Focus: paths are not strings.
//   - Detail: Paths are operating-system values, not just UTF-8 strings. This focus protects your code from subtle portability problems and leads to better function signatures.
// - [ ] Read/inspect:
//   - What to look for: Look for methods that manipulate path components without converting to strings. Pay attention to `OsStr` because it explains why not every path is valid UTF-8.
//   - `std::path::{Path, PathBuf}`
//   - `std::ffi::{OsStr, OsString}`
// - [ ] Exercise:
//   - Goal: The point is to make path manipulation type-correct. Avoiding string conversion will make your code more portable and will prepare you for real CLI/file tools.
//   - Write `fn change_extension(path: &Path, ext: &str) -> PathBuf`.
//   - Walk through a directory and collect files by extension.
//   - Avoid converting paths to strings unless printing.
//   - Decisions to make: for `change_extension`, what happens when the path has no extension yet, when the new extension is empty, or when it already starts with `.`?
// - [ ] Done when:
//   - You stop representing paths as `String` in function signatures.

fn main() {}
