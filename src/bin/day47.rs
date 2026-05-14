// ## Day 47 - Tokio orientation
//
// - [ ] Focus: async runtime basics.
//   - Detail: Async Rust is split between language features and runtimes. This orientation shows that Tokio supplies the executor and async IO tools that the standard library intentionally does not provide.
// - [ ] Read/inspect:
//   - What to look for: Look for the runtime entry point, task spawning, and sleeping/timing examples. Keep the question in mind: what is Tokio providing that `std` threads did not?
//   - Tokio tutorial introduction
//   - `tokio::main`, `tokio::spawn`, `tokio::time`
// - [ ] Exercise:
//   - Goal: The point is to get a first working mental model of async tasks. Sleeping tasks are simple on purpose: they isolate spawning, awaiting, and scheduling from networking complexity.
//   - Create a tiny Tokio binary.
//   - Spawn several async tasks that sleep and return values.
//   - Await their `JoinHandle`s.
// - [ ] Done when:
//   - You understand that `async` syntax is in Rust, but a runtime like Tokio does the scheduling and IO work.

fn main() {}
