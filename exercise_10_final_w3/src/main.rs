use std::io;

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

fn format_size(size: u64) -> String {
    let filesize = if size < 1024 {
        FileSize::Bytes(size)
    } else if size < 1024 * 1024 {
        FileSize::Kilobytes(size as f64 / 1024.0)
    } else if size < 1024 * 1024 * 1024 {
        FileSize::Megabytes(size as f64 / (1024.0 * 1024.0))
    } else {
        FileSize::Gigabytes(size as f64 / (1024.0 * 1024.0 * 1024.0))
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}

fn main() {
    println!("Enter the file size + (bytes, kb, mb or gb): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // divide the input into two parts number and string
    let input: Vec<&str> = input.split_whitespace().collect();

    let size = input[0].parse::<u64>().unwrap();
    let unit: &str = input[1];

    println!("Size {}", input[0]);
    println!("Unit {}", input[1]);

    println!("The file sizes are: {}", format_size(size));
}