// # Week 6: Practical Ecosystem Layer - Serde, Clap, Regex, Itertools
//
// Objective: learn a small set of crates that Rust developers commonly reach for, while keeping a clear distinction between `std` and ecosystem tools.
//
// ## Day 36 - `serde` basics
//
// - [ ] Retrieval (5 min, before reading): rewrite the manual `ConfigError` enum from Day 30 from memory — variants, `Display` impl, `Error` impl. The check is whether the standard error machinery has stuck before you let `thiserror` hide it.
// - [ ] Focus: serialization and deserialization.
//   - Detail: Serde is the standard ecosystem answer for turning Rust data into external formats and back. The focus is on understanding derive-based serialization before worrying about advanced attributes.
// - [ ] Read/inspect:
//   - What to look for: Look for derive usage first, then field naming and format examples. The main thing to learn is how Rust structs map to external data without hand-written parsing code.
//   - serde overview
//   - `serde::{Serialize, Deserialize}`
// - [ ] Exercise:
//   - Goal: The point is to make Rust structs cross a data-format boundary with minimal manual code. Round-tripping JSON helps you see both serialization and deserialization as part of one model.
//   - Add `serde` and `serde_json`.
//   - Define structs for the Week 2 log summary output.
//   - Serialize summary data to pretty JSON.
//   - Deserialize sample JSON back into structs.
// - [ ] Done when:
//   - You understand derive-based serialization at a basic level.

fn main() {}
