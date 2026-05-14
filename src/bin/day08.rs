// # Week 2: Collections and Choosing the Right Container
//
// Objective: learn the everyday collections and the tradeoffs between them. This week should make `std::collections` feel like a familiar toolbox.
//
// ## Day 8 - `HashMap` and the entry API
//
// - [ ] Retrieval (5 min, before reading): rewrite `normalize_whitespace` from Day 3 (or `parse_port` from Day 2 — whichever you remember less well) from memory. Compare to your old version and note any APIs you had to look up.
// - [ ] Focus: maps, counts, updates.
//   - Detail: Maps are central to summarizing, indexing, and counting. The `entry` API is especially important because it gives you a Rust-native way to update a value without doing multiple lookups.
// - [ ] Read/inspect:
//   - What to look for: Look at insertion, lookup, mutation, and especially `entry`. Pay attention to what requires ownership of a key and what can work with borrowed lookup values.
//   - `std::collections::HashMap`
// - [ ] Exercise:
//   - Goal: The point is to compare two valid ways to update a map and feel why `entry` is often cleaner. Sorting the output also reinforces the difference between storage order and presentation order.
//   - Write a word-frequency counter.
//   - Implement it twice:
//     - once with `get_mut`/`insert`,
//     - once with `entry(...).or_insert(...)`.
//   - Sort output by count descending, then word ascending.
//   - Decisions to make: what is a "word" (whitespace split? alphanumeric runs?), and is counting case-sensitive? Commit to a definition before you write the counter.
// - [ ] Done when:
//   - You understand why `entry` is a central Rust collection idiom.

fn main() {}
