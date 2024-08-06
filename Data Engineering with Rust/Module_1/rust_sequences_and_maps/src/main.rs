// create main
fn main() {
    // create a vector of numbers
    let mut numbers = vec![1, 2, 3, 4, 5];
    // add the numbers to a vector numbers
    numbers.push(6);
    // print the numbers
    println!("{:?}", numbers);

    // create deque of names
    let mut names = std::collections::VecDeque::new();
    // add names to the deque
    names.push_back("Alice");
    names.push_front("Bob");
    // print the names
    println!("{:?}", names);

    // create a linked list of numbers
    let mut numbers = std::collections::LinkedList::new();
    // add numbers to the linked list
    numbers.push_back(1);
    numbers.push_back(2);
    numbers.push_front(0);
    // print the numbers
    println!("{:?}", numbers);

    // create a hash map of names
    let mut names = std::collections::HashMap::new();
    // add names to the hash map
    names.insert("Alice", 30);
    names.insert("Bob", 25);
    names.insert("Nikita", 36);
    // print the names
    println!("{:?}", names);
}
