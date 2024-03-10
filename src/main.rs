// this program ask a user to enter a name and then print it out
use std::io;

fn main() {
    println!("Please enter your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    println!("Hello, {}", name);
}

// this function get a number from the user and return it
fn get_number() -> i32 {
    let mut number = String::new();
    io::stdin().read_line(&mut number).unwrap();
    let number: i32 = number.trim().parse().unwrap();
    number
}