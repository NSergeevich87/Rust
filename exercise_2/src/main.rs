#[derive(Debug, PartialEq)] 
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    //mileage: u32, 
    // todo!("Move `mileage: u32` field into `age` field - a tuple with two fields: an `Age` enum, u32");
    age: (Age, u32),
}

#[derive(Debug, PartialEq)] // Add PartialEq
enum Age { New, Used }
// Declare enum for Car transmission type
#[derive(Debug, PartialEq)] // Add PartialEq
enum Transmission { Manual, SemiAuto, Automatic }

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Create a tuple for the car quality with the Age ("New" or "Used") and mileage
// Return a tuple with the arrow `->` syntax
fn car_quality (miles: u32) -> (Age, u32) {

    // Declare and initialize the return tuple value
    // For a new car, set the miles to 0
    let quality: (Age, u32) = if miles == 0 {
        (Age::New, miles)
    } else {
        (Age::Used, miles)
    };

    // Return the completed tuple to the caller
    // todo!("Return the tuple");
    quality
}

// Build a new "Car" using the values of four input arguments
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has a hard top roof)
// - miles (u32)
// Call the car_quality(miles) function to get the car age
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    // Create a new "Car" instance as requested
    // - Bind first three fields to values of input arguments
    // - "age" field calls "car_quality" function with "miles" input argument 
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: (car_quality(miles)), // todo!("Replace `mileage: miles` with `age` tuple field, call `car_quality()` with `miles` as input argument");
    }
}
fn main() {
    // Create car color array
    let colors = ["Blue", "Green", "Red", "Silver"]; //todo!("Set the enum values: 0 = Blue, 1 = Green, 2 = Red, 3 = Silver");

    // Declare the car type and initial values
    let mut car: Car;

    // Declare `engine` as a `Transmission` enum, initialize to `Manual`
    let mut engine = Transmission::Manual;

    // Order 3 cars, one car for each type of transmission

    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #2: Used, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    println!("Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #3: Used, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    println!("Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
}
