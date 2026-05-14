// ## Day 52 - Formatting and output
//
// - [ ] Focus: `Debug`, `Display`, and formatting machinery.
//   - Detail: Formatting is both user-interface and debugging infrastructure. This focus teaches when to expose a clean human-readable representation and when to rely on structural debug output.
// - [ ] Read/inspect:
//   - What to look for: Look for formatter usage and examples of implementing `Display`. Pay attention to `write!` because it lets formatting target any formatter or string buffer, not just stdout.
//   - `std::fmt`
//   - formatting syntax docs
// - [ ] Exercise:
//   - Goal: The point is to control how your types appear in reports, logs, errors, and debugging output. This is a small API surface that heavily affects usability.
//   - Implement `Display` for several domain types.
//   - Use `write!` to build a `String` report.
//   - Compare `format!`, `println!`, and `write!`.
// - [ ] Done when:
//   - You know when to implement `Display` versus relying on `Debug`.

fn main() {}
