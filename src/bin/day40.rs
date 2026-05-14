// ## Day 40 - `itertools`: know when it helps
//
// - [ ] Focus: extending iterator ergonomics.
//   - Detail: `itertools` extends the standard iterator toolbox with convenience methods. The focus is judgement: adding a crate should make the code clearer or more direct, not just fancier.
// - [ ] Read/inspect:
//   - What to look for: Look for methods that correspond to code you have already written manually. Keep notes on which methods feel like clear wins and which feel too magical.
//   - `itertools::Itertools`
// - [ ] Exercise:
//   - Goal: The point is to test whether extra iterator power improves the code. Keeping both versions prevents cargo-culting a crate when `std` is already clear enough.
//   - Take a previous iterator-heavy exercise and try:
//     - `sorted`,
//     - `dedup`,
//     - `group_by`/grouping-style methods available in your version,
//     - `join`,
//     - `counts`.
//   - Keep the std-only version beside it.
// - [ ] Done when:
//   - You can decide whether `itertools` improves or obscures the code.

fn main() {}
