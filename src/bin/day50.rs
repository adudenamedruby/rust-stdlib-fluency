// # Week 8: API Design, Traits, Formatting, Docs, and Reuse
//
// Objective: start writing Rust that feels like it belongs in the ecosystem: clear types, useful traits, good errors, tests, and docs.
//
// ## Day 50 - Smart pointers and ownership tools
//
// - [ ] Focus: `Box`, `Rc`, `Arc`, `Cow`.
//   - Detail: Rust has several ownership helpers because different sharing problems have different constraints. This focus is about recognizing heap allocation, single-threaded sharing, multi-threaded sharing, and copy-on-write.
// - [ ] Read/inspect:
//   - What to look for: Look for the ownership problem each type solves. Compare single ownership on the heap, reference-counted sharing, thread-safe sharing, and borrowed-or-owned data.
//   - `std::boxed::Box`
//   - `std::rc::Rc`
//   - `std::sync::Arc`
//   - `std::borrow::Cow`
// - [ ] Exercise:
//   - Goal: The point is to connect pointer types to specific ownership situations. The examples should make you cautious about reaching for shared ownership when borrowing or moving would be simpler.
//   - Use `Box` for a recursive enum.
//   - Use `Rc` in a single-threaded shared ownership example.
//   - Use `Cow<'_, str>` in a function that sometimes borrows and sometimes allocates.
// - [ ] Done when:
//   - You can explain why Rust has several ownership helpers instead of one universal reference type.

fn main() {}
