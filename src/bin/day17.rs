// ## Day 17 - `IntoIterator`, `FromIterator`, and `Extend`
//
// - [ ] Focus: collection construction and generic iteration.
//   - Detail: These traits are what make Rust functions feel flexible at the call site. The focus is writing code that accepts many iterable inputs without giving up type safety.
// - [ ] Read/inspect:
//   - What to look for: Look for the relationship between accepting inputs, building collections, and extending existing collections. The key is understanding why many APIs accept `IntoIterator` rather than a specific collection.
//   - `std::iter::IntoIterator`
//   - `std::iter::FromIterator`
//   - `std::iter::Extend`
// - [ ] Exercise:
//   - Goal: The point is to make one function work with several caller-owned shapes. The tests prove whether the generic signature is genuinely ergonomic rather than just abstract.
//   - Write `fn histogram<I, S>(items: I) -> HashMap<String, usize>` where `I: IntoIterator<Item = S>` and `S: AsRef<str>`.
//   - Write tests using `Vec<&str>`, `Vec<String>`, and arrays.
// - [ ] Done when:
//   - You can write one generic function that accepts several collection forms.

fn main() {}
