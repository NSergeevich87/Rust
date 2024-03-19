use std::io::{BufReader, BufRead};

/// this function reads a line from stdin and returns it as a string
/// It will panic if it fails to read a line with a message "Failed to read line"
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read line");
    line.trim().to_string()
}