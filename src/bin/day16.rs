// ## Day 16 - Iterating over `Option` and `Result`
//
// - [ ] Focus: fallible pipelines.
//   - Detail: Real data pipelines often combine iteration with fallibility. This day teaches how `Option`, `Result`, and `Iterator` compose so you can express skip, collect, or fail-fast behavior cleanly.
// - [ ] Read/inspect:
//   - What to look for: Look for examples where `collect` changes an iterator of results into a result of a collection. For `transpose`, focus on the shape change between nested `Option` and `Result`.
//   - `Option::transpose`
//   - `Result::transpose`
//   - `Iterator::collect` examples involving `Result`
// - [ ] Exercise:
//   - Goal: The point is to learn three common policies for messy input: ignore bad values, fail fast, or report both successes and failures. This is the basis for many real import/parsing tasks.
//   - Parse a list of strings into numbers.
//   - Implement:
//     - collect all valid numbers and ignore invalid ones,
//     - fail on the first invalid number,
//     - return both valid numbers and rejected strings.
// - [ ] Done when:
//   - You understand `collect::<Result<Vec<_>, _>>()`.

fn main() {}
