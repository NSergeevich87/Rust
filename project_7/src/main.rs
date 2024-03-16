use std::vec;

fn ownership() {
    let vec_nums = vec![1, 2, 3, 4, 5];
    let slice = &vec_nums[..];

    println!("slice = {:?}", slice); 
}

fn modifiable() {
    let mut vec_nums = vec![1, 2, 3, 4, 5];
    let slice = &mut vec_nums[..];

    slice[0] = 100;
    println!("slice = {:?}", slice); 
}

fn get_item(index: usize) {
    let vec_nums = vec![12, 3, 4, 56, 5];
    let item: i32;
    match vec_nums.is_empty() {
        true => panic!("Vector is empty"),
        false => item = vec_nums[index],
    }

    println!("item = {}", item);
}

fn sum_vector(vec_nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in vec_nums {
        sum += num;
    }
    sum
}

fn main() {
    ownership();
    modifiable();

    let vec_nums = vec![1, 2, 3, 4, 5];
    let third_value = vec_nums[2];
    println!("third_value = {}", third_value);
    let last_value = vec_nums.last().unwrap();
    println!("last_value = {}", last_value);

    match vec_nums.first() {
        Some(first) => println!("first_value = {}", first),
        None => println!("No first value"),
    }

    get_item(3);

    let sum = sum_vector(vec_nums);
    println!("sum = {}", sum);
}