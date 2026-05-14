// ## Day 11 - Lookup ergonomics and borrowed keys
//
// - [ ] Focus: avoiding unnecessary allocation.
//   - Detail: Good Rust APIs avoid forcing callers to allocate. This focus is about accepting borrowed or convertible inputs so your functions work with `String`, `&str`, `PathBuf`, `&Path`, and similar shapes naturally.
// - [ ] Read/inspect:
//   - What to look for: Do not read these traits abstractly only. Look for examples where they make function parameters more flexible or allow lookup with a borrowed form of an owned key.
//   - `std::borrow::Borrow`
//   - `std::convert::AsRef`
// - [ ] Exercise:
//   - Goal: The point is to reduce friction at API boundaries. You should feel how accepting `&str`, `&Path`, or `impl AsRef<_>` makes functions easier to call and often avoids clones.
//   - Create a `HashMap<String, usize>` and look up values with `&str`.
//   - Write functions that accept `impl AsRef<Path>` and `impl AsRef<str>`.
//   - Try versions that take `String`, `&String`, and `&str`; note the friction.
// - [ ] Done when:
//   - You can explain why `&String` is usually less useful than `&str`.

fn main() {}
