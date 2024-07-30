// Example showing basic Rust syntax and type safety
// fn main() {
//     let mut x = 5; // Mutable binding 
//     println!("x is {}", x);
//     x = "foo"; // Type error 
// }

// Basic Rayon parallel map example  
use rayon::prelude::*;

fn main() {
  let vals = vec![1, 2, 3];
  let squared = vals.par_iter() // Rayon parallel iterator 
    .map(|x| x * x)  
    .collect::<Vec<_>>(); 
  
  println!("{:?}", squared);   
}