use std::io;
fn main() {
    let mut x = 0;
    loop {
        x += 1;
        if x == 10 {
            break;
        }
        println!("x = {}", x);
    }

    let maybe_number: Option<Option<()>> = Some(None);
    //let maybe_number: Option<Option<()>> = Some(None);
    if let Some(number) = maybe_number {
        println!("number = {:?}", number);
    } else {
        println!("No number");
    }

    let mut y = 0;
    while y < 10 {
        println!("y = {}", y);
        y += 1;
    }

    let mut input = String::new();
    while input.trim() != "quit" {
        input.clear();
        println!("Type 'quit' to quit");
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        println!("You typed: {}", input);
    }

    print!("Goodbye!");

    for i in 1..10 {
        println!("i = {}", i);
    }

    for i in (1..=10).rev() {
        println!("i = {}", i);
    }

    let nubers = vec![77, 2, 3, 4, 5];
    for n in nubers {
        println!("number: {}", n);
    }

    for i in 1..=10 {
        if i % 2 == 0 {
            continue;
        }
        println!("i = {}", i);
        if i == 7 {
            break;
        }
    }

    let string1 = "Hello";
    let string2 = "Goodbye";
    let string3 = "Hell";

    for i in 1..=3 {
        if i == 1 {
            print_string(string1);
        } else if i == 2 {
            print_string(string2);
        } else {
            print_string(string3);
        }
    }

    println!("Enter a string: ");
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read input");

    match input.trim().to_lowercase().as_str() {
        "nikita" => println!("Hello, Nikita!"),
        "ksyusha" => println!("Hello, Ksyusha!"),
        _ => println!("Hello, stranger!"),
    }
}

fn print_string(s: &str) {
    match s.trim().to_lowercase().as_str() {
        "hello" => println!("World"),
        "goodbye" => println!("La Vida Loca"),
        _ => println!("Not Hello"),
    }    
}
