fn numbers_sum(numbers:&[i32]) {
    let mut sum = 0;
    
    for number in numbers {
        sum += number;
    }

    println!("The sum of the numbers is: {}", sum);

    if sum % 2 == 0 {
        println!("The sum of the numbers is even");
    } else {
        println!("The sum of the numbers is odd");
    }
}

fn split_string(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(field);

    result.expect("Problem with string!").to_string()
}

fn sum_nums(num: &[i32]) -> i32 {
    let mut sum = 0;
    for n in num {
        sum += n;
    }
    sum
}

fn avg_nums(num: &[i32]) -> f64 {
    let sum = sum_nums(num) as f64;
    let len = num.len() as f64;
    sum / len
}

fn panic_function(nums: Vec<i32>) {
    for item in nums {
        if item < 0 {
            panic!("Negative number found: {}", item);
        }

        println!("Item: {}", item);
    }
}
fn main() {
    let numbers = [12, 23, 30, 40, 50];
    numbers_sum(&numbers);

    let chunk = split_string("hello,world".to_string(), ',', 1);
    println!("Split string: {}", chunk);

    let num = [1, 2, 3, 4, 5];
    let sum = sum_nums(&num);
    println!("Sum of numbers: {}", sum);

    
    let mut size = String::new();
    // read the size from the user
    println!("Enter the size of the array: ");
    // read line and unwrap the result
    std::io::stdin().read_line(&mut size).unwrap();
    let size: i32 = size.trim().parse().unwrap();
    // create an array of the size
    let mut numbers = vec![0; size as usize];

    for i in 0..size {
        println!("Enter a number {}: ", i + 1);
        // add a number to the numbers array
        let mut number = String::new();
        std::io::stdin().read_line(&mut number).unwrap();
        let number = number.trim().parse().unwrap();
        numbers[i as usize] = number;
    }

    let sum = sum_nums(&numbers);
    println!("Sum of numbers: {}", sum);

    let avg = avg_nums(&numbers);
    println!("Average of numbers: {}", avg);

    let numbers = vec![1, 2, 3, 4, 5, 1, 6, 7, 8, 9];
    panic_function(numbers);
}
