// ## Day 2 - `Option`, `Result`, and basic flow
//
// - [x] Focus: idiomatic absence and failure.
//   - Detail: Rust uses types to make absence and failure explicit. The goal is to make `Option` and `Result` feel like normal control-flow tools rather than ceremony around simple operations.
// - [x] Read/inspect:
//   - What to look for: Pay attention to method names and signatures, especially whether a method consumes `self`, borrows, or returns a new wrapped value. Do not try to memorize every combinator; group them by purpose.
//   - `std::option::Option`
//   - `std::result::Result`
// - [ ] Exercise:
//   - Goal: The point is to practice representing uncertainty and failure in return types. Rewriting with combinators helps you learn the fluent style, while comparing with `match` keeps readability in view.
//   - Write `parse_port(input: &str) -> Result<u16, String>`.
//   - Write `first_non_empty_line(input: &str) -> Option<&str>`.
//   - Rewrite both without `match`, using combinators like `map`, `and_then`, `ok_or_else`, and `filter`.
// - [ ] Done when:
//   - You can explain when you prefer `match` versus combinators.
//   - You understand why `Option<&str>` is often better than returning an owned `String`.

fn parse_port(input: &str) -> Result<u16, String> {
    match input.parse::<u16>() {
        Ok(port) => Ok(port),
        Err(e) => Err(format!("Unable to parse \"{input}\": {e}")),
    }
}

fn first_non_empty_line(input: &str) -> Option<&str> {
    input.lines().find(|&line| !line.is_empty()) //.map(|v| v as _)
}

fn main() {
    let port = parse_port("naw gurl");
    println!("{:?}", port);

    let text = "\n\ni am first\n\n";
    let line = first_non_empty_line(text);
    println!("{:?}", line);
}
