// ## Day 19 - Custom iterators
//
// - [ ] Focus: implementing `Iterator`.
//   - Detail: Custom iterators reveal that iterator chains are built on a simple protocol. Implementing `next` yourself makes the rest of `Iterator` feel less magical.
// - [ ] Read/inspect:
//   - What to look for: Focus on the required method and the meaning of returning `Some` versus `None`. Everything else on the trait builds from that repeated call pattern.
//   - `Iterator::next`
// - [ ] Exercise:
//   - Goal: The point is to demystify iterators by building your own. Once you implement `next`, adapters and consumers should feel like reusable layers on a simple interface.
//   - Implement a `LinesWithNumbers` iterator over `&str` that yields `(usize, &str)`.
//   - Implement a simple `TakeUntil` iterator adapter.
// - [ ] Done when:
//   - You understand that most iterator magic comes from one method: `next`.

fn main() {}
