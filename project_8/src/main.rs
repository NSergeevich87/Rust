fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 0);
    println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8] 

    add_value_into_vector(&mut v, 666);

    let mut other_vec = vec![9, 10, 3, 45, 6];
    add_vec_into_vec(&mut v, &mut other_vec);
}

fn add_value_into_vector(vec: &mut Vec<i32>, value: i32) {
    vec.insert(0, value);
    vec.push(value);
    print!("{:?}", vec);
}

fn add_vec_into_vec(vec: &mut Vec<i32>, other_vec: &mut Vec<i32>) {
    //vec.extend(other_vec);
    vec.append(other_vec);
    print!("\n{:?}", vec);
}