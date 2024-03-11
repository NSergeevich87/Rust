fn main() {
    let permit = true;
    if permit {
        println!("You have a permit to enter the building.");
    } else {
        println!("You do not have a permit to enter the building.");
    }

    let age = 36;
    if age > 50 {
        println!("You are a senior citizen.");
    } else if age > 35 {
        println!("You are middle-aged.");
    } else {
        println!("You are young.");
    }

    let mut number = 3;
    number += 1;
    if number == 10 {
        println!("The number is 10.");
    } else {
        println!("The number is not 10.");
    }
}
