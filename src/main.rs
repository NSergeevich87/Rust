// this program ask a user to enter a name and then print it out
use std::io;

fn main() {
    println!("Please enter your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    println!("Hello, {}", name);
}