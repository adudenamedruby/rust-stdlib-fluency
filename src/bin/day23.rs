// ## Day 23 - `std::io`, buffered reads, and writes
//
// - [ ] Focus: reading and writing streams.
//   - Detail: Streams are data sources or sinks that you process progressively instead of loading everything at once. Rust's IO traits let the same code work over files, buffers, stdin/stdout, network streams, and other readers or writers; buffering keeps that efficient.
// - [ ] Read/inspect:
//   - What to look for: Look for the roles of each trait: `Read` pulls bytes, `Write` pushes bytes, `BufRead` adds line-oriented helpers, and buffered wrappers reduce repeated system calls. Notice that these abstractions are generic over many data sources.
//   - `std::io::{Read, Write, BufRead, BufReader, BufWriter}`
// - [ ] Exercise:
//   - Goal: The point is to learn the tradeoff between convenience and scalability. Reading a whole file is often fine; buffered streaming is better when inputs are large, continuous, or should be processed line by line.
//   - Read a file line by line with `BufReader`.
//   - Write transformed lines to another file with `BufWriter`.
//   - Compare with `fs::read_to_string`.
// - [ ] Done when:
//   - You understand when whole-file reads are fine and when buffered IO is better.

fn main() {}
