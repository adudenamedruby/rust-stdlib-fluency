// ## Day 18 - Closures, sorting, and capture
//
// - [ ] Focus: closures as small behavior units.
//   - Detail: Closures are how you pass small pieces of behavior into library methods like sorting. This day also makes capture rules concrete, which helps with both iterators and threaded code later.
// - [ ] Read/inspect:
//   - What to look for: Look for how closure traits relate to capture behavior: `Fn`, `FnMut`, and `FnOnce`. In sorting docs, notice when stable versus unstable sorting matters.
//   - Rust Book closure chapter
//   - `slice::sort_by`, `sort_by_key`, `sort_unstable_by_key`
// - [ ] Exercise:
//   - Goal: The point is to practice passing logic into standard-library algorithms. Returning a closure with `impl Fn` also introduces how Rust represents behavior in types.
//   - Sort a list of records by multiple fields.
//   - Pick any small record shape (e.g., name + score + date) — the lesson is closure mechanics and sort variants, not the data.
//   - Use closures that borrow external state.
//   - Try to return a closure from a function using `impl Fn`.
// - [ ] Done when:
//   - You can explain closure capture by borrow, mutable borrow, and move at a basic level.

fn main() {}
