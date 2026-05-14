// # Week 4: Files, Paths, IO, Environment, and Processes
//
// Objective: get comfortable writing practical Rust utilities that touch the operating system.
//
// ## Day 22 - `std::fs` and file metadata
//
// - [ ] Focus: filesystem operations.
//   - Detail: Many useful command-line tools begin with filesystem inspection. This focus is about learning the standard ways Rust represents files, directories, and metadata.
// - [ ] Read/inspect:
//   - What to look for: Look at the result types returned by filesystem calls and the metadata available from them. Notice how often OS operations are fallible and therefore return `Result`.
//   - `std::fs`
//   - `std::fs::File`
//   - `std::fs::Metadata`
// - [ ] Exercise:
//   - Goal: The point is to handle filesystem data as fallible, structured information. You are practicing the habit of reporting errors instead of assuming the OS will cooperate.
//   - List files in a directory.
//   - Print file name, size, modified time if available, and whether it is a file or directory.
//   - Handle errors with `Result`, not `unwrap`.
// - [ ] Done when:
//   - You have used `read_dir`, `DirEntry`, and `metadata`.

fn main() {}
