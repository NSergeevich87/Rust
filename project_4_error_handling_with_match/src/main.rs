use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::io::Write;

fn main() {
    let data = "initial contents";
    // write into a file
    let file = File::create("file.txt");
    match file {
        Ok(mut file) => {
            println!("File created successfully: {:?}", file);
            file.write_all(data.as_bytes()).expect("Error writing to the file");
        }
        Err(error) => {
            panic!("Error creating the file: {}", error);
        }
    }
    // write into a file 
    

    let file = File::open("file.txt");
    match file {
        Ok(file) => {
            println!("File opened successfully: {:?}", file);

            let reader = BufReader::new(&file);
            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error);
            }
            //  handle a different type of error, such as a permission error 
            std::io::ErrorKind::PermissionDenied => {
                panic!("Permission denied: {}", error);
            }
            _ => {
                panic!("Error opening the file: {}", error);
            }
        }
    }

    
}
