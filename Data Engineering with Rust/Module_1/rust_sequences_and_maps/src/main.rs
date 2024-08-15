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

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::VecDeque;

fn main() {
    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to VecDeque
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the queue after shuffling
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    // ask to user to add a fruit to the front or to the back of the queue
    println!("For add fruit to the front, type 'f' and for add fruit to the back, type 'b'");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let mut new_fruit = String::new();
    if input == "f" {
        println!("Enter the fruit to add to the front:");
        std::io::stdin().read_line(&mut new_fruit).unwrap();
        let input = new_fruit.trim(); // Create a new String from the trimmed input
        fruit.push_front(input);
    } else if input == "b" {
        println!("Enter the fruit to add to the back:");
        std::io::stdin().read_line(&mut new_fruit).unwrap();
        let input = new_fruit.trim(); // Create a new String from the trimmed input
        fruit.push_back(input);
    }

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
