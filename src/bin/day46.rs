// ## Day 46 - One-time init and atomics
//
// - [ ] Focus: basic synchronization vocabulary.
//   - Detail: Some concurrency tools are mostly vocabulary until you need them. This focus gives you basic recognition of one-time initialization and atomic counters without pretending atomic memory ordering is simple.
// - [ ] Read/inspect:
//   - What to look for: Look for simple, common examples rather than advanced memory-ordering theory. The goal is to recognize safe global initialization and basic atomic counters.
//   - `std::sync::OnceLock`
//   - `std::sync::LazyLock`
//   - `std::sync::atomic`
// - [ ] Exercise:
//   - Goal: The point is to build recognition-level fluency with synchronization primitives you will see in real Rust code. Keep the examples simple and write down what you are intentionally not trying to master yet.
//   - Create a lazily initialized config or regex-like static value.
//   - Use an `AtomicUsize` counter across threads.
//   - Keep memory ordering simple: use `Ordering::Relaxed` for a plain counter and note why deeper atomic ordering is a separate topic.
// - [ ] Done when:
//   - You know these tools exist and can read simple examples without panic.

fn main() {}
