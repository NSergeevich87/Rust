/*
This example code counts the frequency of each number in the vector.
 */
use std::collections::HashMap;

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
    let result = logic(numbers);
    //print the results in a human readable format that explains what the result is.
    println!(
        "The frequency of each number in the vector is: {:?}",
        result
    );
}

// use clap::Parser;
// use rust_sequences_and_maps::create_fruit_salad;

// #[derive(Parser)]
// #[clap(
//     version = "1.0",
//     author = "Nikita <niknikitaser@gmail.com>",
//     about = "Number of fruits to include in the salad"
// )]
// struct Opts {
//     #[clap(short, long)]
//     number: usize,
// }

// fn main() {
//     let opts: Opts = Opts::parse();

//     // Get the number of fruits the user requested
//     let num_fruits = opts.number;

//     // Create the fruit salad
//     create_fruit_salad(num_fruits);

//     // Print the fruit salad in human readable format with a count of fruits used
//     println!(
//         "Created Fruit salad with {} fruits: {:?}",
//         num_fruits,
//         create_fruit_salad(num_fruits)
//     );
// }

// create main
// fn main() {
//     // create a vector of numbers
//     let mut numbers = vec![1, 2, 3, 4, 5];
//     // add the numbers to a vector numbers
//     numbers.push(6);
//     // print the numbers
//     println!("{:?}", numbers);

//     // create deque of names
//     let mut names = std::collections::VecDeque::new();
//     // add names to the deque
//     names.push_back("Alice");
//     names.push_front("Bob");
//     // print the names
//     println!("{:?}", names);

//     // create a linked list of numbers
//     let mut numbers = std::collections::LinkedList::new();
//     // add numbers to the linked list
//     numbers.push_back(1);
//     numbers.push_back(2);
//     numbers.push_front(0);
//     // print the numbers
//     println!("{:?}", numbers);

//     // create a hash map of names
//     let mut names = std::collections::HashMap::new();
//     // add names to the hash map
//     names.insert("Alice", 30);
//     names.insert("Bob", 25);
//     names.insert("Nikita", 36);
//     // print the names
//     println!("{:?}", names);
// }

/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue.
*/

// use rand::seq::SliceRandom; // rand is a random number generation library in Rust
// use rand::thread_rng;
// use std::collections::LinkedList;
// use std::collections::VecDeque;

// fn main() {
//     let mut fruit: VecDeque<&str> = VecDeque::new();
//     fruit.push_back("Arbutus");
//     fruit.push_back("Loquat");
//     fruit.push_back("Strawberry Tree Berry");

//     // Scramble (shuffle) the fruit
//     let mut rng = thread_rng();
//     let mut fruit: Vec<_> = fruit.into_iter().collect();
//     fruit.shuffle(&mut rng);

//     // Convert it back to VecDeque
//     let mut fruit: VecDeque<_> = fruit.into_iter().collect();

//     // Add fruits to the both ends of the queue after shuffling
//     fruit.push_front("Pomegranate");
//     fruit.push_back("Fig");
//     fruit.push_back("Cherry");

//     // ask to user to add a fruit to the front or to the back of the queue
//     println!("For add fruit to the front, type 'f' and for add fruit to the back, type 'b'");
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).unwrap();
//     let input = input.trim();

//     let mut new_fruit = String::new();
//     if input == "f" {
//         println!("Enter the fruit to add to the front:");
//         std::io::stdin().read_line(&mut new_fruit).unwrap();
//         let input = new_fruit.trim(); // Create a new String from the trimmed input
//         fruit.push_front(input);
//     } else if input == "b" {
//         println!("Enter the fruit to add to the back:");
//         std::io::stdin().read_line(&mut new_fruit).unwrap();
//         let input = new_fruit.trim(); // Create a new String from the trimmed input
//         fruit.push_back(input);
//     }

//     let fruit: Vec<_> = fruit.into_iter().collect();
//     let random_fruit = fruit.choose(&mut rng).unwrap();
//     println!("Random fruit: {}", random_fruit);

//     // Print out the fruit salad
//     println!("Fruit Salad:");
//     for (i, item) in fruit.iter().enumerate() {
//         if i != fruit.len() - 1 {
//             print!("{}, ", item);
//         } else {
//             println!("{}", item);
//         }
//     }

//     let mut fruit: VecDeque<_> = fruit.into_iter().collect();

//     // ask user to remove a fruit from the front or back of the queue
//     println!(
//         "For remove fruit from the front, type 'f' and for remove fruit from the back, type 'b'"
//     );
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).unwrap();
//     let input = input.trim();
//     if input == "f" {
//         // show the fruit that will be removed
//         println!(
//             "Removing the fruit from the front: {}",
//             fruit.front().unwrap()
//         );
//         fruit.pop_front();
//     } else if input == "b" {
//         // show the fruit that will be removed
//         println!(
//             "Removing the fruit from the back: {}",
//             fruit.back().unwrap()
//         );
//         fruit.pop_back();
//     }

//     // Print out the fruit salad
//     println!("Fruit Salad:");
//     for (i, item) in fruit.iter().enumerate() {
//         if i != fruit.len() - 1 {
//             print!("{}, ", item);
//         } else {
//             println!("{}", item);
//         }
//     }

//     let mut fruit: LinkedList<_> = fruit.into_iter().collect();
//     // ask user to add a fruit to any position of the queue
//     println!("Enter the fruit to add to any position:");
//     let mut new_fruit = String::new();
//     std::io::stdin().read_line(&mut new_fruit).unwrap();
//     let input = new_fruit.trim(); // Create a new String from the trimmed input
//     println!("Enter the position to add the fruit:");
//     let mut position = String::new();
//     std::io::stdin().read_line(&mut position).unwrap();
//     let position: usize = position.trim().parse().unwrap();
//     let mut index = 0;
//     for item in fruit.iter_mut() {
//         if index == position {
//             *item = input;
//             break;
//         }
//         index += 1;
//     }

//     // Print out the fruit salad
//     println!("Fruit Salad:");
//     for (i, item) in fruit.iter().enumerate() {
//         if i != fruit.len() - 1 {
//             print!("{}, ", item);
//         } else {
//             println!("{}", item);
//         }
//     }
// }
