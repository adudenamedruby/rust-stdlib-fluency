# 60-Day Rust Standard Library Learning Plan

Designed for: an experienced developer who is fairly new to Rust, understands the basics of ownership and borrowing, and wants exercise/project-based fluency with Rust's standard library plus a small set of common ecosystem crates.

Time commitment:

- Weekdays: about 45 minutes/day
- Weekends: about 1-2 hours/day
- Style: small focused exercises, short mini-projects, weekly review checkpoints

Primary goal: build a working mental map of what comes built into Rust, where to look things up, and how idiomatic Rust code tends to combine standard-library types, traits, iterators, errors, collections, files, tests, and basic concurrency.

Secondary goal: build practical familiarity with common crates that are not in `std` but are part of normal Rust application work: `anyhow`, `thiserror`, `serde`, `clap`, `regex`, `itertools`, and `tokio`.

---

## How to Use This Plan

Each day has:

- Focus: the standard-library area or ecosystem concept.
- Read/inspect: documentation to skim, not memorize.
- Exercise: code to write.
- Done when: a concrete completion target.

Recommended daily rhythm:

1. Spend 5-10 minutes reading docs.
2. Write code from memory for 20-30 minutes.
3. Use docs only when stuck.
4. Spend 5 minutes writing notes: new APIs, surprises, borrow-checker issues, and idioms.
5. Commit the day's code.

Suggested repo layout:

```text
rust-stdlib-fluency/
  Cargo.toml
  src/
    bin/
      day01_docs_tour.rs
      day02_option_result.rs
      ...
  exercises/
  notes/
    week01.md
    week02.md
```

You can use one Cargo package with multiple binaries under `src/bin/`, or a Cargo workspace if you prefer more separation. Keep it lightweight. The point is repetition, not architecture.

---

## Core Resources

Use these repeatedly during the plan.

- Rust Standard Library docs: https://doc.rust-lang.org/std/
- Official Rust documentation hub: https://doc.rust-lang.org/
- The Rust Programming Language: https://doc.rust-lang.org/book/
- Rust By Example: https://doc.rust-lang.org/stable/rust-by-example/
- The Cargo Book: https://doc.rust-lang.org/cargo/
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/

Crate references:

- anyhow: https://docs.rs/anyhow/latest/anyhow/
- thiserror: https://docs.rs/thiserror/latest/thiserror/
- serde: https://serde.rs/ and https://docs.rs/serde/latest/serde/
- clap: https://docs.rs/clap/latest/clap/
- regex: https://docs.rs/regex/latest/regex/
- itertools: https://docs.rs/itertools/latest/itertools/
- tokio tutorial: https://tokio.rs/tokio/tutorial
- tokio API docs: https://docs.rs/tokio/latest/tokio/

---

## What You Are Trying to Internalize

By the end, you should have a practical mental index for:

- Core containers: `Vec`, arrays, slices, `String`, `str`, `Box`, `Rc`, `Arc`, `Cow`
- Option and result flow: `Option`, `Result`, `?`, `map`, `and_then`, `ok_or`, `transpose`, `collect`
- Collections: `HashMap`, `BTreeMap`, `HashSet`, `BTreeSet`, `VecDeque`, `BinaryHeap`
- Iterators: `Iterator`, `IntoIterator`, adapters, consumers, `collect`, `FromIterator`, `Extend`
- Text and formatting: `fmt`, `Display`, `Debug`, `write!`, `format!`
- Filesystem and IO: `fs`, `File`, `Read`, `Write`, `BufRead`, `BufReader`, `BufWriter`, `Path`, `PathBuf`
- Environment and process: `env`, CLI args, variables, current dir, `Command`
- Errors: `std::error::Error`, `Box<dyn Error>`, `io::Error`, `anyhow`, `thiserror`
- Testing and docs: unit tests, integration tests, doc tests, `cargo doc --open`
- Concurrency basics: `thread`, `mpsc`, `Arc`, `Mutex`, `RwLock`, `OnceLock`, atomics
- Async basics: what `std` does not provide, and how `tokio` fills the runtime gap
- API design: `From`, `TryFrom`, `AsRef`, `Borrow`, `Default`, builder-like patterns, module visibility

---

# Week 1: Orientation, Core Types, Strings, Option, Result

Objective: stop treating the standard library as a mystery. Learn how to navigate docs and build fluency with the most common Rust value types.

## Day 1 - Standard library tour and local docs

- [x] Focus: how to find things.
  - Detail: This day is about making the standard library searchable and familiar, not memorized. Rust rewards developers who can quickly move from a type or module name to examples, trait impls, and method docs.
- [x] Read/inspect:
  - What to look for: Look for the shape of a docs page: module summaries, structs/enums/traits, method lists, trait implementations, and examples. Notice how local docs include your dependencies once you add them later.
  - `std` crate root: https://doc.rust-lang.org/std/
  - `cargo doc --open`
  - `rustup doc --std`
- [x] Exercise:
  - Goal: The point is to make documentation lookup part of your normal development loop immediately. The tiny `Command` program gives you a first taste of a practical `std` module while the notes file becomes your personal index.
  - Create a scratch project.
  - Make a `notes/week01.md` file.
  - Write a tiny binary that prints your Rust version by shelling out to `rustc --version` using `std::process::Command`.
  - Browse `std` modules and write down 10 modules you expect to use often.
- [x] Done when:
  - You can open local docs.
  - You have a first mental map of `std::{collections, fs, io, path, env, process, thread, sync, time, fmt}`.

## Day 2 - `Option`, `Result`, and basic flow

- [x] Focus: idiomatic absence and failure.
  - Detail: Rust uses types to make absence and failure explicit. The goal is to make `Option` and `Result` feel like normal control-flow tools rather than ceremony around simple operations.
- [x] Read/inspect:
  - What to look for: Pay attention to method names and signatures, especially whether a method consumes `self`, borrows, or returns a new wrapped value. Do not try to memorize every combinator; group them by purpose.
  - `std::option::Option`
  - `std::result::Result`
- [ ] Exercise:
  - Goal: The point is to practice representing uncertainty and failure in return types. Rewriting with combinators helps you learn the fluent style, while comparing with `match` keeps readability in view.
  - Write `parse_port(input: &str) -> Result<u16, String>`.
  - Write `first_non_empty_line(input: &str) -> Option<&str>`.
  - Rewrite both without `match`, using combinators like `map`, `and_then`, `ok_or_else`, and `filter`.
- [ ] Done when:
  - You can explain when you prefer `match` versus combinators.
  - You understand why `Option<&str>` is often better than returning an owned `String`.

## Day 3 - `String`, `str`, slices, and UTF-8 reality

roux

- [ ] Focus: owned versus borrowed text.
  - Detail: Text in Rust forces you to separate ownership from borrowing and bytes from human-readable characters. This focus prevents a lot of early pain around indexing, slicing, and unnecessary allocation.
- [ ] Read/inspect:
  - What to look for: Look for the difference between `String` methods and `str` methods, and notice how many methods return iterators. Pay special attention to docs that mention UTF-8 boundaries.
  - `std::string::String`
  - primitive `str`
- [ ] Exercise:
  - Goal: The point is to write text APIs that borrow by default and allocate only when producing new text. The UTF-8 slicing experiment should make Rust's string design feel protective rather than arbitrary.
  - Write functions that accept `&str`, not `String`, wherever possible.
  - Implement:
    - `normalize_whitespace(s: &str) -> String`
    - `count_chars_words_bytes(s: &str) -> (usize, usize, usize)`
    - `first_n_chars(s: &str, n: usize) -> String`
  - Try slicing a string at a non-character boundary and observe what happens.
- [ ] Done when:
  - You can explain the difference between bytes, chars, and grapheme clusters at a high level.
  - You stop assuming string indexing works like JavaScript/Python.

## Day 4 - Arrays, slices, `Vec`, and borrowing collections

- [ ] Focus: contiguous data and borrowed views.
  - Detail: Most day-to-day Rust data processing uses contiguous memory somewhere: arrays, slices, or vectors. The main idea is to learn when a function needs ownership and when a borrowed slice is the more flexible API.
- [ ] Read/inspect:
  - What to look for: Look for which methods live on `Vec<T>` versus slices. Notice how slice methods are often more broadly useful because arrays and vectors can both be borrowed as slices.
  - `std::vec::Vec`
  - primitive slice docs
- [ ] Exercise:
  - Goal: The point is to practice taking borrowed views over collections and using built-in slice operations. These exercises make you choose when mutation, sorting, allocation, and safe indexing are appropriate.
  - Implement:
    - `median(numbers: &mut [i32]) -> Option<f64>`
    - `dedup_sorted(numbers: &mut Vec<i32>)`
    - `window_sums(numbers: &[i32], window: usize) -> Vec<i32>`
  - Use `sort`, `windows`, `chunks`, `split_at`, and indexing safely.
- [ ] Done when:
  - You can explain why function parameters should often be `&[T]` instead of `&Vec<T>`.

## Day 5 - Ownership through iterator methods

- [ ] Focus: `iter`, `iter_mut`, `into_iter`.
  - Detail: Iterator methods are where ownership rules become very visible. This day is about deliberately choosing whether you want to read, mutate, or consume a collection.
- [ ] Read/inspect:
  - What to look for: Focus on the `Item` type each iterator produces. When reading examples, ask whether each step is borrowing, mutating, or moving values.
  - `std::iter::Iterator`
- [ ] Exercise:
  - Goal: The point is to make movement through iterator methods predictable. By writing three versions of similar logic, you will see how API choice changes ownership of the original vector and its elements.
  - Given a `Vec<String>`, write three functions:
    - one that reads names without moving them,
    - one that mutates names in place,
    - one that consumes the vector and returns transformed names.
  - Use `iter`, `iter_mut`, and `into_iter` deliberately.
- [ ] Done when:
  - You can predict whether a loop/adaptor gives you `T`, `&T`, or `&mut T`.

## Day 6 - Weekend mini-project: text stats

- [ ] Focus: strings, slices, vectors, option/result, tests.
  - Detail: This mini-project combines the core Week 1 pieces in a realistic text-processing task. You are practicing how borrowed strings, vectors, options/results, and tests fit together in one small program.
- [ ] Project:
  - Goal: The point is to combine several small concepts into one finished utility-shaped program. Tests matter here because they force you to clarify behavior for edge cases before the code becomes larger.
  - Build `text_stats`, a CLI-ish binary that reads a hard-coded multiline string and reports:
    - lines,
    - non-empty lines,
    - words,
    - bytes,
    - chars,
    - top 5 longest lines.
  - No external crates.
  - Add tests for at least 5 edge cases.
- [ ] Done when:
  - The implementation uses borrowed `&str` internally where practical.
  - You have tests for empty input, whitespace-only input, Unicode input, single-line input, and repeated words.

## Day 7 - Review checkpoint

- [ ] Review:
  - Goal: The point is retrieval and cleanup, not new material. Rewriting from memory and removing unnecessary clones will reveal which Week 1 concepts are starting to stick.
  - Rewrite one Day 2 or Day 3 function from scratch without looking.
  - Read your own code and mark any unnecessary cloning.
  - Run `cargo clippy` and address useful warnings.
- [ ] Reflection prompts:
  - Which APIs did you reach for repeatedly?
  - Where did borrowing still feel awkward?
  - Which docs pages were easiest/hardest to read?

---

# Week 2: Collections and Choosing the Right Container

Objective: learn the everyday collections and the tradeoffs between them. This week should make `std::collections` feel like a familiar toolbox.

## Day 8 - `HashMap` and the entry API

- [ ] Focus: maps, counts, updates.
  - Detail: Maps are central to summarizing, indexing, and counting. The `entry` API is especially important because it gives you a Rust-native way to update a value without doing multiple lookups.
- [ ] Read/inspect:
  - What to look for: Look at insertion, lookup, mutation, and especially `entry`. Pay attention to what requires ownership of a key and what can work with borrowed lookup values.
  - `std::collections::HashMap`
- [ ] Exercise:
  - Goal: The point is to compare two valid ways to update a map and feel why `entry` is often cleaner. Sorting the output also reinforces the difference between storage order and presentation order.
  - Write a word-frequency counter.
  - Implement it twice:
    - once with `get_mut`/`insert`,
    - once with `entry(...).or_insert(...)`.
  - Sort output by count descending, then word ascending.
- [ ] Done when:
  - You understand why `entry` is a central Rust collection idiom.

## Day 9 - `BTreeMap`, `HashSet`, and `BTreeSet`

- [ ] Focus: hashing versus ordering.
  - Detail: Rust gives you both hash-based and ordered collections because performance and determinism are different needs. This day teaches the habit of choosing a collection based on access pattern and output requirements.
- [ ] Read/inspect:
  - What to look for: Compare the guarantees each collection gives: ordering, uniqueness, and lookup behavior. Notice the APIs that overlap and the places where tree collections support range-like ordered behavior.
  - `std::collections::{BTreeMap, HashSet, BTreeSet}`
- [ ] Exercise:
  - Goal: The point is to experience how the same input can produce different summaries depending on collection choice. Sorted output should come naturally from tree collections rather than from extra sorting everywhere.
  - Given log lines with timestamps and user IDs, collect:
    - unique users,
    - users in sorted order,
    - count by day in sorted order.
  - Use both hash-based and tree-based collections.
- [ ] Done when:
  - You can explain when deterministic ordering is worth choosing `BTreeMap`/`BTreeSet`.

## Day 10 - `VecDeque` and `BinaryHeap`

- [ ] Focus: queues and priority queues.
  - Detail: Queues and priority queues solve problems that plain vectors can solve awkwardly but not cleanly. The focus is recognizing when insertion/removal patterns matter enough to pick a specialized container.
- [ ] Read/inspect:
  - What to look for: Look for operations at the front/back of `VecDeque` and push/pop behavior in `BinaryHeap`. Pay attention to whether the heap is max-oriented and how that affects top-N problems.
  - `std::collections::VecDeque`
  - `std::collections::BinaryHeap`
- [ ] Exercise:
  - Goal: The point is to model data that changes at the ends or needs priority access. Comparing heap versus sorting helps you decide when specialized data structures are worth the extra concept.
  - Implement a fixed-size recent-events buffer using `VecDeque`.
  - Implement top-N largest numbers using `BinaryHeap` or sorting; compare the approaches.
- [ ] Done when:
  - You know where to reach for queue-like behavior without abusing `Vec`.

## Day 11 - Lookup ergonomics and borrowed keys

- [ ] Focus: avoiding unnecessary allocation.
  - Detail: Good Rust APIs avoid forcing callers to allocate. This focus is about accepting borrowed or convertible inputs so your functions work with `String`, `&str`, `PathBuf`, `&Path`, and similar shapes naturally.
- [ ] Read/inspect:
  - What to look for: Do not read these traits abstractly only. Look for examples where they make function parameters more flexible or allow lookup with a borrowed form of an owned key.
  - `std::borrow::Borrow`
  - `std::convert::AsRef`
- [ ] Exercise:
  - Goal: The point is to reduce friction at API boundaries. You should feel how accepting `&str`, `&Path`, or `impl AsRef<_>` makes functions easier to call and often avoids clones.
  - Create a `HashMap<String, usize>` and look up values with `&str`.
  - Write functions that accept `impl AsRef<Path>` and `impl AsRef<str>`.
  - Try versions that take `String`, `&String`, and `&str`; note the friction.
- [ ] Done when:
  - You can explain why `&String` is usually less useful than `&str`.

## Day 12 - Ordering, hashing, equality, and derived traits

- [ ] Focus: making your own types collection-friendly.
  - Detail: Collections rely on traits like equality, ordering, and hashing. By making your own type collection-friendly, you learn what the derives actually mean and why the compiler asks for them.
- [ ] Read/inspect:
  - What to look for: Look at what each trait promises: equality, total ordering, hashing, comparison. Connect each trait to the collection that requires it.
  - `std::cmp`
  - `std::hash`
- [ ] Exercise:
  - Goal: The point is to let compiler errors teach trait requirements. Removing derives one by one makes collection constraints concrete instead of theoretical.
  - Define a `UserId(String)` newtype.
  - Put it in `HashMap`, `HashSet`, and `BTreeSet`.
  - Use derives where possible: `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`, `PartialOrd`, `Ord`.
  - Then remove one derive at a time and observe compiler errors.
- [ ] Done when:
  - You understand which traits a type needs for hash maps versus tree maps.

## Day 13 - Weekend mini-project: log summarizer

- [ ] Focus: collections, parsing, sorting, tests.
  - Detail: This project turns collection knowledge into a practical summarization tool. The important part is not perfect parsing; it is choosing containers deliberately and keeping bad input from crashing the program.
- [ ] Project:
  - Goal: The point is to build a realistic summary tool while keeping parsing intentionally basic. You should practice returning recoverable errors and choosing containers for each summary output.
  - Build `log_summary` that parses lines shaped like:
    - `2026-04-01 INFO user=roux action=login`
    - `2026-04-01 ERROR user=alex action=upload`
  - Produce:
    - count by level,
    - count by user,
    - count by day,
    - top 3 actions.
  - No external crates yet.
- [ ] Done when:
  - Uses at least 3 collection types.
  - Has tests for malformed lines.
  - Does not panic on bad input.

## Day 14 - Review checkpoint

- [ ] Review:
  - Goal: The point is to revisit the project through the lens of container choice and ownership. The written note should become a personal decision guide for future Rust work.
  - Refactor Day 13 to reduce cloning.
  - Replace one `Vec` with a more appropriate collection.
  - Write a short note: "When I choose each collection".
- [ ] Reflection prompts:
  - Which collection API felt most Rust-specific?
  - Did `entry` click?
  - Where did owned versus borrowed keys matter?

---

# Week 3: Iterator Fluency

Objective: become comfortable reading and writing iterator-heavy Rust without turning every program into unreadable point-free soup.

## Day 15 - Iterator adapters and consumers

- [ ] Focus: `map`, `filter`, `filter_map`, `fold`, `sum`, `count`, `collect`.
  - Detail: Iterator fluency is a core Rust skill, but clarity still wins. This focus is about learning the common adapters and consumers while keeping the ability to drop back to a loop when that reads better.
- [ ] Read/inspect:
  - What to look for: Skim adapter and consumer methods by category. Your goal is to recognize common names and understand when chains are lazy versus when a consumer actually runs the work.
  - `std::iter::Iterator`
- [ ] Exercise:
  - Goal: The point is to build translation skill between loops and iterator chains. You should end the day more comfortable choosing the form that makes intent clearest.
  - Rewrite simple loops from Week 1 and 2 using iterators.
  - Then rewrite one iterator chain back into a loop.
- [ ] Done when:
  - You can choose clarity over cleverness, and justify it.

## Day 16 - Iterating over `Option` and `Result`

- [ ] Focus: fallible pipelines.
  - Detail: Real data pipelines often combine iteration with fallibility. This day teaches how `Option`, `Result`, and `Iterator` compose so you can express skip, collect, or fail-fast behavior cleanly.
- [ ] Read/inspect:
  - What to look for: Look for examples where `collect` changes an iterator of results into a result of a collection. For `transpose`, focus on the shape change between nested `Option` and `Result`.
  - `Option::transpose`
  - `Result::transpose`
  - `Iterator::collect` examples involving `Result`
- [ ] Exercise:
  - Goal: The point is to learn three common policies for messy input: ignore bad values, fail fast, or report both successes and failures. This is the basis for many real import/parsing tasks.
  - Parse a list of strings into numbers.
  - Implement:
    - collect all valid numbers and ignore invalid ones,
    - fail on the first invalid number,
    - return both valid numbers and rejected strings.
- [ ] Done when:
  - You understand `collect::<Result<Vec<_>, _>>()`.

## Day 17 - `IntoIterator`, `FromIterator`, and `Extend`

- [ ] Focus: collection construction and generic iteration.
  - Detail: These traits are what make Rust functions feel flexible at the call site. The focus is writing code that accepts many iterable inputs without giving up type safety.
- [ ] Read/inspect:
  - What to look for: Look for the relationship between accepting inputs, building collections, and extending existing collections. The key is understanding why many APIs accept `IntoIterator` rather than a specific collection.
  - `std::iter::IntoIterator`
  - `std::iter::FromIterator`
  - `std::iter::Extend`
- [ ] Exercise:
  - Goal: The point is to make one function work with several caller-owned shapes. The tests prove whether the generic signature is genuinely ergonomic rather than just abstract.
  - Write `fn histogram<I, S>(items: I) -> HashMap<String, usize>` where `I: IntoIterator<Item = S>` and `S: AsRef<str>`.
  - Write tests using `Vec<&str>`, `Vec<String>`, and arrays.
- [ ] Done when:
  - You can write one generic function that accepts several collection forms.

## Day 18 - Closures, sorting, and capture

- [ ] Focus: closures as small behavior units.
  - Detail: Closures are how you pass small pieces of behavior into library methods like sorting. This day also makes capture rules concrete, which helps with both iterators and threaded code later.
- [ ] Read/inspect:
  - What to look for: Look for how closure traits relate to capture behavior: `Fn`, `FnMut`, and `FnOnce`. In sorting docs, notice when stable versus unstable sorting matters.
  - Rust Book closure chapter
  - `slice::sort_by`, `sort_by_key`, `sort_unstable_by_key`
- [ ] Exercise:
  - Goal: The point is to practice passing logic into standard-library algorithms. Returning a closure with `impl Fn` also introduces how Rust represents behavior in types.
  - Sort a list of records by multiple fields.
  - Use closures that borrow external state.
  - Try to return a closure from a function using `impl Fn`.
- [ ] Done when:
  - You can explain closure capture by borrow, mutable borrow, and move at a basic level.

## Day 19 - Custom iterators

- [ ] Focus: implementing `Iterator`.
  - Detail: Custom iterators reveal that iterator chains are built on a simple protocol. Implementing `next` yourself makes the rest of `Iterator` feel less magical.
- [ ] Read/inspect:
  - What to look for: Focus on the required method and the meaning of returning `Some` versus `None`. Everything else on the trait builds from that repeated call pattern.
  - `Iterator::next`
- [ ] Exercise:
  - Goal: The point is to demystify iterators by building your own. Once you implement `next`, adapters and consumers should feel like reusable layers on a simple interface.
  - Implement a `LinesWithNumbers` iterator over `&str` that yields `(usize, &str)`.
  - Implement a simple `TakeUntil` iterator adapter.
- [ ] Done when:
  - You understand that most iterator magic comes from one method: `next`.

## Day 20 - Weekend mini-project: small CSV-ish parser

- [ ] Focus: iterator pipelines, string parsing, result handling.
  - Detail: This project makes you combine parsing, iteration, and errors in a bounded problem. Keeping the CSV rules intentionally simple lets you focus on Rust structure rather than parser edge cases.
- [ ] Project:
  - Goal: The point is to integrate parsing, errors, and summaries without expanding the format too far. Maintaining both loop and iterator versions gives you a concrete readability comparison.
  - Build a tiny parser for comma-separated rows.
  - Keep it intentionally limited: no quoted commas required.
  - Return structured records and useful errors.
  - Produce summary stats using iterator chains.
- [ ] Done when:
  - You have both loop-based and iterator-based versions of one processing step.
  - Tests cover empty rows, missing columns, and invalid numeric fields.

## Day 21 - Review checkpoint and `itertools` preview

- [ ] Review:
  - Goal: The point is to compare standard iterator fluency with a popular extension crate. You should only keep the `itertools` version when it makes the code easier to understand or maintain.
  - Read `itertools` docs for 15 minutes, especially `Itertools` trait methods.
  - Refactor one Week 3 exercise using `itertools` only if it genuinely simplifies the code.
- [ ] Reflection prompts:
  - Which iterator chains improved clarity?
  - Which ones became too clever?
  - What methods do you wish `std::iter` had?

---

# Week 4: Files, Paths, IO, Environment, and Processes

Objective: get comfortable writing practical Rust utilities that touch the operating system.

## Day 22 - `std::fs` and file metadata

- [ ] Focus: filesystem operations.
  - Detail: Many useful command-line tools begin with filesystem inspection. This focus is about learning the standard ways Rust represents files, directories, and metadata.
- [ ] Read/inspect:
  - What to look for: Look at the result types returned by filesystem calls and the metadata available from them. Notice how often OS operations are fallible and therefore return `Result`.
  - `std::fs`
  - `std::fs::File`
  - `std::fs::Metadata`
- [ ] Exercise:
  - Goal: The point is to handle filesystem data as fallible, structured information. You are practicing the habit of reporting errors instead of assuming the OS will cooperate.
  - List files in a directory.
  - Print file name, size, modified time if available, and whether it is a file or directory.
  - Handle errors with `Result`, not `unwrap`.
- [ ] Done when:
  - You have used `read_dir`, `DirEntry`, and `metadata`.

## Day 23 - `std::io`, buffered reads, and writes

- [ ] Focus: reading and writing streams.
  - Detail: Streams are data sources or sinks that you process progressively instead of loading everything at once. Rust's IO traits let the same code work over files, buffers, stdin/stdout, network streams, and other readers or writers; buffering keeps that efficient.
- [ ] Read/inspect:
  - What to look for: Look for the roles of each trait: `Read` pulls bytes, `Write` pushes bytes, `BufRead` adds line-oriented helpers, and buffered wrappers reduce repeated system calls. Notice that these abstractions are generic over many data sources.
  - `std::io::{Read, Write, BufRead, BufReader, BufWriter}`
- [ ] Exercise:
  - Goal: The point is to learn the tradeoff between convenience and scalability. Reading a whole file is often fine; buffered streaming is better when inputs are large, continuous, or should be processed line by line.
  - Read a file line by line with `BufReader`.
  - Write transformed lines to another file with `BufWriter`.
  - Compare with `fs::read_to_string`.
- [ ] Done when:
  - You understand when whole-file reads are fine and when buffered IO is better.

## Day 24 - `Path`, `PathBuf`, `OsStr`, and platform reality

- [ ] Focus: paths are not strings.
  - Detail: Paths are operating-system values, not just UTF-8 strings. This focus protects your code from subtle portability problems and leads to better function signatures.
- [ ] Read/inspect:
  - What to look for: Look for methods that manipulate path components without converting to strings. Pay attention to `OsStr` because it explains why not every path is valid UTF-8.
  - `std::path::{Path, PathBuf}`
  - `std::ffi::{OsStr, OsString}`
- [ ] Exercise:
  - Goal: The point is to make path manipulation type-correct. Avoiding string conversion will make your code more portable and will prepare you for real CLI/file tools.
  - Write `fn change_extension(path: &Path, ext: &str) -> PathBuf`.
  - Walk through a directory and collect files by extension.
  - Avoid converting paths to strings unless printing.
- [ ] Done when:
  - You stop representing paths as `String` in function signatures.

## Day 25 - `std::env` and basic CLI input

- [ ] Focus: environment and args without `clap`.
  - Detail: Before using a CLI framework, it helps to know the standard baseline. Manual parsing will feel clumsy, and that discomfort is useful context for understanding what `clap` buys you later.
- [ ] Read/inspect:
  - What to look for: Look for what `env::args` gives you and what it does not: no validation model, no help text, and no structured flag handling. That gap is the lesson.
  - `std::env`
- [ ] Exercise:
  - Goal: The point is to understand the lowest-level CLI input available in `std`. You should come away able to write a tiny parser, and also able to explain why you would usually use `clap` for serious tools.
  - Write a small binary that accepts:
    - an input path,
    - an optional `--uppercase` flag,
    - an optional environment variable for default output path.
  - Parse manually using `env::args`.
- [ ] Done when:
  - You appreciate why `clap` exists, but know the standard baseline.

## Day 26 - `std::process::Command`

- [ ] Focus: shelling out safely.
  - Detail: Shelling out is common in developer tools, but it has distinct failure modes. This focus separates failure to launch a command from a command that runs successfully but exits with an error status.
- [ ] Read/inspect:
  - What to look for: Look closely at `output`, `status`, and `spawn`, and at how stdout/stderr are represented. Separate process setup, execution, captured output, and exit status in your head.
  - `std::process::{Command, Output, Stdio}`
- [ ] Exercise:
  - Goal: The point is to treat subprocesses as fallible external systems. Capturing output and checking status explicitly prevents a common bug: assuming a command succeeded because it launched.
  - Run `rustc --version` and capture output.
  - Run `cargo metadata` or `git status` if available.
  - Handle non-zero exit status explicitly.
- [ ] Done when:
  - You can distinguish command spawn errors from unsuccessful exit status.

## Day 27 - Weekend mini-project: `grep_lite`

- [ ] Focus: paths, files, buffered IO, errors.
  - Detail: This project ties together path handling, buffered IO, search logic, and user-facing errors. It is intentionally pre-regex so you understand the simple standard-library version first.
- [ ] Project:
  - Goal: The point is to build a complete file-processing tool with only standard-library tools. Separating search logic from IO gives you testable code and a cleaner path to later adding `regex` or `clap`.
  - Build `grep_lite` using only `std`.
  - Inputs: search term and file path.
  - Output matching lines with line numbers.
  - Add optional case-insensitive mode.
  - No `regex` yet.
- [ ] Done when:
  - Handles missing args, missing files, invalid UTF-8-ish situations as gracefully as you can at this stage.
  - Has tests for the search logic independent of file IO.

## Day 28 - Review checkpoint

- [ ] Review:
  - Goal: The point is to improve design after the first working version. Refactoring toward pure functions and path-friendly signatures is exactly how small Rust tools become maintainable.
  - Refactor Day 27 into pure logic plus IO shell.
  - Change function signatures to accept `impl AsRef<Path>` where useful.
  - Run `cargo clippy`.
- [ ] Reflection prompts:
  - Where should your code own a `PathBuf`?
  - Where should it borrow a `&Path`?
  - What error messages would help a real user?

---

# Week 5: Error Handling, Testing, and Library Boundaries

Objective: learn Rust's error stack from `std` first, then add the common application/library crates.

## Day 29 - `std::error::Error` and `Box<dyn Error>`

- [ ] Focus: standard error trait.
  - Detail: Rust's standard error trait is the foundation underneath both standard and ecosystem error handling. This day teaches the tradeoff between convenient erased errors and more precise typed errors.
- [ ] Read/inspect:
  - What to look for: Look for the minimum contract of an error: display, debug, and optional source chaining. Notice how `io::Error` carries a kind plus more detailed platform information.
  - `std::error::Error`
  - `std::io::Error`
- [ ] Exercise:
  - Goal: The point is to see how one return type can carry multiple underlying error kinds. This is convenient for application code, but the exercise should also reveal what specificity you lose.
  - Write a function that reads a file and parses numbers from it.
  - Return `Result<Vec<i32>, Box<dyn std::error::Error>>`.
  - Use `?` across both IO and parse errors.
- [ ] Done when:
  - You understand why boxed dynamic errors are convenient but less specific.

## Day 30 - Manual custom errors

- [ ] Focus: custom error type without macros.
  - Detail: Writing a custom error type manually shows the machinery behind ergonomic error crates. You are learning what `Display`, `Error`, and `From` contribute to the `?` workflow.
- [ ] Read/inspect:
  - What to look for: Look for how `Display` differs from `Debug`, and how implementing `Error` lets your type participate in normal error handling. Notice how `From` supports the `?` operator.
  - `std::fmt::Display`
  - `std::error::Error`
- [ ] Exercise:
  - Goal: The point is to build the error plumbing yourself once. After this, derive-based error crates will feel like time-savers rather than black boxes.
  - Create an enum `ConfigError` with variants for IO, parse, and missing field.
  - Implement `Display` and `Error` manually.
  - Implement `From<std::io::Error>` where useful.
- [ ] Done when:
  - You understand what `thiserror` automates.

## Day 31 - `thiserror` for library-like errors

- [ ] Focus: ergonomic typed errors.
  - Detail: `thiserror` removes repetitive error boilerplate while keeping your error type explicit. This is the pattern you usually want in library-like code where callers may need to inspect error variants.
- [ ] Read/inspect:
  - What to look for: Look for how attributes map enum variants to display messages and source errors. The point is to recognize the boilerplate being generated, not to learn every attribute immediately.
  - `thiserror` docs
- [ ] Exercise:
  - Goal: The point is to keep typed errors while removing boilerplate. Matching variants in tests reinforces why typed errors are useful when callers need to respond differently to different failures.
  - Rewrite Day 30 using `thiserror`.
  - Add tests that match specific error variants.
- [ ] Done when:
  - You can explain why `thiserror` is good for reusable/library-ish code.

## Day 32 - `anyhow` for application boundaries

- [ ] Focus: context-rich app errors.
  - Detail: Applications often need helpful context more than inspectable error types. `anyhow` is useful at the outer edge of a program where you want failures to explain what the program was trying to do.
- [ ] Read/inspect:
  - What to look for: Look for `Context` and how it adds information at the call site. Pay attention to the difference between preserving the underlying error and improving the top-level message.
  - `anyhow` docs
- [ ] Exercise:
  - Goal: The point is to add human context at the application boundary without polluting lower-level code. This is where error messages become useful to the person running the program.
  - Write a binary that calls your library-like code.
  - Use `anyhow::Result` and `.context(...)` / `.with_context(...)` at the app boundary.
- [ ] Done when:
  - You can explain `thiserror` inside libraries, `anyhow` near application edges.

## Day 33 - Testing: unit, integration, and doc tests

- [ ] Focus: testing habits.
  - Detail: Testing in Rust is lightweight enough to use as part of design. This focus is about seeing unit, integration, and doc tests as complementary tools rather than separate rituals.
- [ ] Read/inspect:
  - What to look for: Look for where each kind of test lives and how Cargo discovers it. Pay attention to the difference between testing private helpers, public APIs, and examples in docs.
  - Rust Book testing chapter
  - Cargo Book testing/package layout sections
- [ ] Exercise:
  - Goal: The point is to cover code at different boundaries. Unit tests protect small behavior, integration tests protect public behavior, and doc tests keep examples honest.
  - Add unit tests to one pure function.
  - Add an integration test under `tests/`.
  - Add one doc comment with a runnable example.
- [ ] Done when:
  - `cargo test` runs all three kinds of tests.

## Day 34 - Weekend mini-project: fallible config loader

- [ ] Focus: errors, tests, filesystem.
  - Detail: This project forces error boundaries to become concrete. It gives you practice separating reusable parsing logic from the application shell that reads files and reports failures.
- [ ] Project:
  - Goal: The point is to build a small but realistic fallible library. You should be able to tell which errors belong to parsing/config logic and which belong to the binary that reads files.
  - Build `config_loader`.
  - Input: simple key-value text file like `host=localhost` and `port=8080`.
  - Output: a `Config` struct.
  - Use `thiserror` for typed library errors.
  - Use `anyhow` in the binary.
  - Test malformed configs.
- [ ] Done when:
  - Error messages are useful.
  - Tests can identify specific error variants.

## Day 35 - Review checkpoint

- [ ] Review:
  - Goal: The point is to turn error-handling experience into rules you can reuse. Removing `unwrap` from older code should make the difference between prototypes and robust utilities obvious.
  - Write a short note: "My Rust error-handling rules of thumb".
  - Refactor one older exercise to remove `unwrap`.
  - Add context to app-level errors.
- [ ] Reflection prompts:
  - Where do typed errors matter?
  - Where is `anyhow` enough?
  - Did tests make refactoring easier?

---

# Week 6: Practical Ecosystem Layer - Serde, Clap, Regex, Itertools

Objective: learn a small set of crates that Rust developers commonly reach for, while keeping a clear distinction between `std` and ecosystem tools.

## Day 36 - `serde` basics

- [ ] Focus: serialization and deserialization.
  - Detail: Serde is the standard ecosystem answer for turning Rust data into external formats and back. The focus is on understanding derive-based serialization before worrying about advanced attributes.
- [ ] Read/inspect:
  - What to look for: Look for derive usage first, then field naming and format examples. The main thing to learn is how Rust structs map to external data without hand-written parsing code.
  - serde overview
  - `serde::{Serialize, Deserialize}`
- [ ] Exercise:
  - Goal: The point is to make Rust structs cross a data-format boundary with minimal manual code. Round-tripping JSON helps you see both serialization and deserialization as part of one model.
  - Add `serde` and `serde_json`.
  - Define structs for the Week 2 log summary output.
  - Serialize summary data to pretty JSON.
  - Deserialize sample JSON back into structs.
- [ ] Done when:
  - You understand derive-based serialization at a basic level.

## Day 37 - `clap` derive CLI

- [ ] Focus: real CLI parsing.
  - Detail: `clap` replaces fragile manual argument parsing with typed, documented CLI structure. The goal is to see how much user-facing behavior you get from a well-defined argument model.
- [ ] Read/inspect:
  - What to look for: Look for the derive model: struct fields become arguments, attributes shape parsing and help output, and types drive validation where possible.
  - `clap` derive tutorial/docs
- [ ] Exercise:
  - Goal: The point is to replace ad hoc CLI parsing with declarative structure. Your reward should be cleaner code and a better user experience through generated help and validation.
  - Replace manual arg parsing in `grep_lite` or `config_loader` with `clap`.
  - Add flags, positional args, and help text.
- [ ] Done when:
  - `--help` produces useful output without custom code.

## Day 38 - `regex` basics

- [ ] Focus: regular expressions in Rust.
  - Detail: Rust keeps regex outside `std`, but the `regex` crate is a common tool for real parsing and search tasks. This focus is about compiling patterns, using captures, and handling invalid patterns safely.
- [ ] Read/inspect:
  - What to look for: Look for `Regex::new`, matching APIs, captures, and error handling for invalid patterns. Also notice the crate's guarantees and limitations compared with backtracking regex engines.
  - `regex` crate docs
- [ ] Exercise:
  - Goal: The point is to add pattern matching without making parsing reckless. Compiling once, reusing the regex, and handling invalid patterns are the habits that matter.
  - Upgrade `grep_lite` to support regex search.
  - Extract named captures from log lines.
  - Handle invalid regex patterns gracefully.
- [ ] Done when:
  - You can compile a `Regex` once and reuse it.

## Day 39 - Combining `regex`, collections, and errors

- [ ] Focus: practical parsing pipeline.
  - Detail: This day combines regex with the collection and error skills you already built. The important skill is turning text matches into typed records without letting parsing details leak everywhere.
- [ ] Exercise:
  - Goal: The point is to build a clearer parsing pipeline from raw text to captures to typed summaries. Typed errors should tell you which part of that pipeline failed.
  - Rewrite the Week 2 log parser using `regex` named captures.
  - Count by captured fields.
  - Use typed errors for invalid lines.
- [ ] Done when:
  - Your regex code does not allocate more than needed in obvious places.

## Day 40 - `itertools`: know when it helps

- [ ] Focus: extending iterator ergonomics.
  - Detail: `itertools` extends the standard iterator toolbox with convenience methods. The focus is judgement: adding a crate should make the code clearer or more direct, not just fancier.
- [ ] Read/inspect:
  - What to look for: Look for methods that correspond to code you have already written manually. Keep notes on which methods feel like clear wins and which feel too magical.
  - `itertools::Itertools`
- [ ] Exercise:
  - Goal: The point is to test whether extra iterator power improves the code. Keeping both versions prevents cargo-culting a crate when `std` is already clear enough.
  - Take a previous iterator-heavy exercise and try:
    - `sorted`,
    - `dedup`,
    - `group_by`/grouping-style methods available in your version,
    - `join`,
    - `counts`.
  - Keep the std-only version beside it.
- [ ] Done when:
  - You can decide whether `itertools` improves or obscures the code.

## Day 41 - Weekend mini-project: report generator CLI

- [ ] Focus: std plus practical crates.
  - Detail: This mini-project is the first intentionally realistic Rust CLI in the plan. You are combining `std` and common crates while keeping a clear boundary between parsing, summarizing, output, and app wiring.
- [ ] Project:
  - Goal: The point is to assemble a realistic command-line report generator from small, testable parts. Each crate has a job; the design exercise is keeping those jobs from blurring together.
  - Build `reporter`.
  - Input: path to a log file.
  - CLI: `reporter --input logs.txt --format json|text --min-level ERROR`.
  - Use:
    - `clap` for args,
    - `regex` for parsing,
    - `serde_json` for JSON output,
    - `thiserror` for parser/library errors,
    - `anyhow` in `main`.
- [ ] Done when:
  - Both text and JSON output work.
  - Bad input gives clear errors.
  - Tests cover parser and summary logic.

## Day 42 - Review checkpoint

- [ ] Review:
  - Goal: The point is to form judgement about dependencies. Your table should help you decide when a crate earns its place versus when the standard library is enough.
  - Make a table in your notes: "std solution versus crate solution".
  - Identify which crates you would add by default and which you would wait to justify.
- [ ] Reflection prompts:
  - Which crate felt most immediately useful?
  - Which crate hid complexity you should still understand?
  - What belongs in `std`, and what is better as a crate?

---

# Week 7: Basic Concurrency and Async Orientation

Objective: learn the standard concurrency primitives well enough to read and write simple threaded Rust, then get a beginner-level orientation to Tokio.

## Day 43 - `std::thread`

- [ ] Focus: spawning and joining threads.
  - Detail: Threads are Rust's standard-library baseline for parallel execution. This focus is about ownership across thread boundaries and why spawned work usually needs owned or `'static` data.
- [ ] Read/inspect:
  - What to look for: Look for `spawn`, `JoinHandle`, and the examples using `move`. Pay attention to why thread closures differ from ordinary local closures.
  - `std::thread`
- [ ] Exercise:
  - Goal: The point is to make ownership transfer into threads concrete. The failed borrowed-data attempt is part of the lesson, because it shows what Rust is preventing.
  - Spawn several threads that compute partial sums.
  - Join them and combine results.
  - Try capturing borrowed data, then fix with `move` and owned data.
- [ ] Done when:
  - You understand why spawned threads usually need `'static` data or owned values.

## Day 44 - Channels with `std::sync::mpsc`

- [ ] Focus: message passing.
  - Detail: Message passing is often simpler than shared mutable state. Channels let workers communicate results back to one place while keeping ownership movement explicit.
- [ ] Read/inspect:
  - What to look for: Look for the sender/receiver split and what happens when either side is dropped. Notice that messages move through the channel, which often simplifies ownership.
  - `std::sync::mpsc`
- [ ] Exercise:
  - Goal: The point is to coordinate concurrent work without sharing mutable structures. Sending successes and errors through the same channel also mirrors real worker pipelines.
  - Build a worker setup where threads send results back to the main thread.
  - Send either successful results or errors.
- [ ] Done when:
  - You can use channels without shared mutable state.

## Day 45 - Shared state with `Arc`, `Mutex`, and `RwLock`

- [ ] Focus: safe shared ownership.
  - Detail: Shared state is sometimes necessary, but Rust makes the synchronization visible. This day teaches both how `Arc<Mutex<T>>` works and why reducing shared state is often cleaner.
- [ ] Read/inspect:
  - What to look for: Look for what `Arc` solves versus what `Mutex` or `RwLock` solves. Notice the locking APIs and the possibility of poisoning after panic.
  - `std::sync::{Arc, Mutex, RwLock}`
- [ ] Exercise:
  - Goal: The point is to compare two concurrency designs: shared state versus local work plus merge. You should not leave thinking `Arc<Mutex<_>>` is always wrong, only that it is not always best.
  - Count words across multiple chunks using shared `Arc<Mutex<HashMap<_, _>>>`.
  - Then rewrite using per-thread maps plus merge at the end.
  - Compare which design is simpler.
- [ ] Done when:
  - You know that `Arc<Mutex<T>>` is useful, but not always the best first design.

## Day 46 - One-time init and atomics

- [ ] Focus: basic synchronization vocabulary.
  - Detail: Some concurrency tools are mostly vocabulary until you need them. This focus gives you basic recognition of one-time initialization and atomic counters without pretending atomic memory ordering is simple.
- [ ] Read/inspect:
  - What to look for: Look for simple, common examples rather than advanced memory-ordering theory. The goal is to recognize safe global initialization and basic atomic counters.
  - `std::sync::OnceLock`
  - `std::sync::LazyLock`
  - `std::sync::atomic`
- [ ] Exercise:
  - Goal: The point is to build recognition-level fluency with synchronization primitives you will see in real Rust code. Keep the examples simple and write down what you are intentionally not trying to master yet.
  - Create a lazily initialized config or regex-like static value.
  - Use an `AtomicUsize` counter across threads.
  - Keep memory ordering simple: use `Ordering::Relaxed` for a plain counter and note why deeper atomic ordering is a separate topic.
- [ ] Done when:
  - You know these tools exist and can read simple examples without panic.

## Day 47 - Tokio orientation

- [ ] Focus: async runtime basics.
  - Detail: Async Rust is split between language features and runtimes. This orientation shows that Tokio supplies the executor and async IO tools that the standard library intentionally does not provide.
- [ ] Read/inspect:
  - What to look for: Look for the runtime entry point, task spawning, and sleeping/timing examples. Keep the question in mind: what is Tokio providing that `std` threads did not?
  - Tokio tutorial introduction
  - `tokio::main`, `tokio::spawn`, `tokio::time`
- [ ] Exercise:
  - Goal: The point is to get a first working mental model of async tasks. Sleeping tasks are simple on purpose: they isolate spawning, awaiting, and scheduling from networking complexity.
  - Create a tiny Tokio binary.
  - Spawn several async tasks that sleep and return values.
  - Await their `JoinHandle`s.
- [ ] Done when:
  - You understand that `async` syntax is in Rust, but a runtime like Tokio does the scheduling and IO work.

## Day 48 - Weekend mini-project: parallel file stats

- [ ] Focus: practical concurrency.
  - Detail: This project turns concurrency concepts into a practical batch-processing utility. The goal is to handle independent work safely, collect results, and keep individual failures from sinking the whole run.
- [ ] Project:
  - Goal: The point is to use threads for independent IO-ish work and report partial failure cleanly. The stretch Tokio version is optional because standard threading should be solid first.
  - Build `parallel_stats`.
  - Input: list of file paths.
  - Compute line/word/byte counts per file in parallel with std threads.
  - Send results back over channels.
  - Print a combined summary.
- [ ] Stretch:
  - Add a Tokio version only if the std-threaded version is working.
- [ ] Done when:
  - Errors for individual files do not crash the whole program.
  - You can explain thread-per-file limitations.

## Day 49 - Review checkpoint

- [ ] Review:
  - Goal: The point is to compare concurrency models in your own words. Removing unnecessary shared state is a strong sign that you are starting to design concurrent Rust instead of just making it compile.
  - Write a note comparing:
    - ownership transfer to threads,
    - message passing,
    - shared state,
    - async tasks.
  - Refactor one threaded exercise to remove unnecessary shared state.
- [ ] Reflection prompts:
  - Which model felt simplest?
  - Where did lifetimes become stricter?
  - What would you need to learn before writing production async Rust?

---

# Week 8: API Design, Traits, Formatting, Docs, and Reuse

Objective: start writing Rust that feels like it belongs in the ecosystem: clear types, useful traits, good errors, tests, and docs.

## Day 50 - Smart pointers and ownership tools

- [ ] Focus: `Box`, `Rc`, `Arc`, `Cow`.
  - Detail: Rust has several ownership helpers because different sharing problems have different constraints. This focus is about recognizing heap allocation, single-threaded sharing, multi-threaded sharing, and copy-on-write.
- [ ] Read/inspect:
  - What to look for: Look for the ownership problem each type solves. Compare single ownership on the heap, reference-counted sharing, thread-safe sharing, and borrowed-or-owned data.
  - `std::boxed::Box`
  - `std::rc::Rc`
  - `std::sync::Arc`
  - `std::borrow::Cow`
- [ ] Exercise:
  - Goal: The point is to connect pointer types to specific ownership situations. The examples should make you cautious about reaching for shared ownership when borrowing or moving would be simpler.
  - Use `Box` for a recursive enum.
  - Use `Rc` in a single-threaded shared ownership example.
  - Use `Cow<'_, str>` in a function that sometimes borrows and sometimes allocates.
- [ ] Done when:
  - You can explain why Rust has several ownership helpers instead of one universal reference type.

## Day 51 - Conversion traits

- [ ] Focus: ergonomic APIs.
  - Detail: Conversion traits make APIs easier to call without weakening types. This day is about designing inputs so callers can pass natural values while your internals still use precise domain types.
- [ ] Read/inspect:
  - What to look for: Look for which conversions should be infallible versus fallible. Notice that `From` implies `Into`, while `TryFrom` and `FromStr` communicate validation.
  - `std::convert::{From, Into, TryFrom, TryInto, AsRef}`
- [ ] Exercise:
  - Goal: The point is to make invalid states harder to represent while keeping caller ergonomics good. Domain types plus conversion traits are a common Rust API-design pattern.
  - Create domain types like `Port`, `UserId`, or `LogLevel`.
  - Implement `FromStr` or `TryFrom<&str>` where useful.
  - Use `impl AsRef<Path>` in IO-facing APIs.
- [ ] Done when:
  - You understand why callers like APIs that accept common input shapes.

## Day 52 - Formatting and output

- [ ] Focus: `Debug`, `Display`, and formatting machinery.
  - Detail: Formatting is both user-interface and debugging infrastructure. This focus teaches when to expose a clean human-readable representation and when to rely on structural debug output.
- [ ] Read/inspect:
  - What to look for: Look for formatter usage and examples of implementing `Display`. Pay attention to `write!` because it lets formatting target any formatter or string buffer, not just stdout.
  - `std::fmt`
  - formatting syntax docs
- [ ] Exercise:
  - Goal: The point is to control how your types appear in reports, logs, errors, and debugging output. This is a small API surface that heavily affects usability.
  - Implement `Display` for several domain types.
  - Use `write!` to build a `String` report.
  - Compare `format!`, `println!`, and `write!`.
- [ ] Done when:
  - You know when to implement `Display` versus relying on `Debug`.

## Day 53 - Modules, visibility, documentation, and `cargo doc`

- [ ] Focus: making code navigable.
  - Detail: A Rust project becomes easier to maintain when its module boundaries match its concepts. This focus is about making code, public APIs, and generated docs navigable.
- [ ] Read/inspect:
  - What to look for: Look for visibility rules, file layout conventions, and how docs are generated from public items. Your goal is to see documentation as part of the public API.
  - Rust Book modules chapter
  - Cargo docs for package layout
- [ ] Exercise:
  - Goal: The point is to turn a script-like project into a navigable package. Documentation generation should show whether your public surface area makes sense.
  - Take one mini-project and split it into:
    - `lib.rs`,
    - parser module,
    - model module,
    - output module,
    - `main.rs`.
  - Add public docs to exported types.
  - Run `cargo doc --open`.
- [ ] Done when:
  - Your own docs are useful enough that future-you can re-enter the project quickly.

## Day 54 - Rust API Guidelines pass

- [ ] Focus: idiomatic public surface area.
  - Detail: The API Guidelines help turn working code into idiomatic Rust code. This focus shifts your attention from implementation to the experience of someone calling or maintaining your code.
- [ ] Read/inspect:
  - What to look for: Look through the checklist as a source of concrete audit questions. Focus on items you can apply immediately to your own small projects.
  - Rust API Guidelines checklist
- [ ] Exercise:
  - Goal: The point is to practice API review, not just code cleanup. Five concrete refactors force you to translate guidelines into better names, types, errors, or docs.
  - Pick one mini-project and audit it for:
    - naming,
    - trait derives,
    - error types,
    - `Default`,
    - `From`/`TryFrom`,
    - avoiding boolean traps,
    - useful docs.
- [ ] Done when:
  - You have at least 5 concrete refactors based on API design, not just implementation cleanup.

## Day 55 - Weekend mini-project: small reusable library plus CLI wrapper

- [ ] Focus: reusable core plus application shell.
  - Detail: This project practices the common Rust architecture of reusable library core plus thin CLI wrapper. The focus is keeping business logic testable and keeping application concerns at the edge.
- [ ] Project:
  - Goal: The point is to separate reusable logic from executable plumbing. If the CLI is thin and the library is easy to test, you are moving toward idiomatic Rust project structure.
  - Build a small library from one previous exercise, such as text stats, log summary, config parsing, or grep matching.
  - Provide:
    - typed input/output structs,
    - typed library errors with `thiserror`,
    - unit tests,
    - doc tests,
    - CLI wrapper using `clap` and `anyhow`.
- [ ] Done when:
  - The core library can be tested without invoking the CLI.
  - The CLI is thin.

## Day 56 - Review checkpoint

- [ ] Review:
  - Goal: The point is to validate the whole Week 8 result with tools and documentation. The top-20 API list is also a confidence check on your growing standard-library vocabulary.
  - Run `cargo test`, `cargo clippy`, and `cargo doc --open`.
  - Write a short README for the Day 55 project.
  - Note the top 20 std APIs you now recognize.
- [ ] Reflection prompts:
  - Does your code expose borrowed or owned values appropriately?
  - Are error types at the right boundary?
  - Could another Rust developer use your library without reading the implementation?

---

# Days 57-60: Consolidation and Final Assessment

Objective: force retrieval, identify weak spots, and leave with a clear next-step map.

## Day 57 - Blank-page rebuild

- [ ] Focus: retrieval practice.
  - Detail: Retrieval practice exposes the difference between recognition and fluency. Rebuilding without looking forces the APIs you have learned to come from memory first.
- [ ] Exercise:
  - Goal: The point is to expose what you can actually reproduce without prompts. The comparison step is where you turn memory gaps into targeted follow-up notes.
  - Choose one previous mini-project.
  - Rebuild the core logic from scratch in a fresh file without looking at your old implementation for the first 30 minutes.
  - After that, compare and improve.
- [ ] Done when:
  - You can see which APIs are now automatic and which still need lookup.

## Day 58 - Standard library scavenger hunt

- [ ] Focus: breadth without memorization.
  - Detail: You do not need deep mastery of every module, but you do need a broad mental index. This focus is about knowing that certain tools exist so you can find them later.
- [ ] Exercise:
  - Goal: The point is breadth and discoverability, not mastery. A tiny snippet or note per module is enough to make future lookup faster and less intimidating.
  - Spend 10 minutes each exploring these modules:
    - `std::time`
    - `std::net`
    - `std::cell`
    - `std::mem`
    - `std::ops`
    - `std::cmp`
  - For each, write one tiny code snippet or note explaining when you might need it.
- [ ] Done when:
  - You have a broader mental index, even for modules you do not use daily.

## Day 59 - Refactoring challenge

- [ ] Focus: idiomatic cleanup.
  - Detail: Refactoring is where Rust idioms become visible. This day asks you to improve old code using better ownership, error, iterator, and module choices rather than simply adding features.
- [ ] Exercise:
  - Goal: The point is to make improvement explicit. Naming the idioms you used is important because it converts vague polish into reusable judgement.
  - Pick an older exercise and improve it using everything you know now:
    - fewer clones,
    - cleaner errors,
    - better function signatures,
    - more tests,
    - better iterator use,
    - clearer modules.
- [ ] Done when:
  - You can name the specific Rust idioms that improved the code.

## Day 60 - Final review and next plan

- [ ] Focus: assessment.
  - Detail: The final assessment turns two months of practice into a personal map. The goal is to leave with an honest inventory of what is automatic, what is familiar, and what deserves the next round of practice.
- [ ] Exercise:
  - Goal: The point is to close the loop with evidence. Your retrospective and cheat sheet should become the starting point for a more personalized next month.
  - Write a 1-2 page retrospective in `notes/final.md`:
    - APIs I know cold.
    - APIs I recognize but still need docs for.
    - Topics that still feel shaky.
    - Crates I am comfortable adding to a project.
    - Five exercises I would repeat next month.
  - Build a personal cheat sheet of standard-library modules and favorite methods.
- [ ] Done when:
  - You have a concrete next 30-day Rust plan based on your actual weak spots.

---

# Weekly Review Template

Use this every 7 days.

```markdown
# Week N Review

## APIs I used repeatedly

-
-
-

## APIs I looked up more than once

-
-
-

## Borrowing/ownership friction

-
-
-

## Error-handling notes

-
-
-

## Things to revisit

-
-
-

## One refactor I made

-

## One thing I can now explain clearly

-
```

---

# Suggested Mini-Project Difficulty Rules

Keep the projects small enough that you finish. The point is not to build impressive tools. The point is to repeatedly combine Rust's standard library pieces.

A mini-project is the right size if:

- core logic can be written in 100-250 lines,
- tests can cover the important behavior,
- the CLI can be crude,
- you finish or mostly finish in a weekend session,
- you can refactor it later.

Avoid:

- GUIs,
- full web apps,
- elaborate async networking,
- complex parsers,
- perfect architecture,
- spending the whole session fighting setup.

---

# Personal Rules of Thumb to Develop

Fill these in as you learn.

- Use `&str` when:
- Use `String` when:
- Use `&[T]` when:
- Use `Vec<T>` when:
- Use `HashMap` when:
- Use `BTreeMap` when:
- Use `Result<T, E>` when:
- Use `Option<T>` when:
- Use `thiserror` when:
- Use `anyhow` when:
- Use `clap` when:
- Use `serde` when:
- Use `regex` when:
- Use `itertools` when:
- Use threads when:
- Use Tokio when:

---

# Final Expected Outcome

After 60 days, you should not know the entire standard library. That is not the goal.

You should be able to:

- recognize the major modules in `std`,
- know where to look for details,
- choose common containers confidently,
- use iterators without fear,
- handle paths and files idiomatically,
- write useful errors,
- structure small Rust projects cleanly,
- use tests as a design tool,
- understand basic threading primitives,
- know why async Rust usually means choosing a runtime like Tokio,
- distinguish what Rust gives you in `std` from what the ecosystem commonly provides.

That is the useful working knowledge that compounds.
