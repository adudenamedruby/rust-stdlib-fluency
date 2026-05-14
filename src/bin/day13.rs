// ## Day 13 - Weekend mini-project: log summarizer
//
// - [ ] Focus: collections, parsing, sorting, tests.
//   - Detail: This project turns collection knowledge into a practical summarization tool. The important part is not perfect parsing; it is choosing containers deliberately and keeping bad input from crashing the program.
// - [ ] Project:
//   - Goal: The point is to build a realistic summary tool while keeping parsing intentionally basic. You should practice returning recoverable errors and choosing containers for each summary output.
//   - Build `log_summary` that parses lines shaped like:
//     - `2026-04-01 INFO user=roux action=login`
//     - `2026-04-01 ERROR user=alex action=upload`
//   - Produce:
//     - count by level,
//     - count by user,
//     - count by day,
//     - top 3 actions.
//   - Decisions to make: what counts as a "malformed" line — missing fields, unparseable date, extra fields, unknown level? Decide before you write the parser so your errors stay typed and your tests stay meaningful.
//   - No external crates yet.
//   - Continuity: Day 38 will replace your manual line parsing here with `regex` named captures, and Day 41 (`reporter`) wraps the whole thing with `clap` + JSON output. Keep parsing and summarizing in separate functions so they can be swapped.
// - [ ] Done when:
//   - Uses at least 3 collection types.
//   - Has tests for malformed lines.
//   - Does not panic on bad input.

fn main() {}
