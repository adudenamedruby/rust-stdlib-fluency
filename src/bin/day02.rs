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
