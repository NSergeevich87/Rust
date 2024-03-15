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
    //let index_val = 3;
    let vec_nums = vec![1, 2, 3, 4, 5];
    let item = vec_nums.get(index).unwrap();
    println!("item with index {} = {}", index, item);
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
}