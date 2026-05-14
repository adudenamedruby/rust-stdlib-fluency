// ## Day 48 - Weekend mini-project: parallel file stats
//
// - [ ] Focus: practical concurrency.
//   - Detail: This project turns concurrency concepts into a practical batch-processing utility. The goal is to handle independent work safely, collect results, and keep individual failures from sinking the whole run.
// - [ ] Project:
//   - Goal: The point is to use threads for independent IO-ish work and report partial failure cleanly. The stretch Tokio version is optional because standard threading should be solid first.
//   - Build `parallel_stats`.
//   - Input: list of file paths.
//   - Compute line/word/byte counts per file in parallel with std threads.
//   - Send results back over channels.
//   - Print a combined summary.
// - [ ] Stretch:
//   - Add a Tokio version only if the std-threaded version is working.
// - [ ] Done when:
//   - Errors for individual files do not crash the whole program.
//   - You can explain thread-per-file limitations.

fn main() {}
