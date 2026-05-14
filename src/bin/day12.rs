// ## Day 12 - Ordering, hashing, equality, and derived traits
//
// - [ ] Focus: making your own types collection-friendly.
//   - Detail: Collections rely on traits like equality, ordering, and hashing. By making your own type collection-friendly, you learn what the derives actually mean and why the compiler asks for them.
// - [ ] Read/inspect:
//   - What to look for: Look at what each trait promises: equality, total ordering, hashing, comparison. Connect each trait to the collection that requires it.
//   - `std::cmp`
//   - `std::hash`
// - [ ] Exercise:
//   - Goal: The point is to let compiler errors teach trait requirements. Removing derives one by one makes collection constraints concrete instead of theoretical.
//   - Define a `UserId(String)` newtype.
//   - Put it in `HashMap`, `HashSet`, and `BTreeSet`.
//   - Use derives where possible: `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`, `PartialOrd`, `Ord`.
//   - Then remove one derive at a time and observe compiler errors.
// - [ ] Done when:
//   - You understand which traits a type needs for hash maps versus tree maps.

fn main() {}
