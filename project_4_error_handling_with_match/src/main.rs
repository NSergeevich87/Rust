use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

fn main() {
    let file = File::open("no_existing_file.txt");
    match file {
        Ok(file) => {
            println!("File opened successfully: {:?}", file);
            let reader = BufReader::new(file);
            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error);
            }
            _ => {
                panic!("Error opening the file: {}", error);
            }
        }
    }
}
