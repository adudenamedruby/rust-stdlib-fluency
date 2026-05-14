// ## Day 34 - Weekend mini-project: fallible config loader
//
// - [ ] Focus: errors, tests, filesystem.
//   - Detail: This project forces error boundaries to become concrete. It gives you practice separating reusable parsing logic from the application shell that reads files and reports failures.
// - [ ] Project:
//   - Goal: The point is to build a small but realistic fallible library. You should be able to tell which errors belong to parsing/config logic and which belong to the binary that reads files.
//   - Build `config_loader`.
//   - Input: simple key-value text file like `host=localhost` and `port=8080`.
//   - Output: a `Config` struct.
//   - Use `thiserror` for typed library errors.
//   - Use `anyhow` in the binary.
//   - Test malformed configs.
// - [ ] Done when:
//   - Error messages are useful.
//   - Tests can identify specific error variants.

fn main() {}
