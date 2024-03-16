enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64),
}

fn largest_shape(vector_shapes: Vec<Shape>) -> f64 {
    let mut largest_area: f64 = 0.0;
    let mut temp: f64 = 0.0;
    for shape in vector_shapes {
        temp = match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base, height) => 0.5 * base * height,
        };

        if temp > largest_area {
            largest_area = temp;
        }
    }

    

    largest_area
}
fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0), Shape::Triangle(4.0, 3.0)];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base, height) => 0.5 * base * height,
        })
        .sum();

    println!("Total area: {} sq. units", total_area);
    
    let largest_area = largest_shape(shapes);
    println!("Largest area: {} sq. units", largest_area);
}