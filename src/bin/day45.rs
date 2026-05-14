// ## Day 45 - Shared state with `Arc`, `Mutex`, and `RwLock`
//
// - [ ] Focus: safe shared ownership.
//   - Detail: Shared state is sometimes necessary, but Rust makes the synchronization visible. This day teaches both how `Arc<Mutex<T>>` works and why reducing shared state is often cleaner.
// - [ ] Read/inspect:
//   - What to look for: Look for what `Arc` solves versus what `Mutex` or `RwLock` solves. Notice the locking APIs and the possibility of poisoning after panic.
//   - `std::sync::{Arc, Mutex, RwLock}`
// - [ ] Exercise:
//   - Goal: The point is to compare two concurrency designs: shared state versus local work plus merge. You should not leave thinking `Arc<Mutex<_>>` is always wrong, only that it is not always best.
//   - Count words across multiple chunks using shared `Arc<Mutex<HashMap<_, _>>>`.
//   - Then rewrite using per-thread maps plus merge at the end.
//   - Compare which design is simpler.
// - [ ] Done when:
//   - You know that `Arc<Mutex<T>>` is useful, but not always the best first design.

fn main() {}
