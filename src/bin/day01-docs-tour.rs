// # Week 1: Orientation, Core Types, Strings, Option, Result
//
// Objective: stop treating the standard library as a mystery. Learn how to navigate docs and build fluency with the most common Rust value types.
//
// ## Day 1 - Standard library tour and local docs
//
// - [x] Focus: how to find things.
//   - Detail: This day is about making the standard library searchable and familiar, not memorized. Rust rewards developers who can quickly move from a type or module name to examples, trait impls, and method docs.
// - [x] Read/inspect:
//   - What to look for: Look for the shape of a docs page: module summaries, structs/enums/traits, method lists, trait implementations, and examples. Notice how local docs include your dependencies once you add them later.
//   - `std` crate root: https://doc.rust-lang.org/std/
//   - `cargo doc --open`
//   - `rustup doc --std`
// - [x] Exercise:
//   - Goal: The point is to make documentation lookup part of your normal development loop immediately. The tiny `Command` program gives you a first taste of a practical `std` module while the notes file becomes your personal index.
//   - Create a scratch project.
//   - Make a `notes/week01.md` file.
//   - Write a tiny binary that prints your Rust version by shelling out to `rustc --version` using `std::process::Command`.
//   - Browse `std` modules and write down 10 modules you expect to use often.
// - [x] Done when:
//   - You can open local docs.
//   - You have a first mental map of `std::{collections, fs, io, path, env, process, thread, sync, time, fmt}`.

fn main() {
    println!("day 1");
}
