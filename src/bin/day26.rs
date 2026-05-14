// ## Day 26 - `std::process::Command`
//
// - [ ] Focus: shelling out safely.
//   - Detail: Shelling out is common in developer tools, but it has distinct failure modes. This focus separates failure to launch a command from a command that runs successfully but exits with an error status.
// - [ ] Read/inspect:
//   - What to look for: Look closely at `output`, `status`, and `spawn`, and at how stdout/stderr are represented. Separate process setup, execution, captured output, and exit status in your head.
//   - `std::process::{Command, Output, Stdio}`
// - [ ] Exercise:
//   - Goal: The point is to treat subprocesses as fallible external systems. Capturing output and checking status explicitly prevents a common bug: assuming a command succeeded because it launched.
//   - Run `rustc --version` and capture output.
//   - Run `cargo metadata` or `git status` if available.
//   - Handle non-zero exit status explicitly.
// - [ ] Done when:
//   - You can distinguish command spawn errors from unsuccessful exit status.

fn main() {}
