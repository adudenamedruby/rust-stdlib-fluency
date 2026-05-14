// # Week 7: Basic Concurrency and Async Orientation
//
// Objective: learn the standard concurrency primitives well enough to read and write simple threaded Rust, then get a beginner-level orientation to Tokio.
//
// ## Day 43 - `std::thread`
//
// - [ ] Retrieval (5 min, before reading): rewrite a regex-based parser from Week 6 from memory — e.g., extracting named captures from a log line.
// - [ ] Focus: spawning and joining threads.
//   - Detail: Threads are Rust's standard-library baseline for parallel execution. This focus is about ownership across thread boundaries and why spawned work usually needs owned or `'static` data.
// - [ ] Read/inspect:
//   - What to look for: Look for `spawn`, `JoinHandle`, and the examples using `move`. Pay attention to why thread closures differ from ordinary local closures.
//   - `std::thread`
// - [ ] Exercise:
//   - Goal: The point is to make ownership transfer into threads concrete. The failed borrowed-data attempt is part of the lesson, because it shows what Rust is preventing.
//   - Spawn several threads that compute partial sums.
//   - Join them and combine results.
//   - Try capturing borrowed data, then fix with `move` and owned data.
// - [ ] Done when:
//   - You understand why spawned threads usually need `'static` data or owned values.

fn main() {}
