// ## Day 25 - `std::env` and basic CLI input
//
// - [ ] Focus: environment and args without `clap`.
//   - Detail: Before using a CLI framework, it helps to know the standard baseline. Manual parsing will feel clumsy, and that discomfort is useful context for understanding what `clap` buys you later.
// - [ ] Read/inspect:
//   - What to look for: Look for what `env::args` gives you and what it does not: no validation model, no help text, and no structured flag handling. That gap is the lesson.
//   - `std::env`
// - [ ] Exercise:
//   - Goal: The point is to understand the lowest-level CLI input available in `std`. You should come away able to write a tiny parser, and also able to explain why you would usually use `clap` for serious tools.
//   - Write a small binary that accepts:
//     - an input path,
//     - an optional `--uppercase` flag,
//     - an optional environment variable for default output path.
//   - Parse manually using `env::args`.
// - [ ] Done when:
//   - You appreciate why `clap` exists, but know the standard baseline.

fn main() {}
