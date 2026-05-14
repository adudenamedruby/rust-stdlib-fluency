// ## Day 28 - Review checkpoint
//
// - [ ] Review:
//   - Goal: The point is to improve design after the first working version. Refactoring toward pure functions and path-friendly signatures is exactly how small Rust tools become maintainable.
//   - Refactor Day 27 into pure logic plus IO shell.
//   - Change function signatures to accept `impl AsRef<Path>` where useful.
//   - Run `cargo clippy`.
//   - Update your Personal Rules of Thumb at the top of this plan — especially around paths, IO, and when to use `impl AsRef<Path>` in your signatures.
// - [ ] Reflection prompts:
//   - Where should your code own a `PathBuf`?
//   - Where should it borrow a `&Path`?
//   - What error messages would help a real user?

fn main() {}
