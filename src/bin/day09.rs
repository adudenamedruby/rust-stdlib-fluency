// ## Day 9 - `BTreeMap`, `HashSet`, and `BTreeSet`
//
// - [ ] Focus: hashing versus ordering.
//   - Detail: Rust gives you both hash-based and ordered collections because performance and determinism are different needs. This day teaches the habit of choosing a collection based on access pattern and output requirements.
// - [ ] Read/inspect:
//   - What to look for: Compare the guarantees each collection gives: ordering, uniqueness, and lookup behavior. Notice the APIs that overlap and the places where tree collections support range-like ordered behavior.
//   - `std::collections::{BTreeMap, HashSet, BTreeSet}`
// - [ ] Exercise:
//   - Goal: The point is to experience how the same input can produce different summaries depending on collection choice. Sorted output should come naturally from tree collections rather than from extra sorting everywhere.
//   - Given log lines with timestamps and user IDs, collect:
//     - unique users,
//     - users in sorted order,
//     - count by day in sorted order.
//   - Use both hash-based and tree-based collections.
//   - Decisions to make: invent a simple line shape yourself (e.g., `2026-04-01 user=alice`). The lesson is collection choice, not parsing fidelity.
// - [ ] Done when:
//   - You can explain when deterministic ordering is worth choosing `BTreeMap`/`BTreeSet`.

fn main() {}
