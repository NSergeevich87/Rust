use std::io::BufRead;
use std::io::Write;
use std::fs::File;

fn main() {
    let mut data = String::new();
    // ask the user write something to data
    println!("Please write something to the file: ");
    std::io::stdin().read_line(&mut data).unwrap();

    // Add the ability to pass in any file path as an argument to the program
    let args: Vec<String> = std::env::args().collect();
    let file_path = &args[0];
    print!("My file path is: {}", file_path);

    // create a new file in the current directory
    let file = std::fs::File::create("new_file.txt");
    match file {
        Ok(mut file) => {
            println!("\nFile created successfully");
            file.write_all(data.as_bytes()).expect("Error writing to the file");
        }
        Err(error) => {
            println!("Failed to create file: {}", error);
        }
    };

    // open a file in the current directory
    let file = File::open("new_file.txt");
    let file = match file {
        // if Ok write the data to the file
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };

    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => println!("Error reading line: {}", error),
        }
    };
}
