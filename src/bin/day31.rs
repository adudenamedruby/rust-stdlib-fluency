// ## Day 31 - `thiserror` for library-like errors
//
// - [ ] Focus: ergonomic typed errors.
//   - Detail: `thiserror` removes repetitive error boilerplate while keeping your error type explicit. This is the pattern you usually want in library-like code where callers may need to inspect error variants.
// - [ ] Read/inspect:
//   - What to look for: Look for how attributes map enum variants to display messages and source errors. The point is to recognize the boilerplate being generated, not to learn every attribute immediately.
//   - `thiserror` docs
// - [ ] Exercise:
//   - Goal: The point is to keep typed errors while removing boilerplate. Matching variants in tests reinforces why typed errors are useful when callers need to respond differently to different failures.
//   - Rewrite Day 30 using `thiserror`.
//   - Add tests that match specific error variants.
// - [ ] Done when:
//   - You can explain why `thiserror` is good for reusable/library-ish code.

fn main() {}
