// Write tests for a custom Rust collection type that leverages iterators and implements IntoIterator.

use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug)]
struct MyUnique {
    data: HashSet<i32>,
}

impl MyUnique {
    fn new() -> Self {
        MyUnique {
            data: HashSet::new(),
        }
    }

    fn insert(&mut self, value: i32) -> bool {
        self.data.insert(value)
    }

    fn iter(&self) -> std::collections::hash_set::Iter<i32> {
        self.data.iter()
    }
}

impl IntoIterator for MyUnique {
    type Item = i32;
    type IntoIter = std::collections::hash_set::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl FromIterator<i32> for MyUnique {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        let mut unique = MyUnique::new();
        for i in iter {
            unique.insert(i);
        }
        unique
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_MyUnique() {
        let mut my_unique = MyUnique::new();
        assert_eq!(my_unique.insert(1), true);
        assert_eq!(my_unique.insert(2), true);
        assert_eq!(my_unique.insert(1), false);
        assert_eq!(my_unique.insert(2), false);

        let mut iter = my_unique.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);

        let my_unique: MyUnique = my_unique.into_iter().collect();
        assert_eq!(my_unique.data.len(), 2);
    }
}

fn main() {
    let mut my_unique = MyUnique::new();
    my_unique.insert(1);
    my_unique.insert(2);
    my_unique.insert(1);
    my_unique.insert(2);

    for i in my_unique.iter() {
        println!("{}", i);
    }

    let my_unique: MyUnique = my_unique.into_iter().collect();
    println!("{:?}", my_unique);
}

// Create a Rust struct that implements traits like PartialEq and Hash to be used as keys in a HashMap.

// use std::collections::HashMap;

// #[derive(PartialEq, Eq, Hash, Debug)]
// struct Person {
//     name: String,
//     age: u8,
// }

// fn main() {
//     let mut people = HashMap::new();
//     let john = Person {
//         name: "John".to_string(),
//         age: 25,
//     };
//     let jane = Person {
//         name: "Jane".to_string(),
//         age: 20,
//     };

//     // compare two Person instances
//     if john == jane {
//         println!("John and Jane are the same person.");
//     } else {
//         println!("John and Jane are different people.");
//     }

//     people.insert(john, "some data about John");
//     people.insert(jane, "some data about Jane");
//     println!("{:?}", people);
// }

// Build a priority queue with BinaryHeap and implement heap sort on a vector of integers.

// use std::collections::BinaryHeap;

// fn heap_sort(mut v: Vec<i32>) -> Vec<i32> {
//     let mut heap = BinaryHeap::from(v);
//     let mut sorted = Vec::new();
//     while let Some(i) = heap.pop() {
//         sorted.push(i);
//     }
//     sorted
// }

// fn main() {
//     let v = vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
//     let sorted = heap_sort(v);
//     println!("{:?}", sorted);
// }

// Benchmark insertion and lookup performance on Vec, LinkedList, HashMap, and BTreeMap with 10k elements.

// use std::collections::{BTreeMap, HashMap, LinkedList};
// use std::time::{Duration, Instant};

// fn main() {
//     let n = 10_000;
//     let mut vec = Vec::new();
//     let mut linked_list = LinkedList::new();
//     let mut hash_map = HashMap::new();
//     let mut btree_map = BTreeMap::new();

//     for i in 0..n {
//         vec.push(i);
//         linked_list.push_back(i);
//         hash_map.insert(i, i);
//         btree_map.insert(i, i);
//     }

//     let start = Instant::now();
//     for i in 0..n {
//         assert_eq!(vec[i], i);
//     }
//     let vec_time = start.elapsed();

//     let start = Instant::now();
//     for i in 0..n {
//         assert_eq!(linked_list.pop_front().unwrap(), i);
//     }
//     let linked_list_time = start.elapsed();

//     let start = Instant::now();
//     for i in 0..n {
//         assert_eq!(*hash_map.get(&i).unwrap(), i);
//     }
//     let hash_map_time = start.elapsed();

//     let start = Instant::now();
//     for i in 0..n {
//         assert_eq!(*btree_map.get(&i).unwrap(), i);
//     }
//     let btree_map_time = start.elapsed();

//     println!("Vec: {:?}", vec_time);
//     println!("LinkedList: {:?}", linked_list_time);
//     println!("HashMap: {:?}", hash_map_time);
//     println!("BTreeMap: {:?}", btree_map_time);
// }
