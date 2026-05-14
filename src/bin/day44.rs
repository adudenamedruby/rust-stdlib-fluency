// ## Day 44 - Channels with `std::sync::mpsc`
//
// - [ ] Focus: message passing.
//   - Detail: Message passing is often simpler than shared mutable state. Channels let workers communicate results back to one place while keeping ownership movement explicit.
// - [ ] Read/inspect:
//   - What to look for: Look for the sender/receiver split and what happens when either side is dropped. Notice that messages move through the channel, which often simplifies ownership.
//   - `std::sync::mpsc`
// - [ ] Exercise:
//   - Goal: The point is to coordinate concurrent work without sharing mutable structures. Sending successes and errors through the same channel also mirrors real worker pipelines.
//   - Build a worker setup where threads send results back to the main thread.
//   - Send either successful results or errors.
// - [ ] Done when:
//   - You can use channels without shared mutable state.

fn main() {}
