// ## Day 5 - Ownership through iterator methods
//
// - [ ] Focus: `iter`, `iter_mut`, `into_iter`.
//   - Detail: Iterator methods are where ownership rules become very visible. This day is about deliberately choosing whether you want to read, mutate, or consume a collection.
// - [ ] Read/inspect:
//   - What to look for: Focus on the `Item` type each iterator produces. When reading examples, ask whether each step is borrowing, mutating, or moving values.
//   - `std::iter::Iterator`
// - [ ] Exercise:
//   - Goal: The point is to make movement through iterator methods predictable. By writing three versions of similar logic, you will see how API choice changes ownership of the original vector and its elements.
//   - Given a `Vec<String>`, write three functions:
//     - one that reads names without moving them,
//     - one that mutates names in place,
//     - one that consumes the vector and returns transformed names.
//   - Use `iter`, `iter_mut`, and `into_iter` deliberately.
//   - The transformation itself does not matter — uppercase, append a suffix, reverse, anything. The point is which iterator method you reach for and what the function ends up taking and returning.
// - [ ] Done when:
//   - You can predict whether a loop/adaptor gives you `T`, `&T`, or `&mut T`.

fn main() {}
