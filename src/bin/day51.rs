// ## Day 51 - Conversion traits
//
// - [ ] Focus: ergonomic APIs.
//   - Detail: Conversion traits make APIs easier to call without weakening types. This day is about designing inputs so callers can pass natural values while your internals still use precise domain types.
// - [ ] Read/inspect:
//   - What to look for: Look for which conversions should be infallible versus fallible. Notice that `From` implies `Into`, while `TryFrom` and `FromStr` communicate validation.
//   - `std::convert::{From, Into, TryFrom, TryInto, AsRef}`
// - [ ] Exercise:
//   - Goal: The point is to make invalid states harder to represent while keeping caller ergonomics good. Domain types plus conversion traits are a common Rust API-design pattern.
//   - Create domain types like `Port`, `UserId`, or `LogLevel`.
//   - Implement `FromStr` or `TryFrom<&str>` where useful.
//   - Use `impl AsRef<Path>` in IO-facing APIs.
// - [ ] Done when:
//   - You understand why callers like APIs that accept common input shapes.

fn main() {}
