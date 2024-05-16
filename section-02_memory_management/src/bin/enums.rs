#[allow(unused)]
#[derive(Debug)]
/* enum Direction {
    North,
    East,
    West,
    South,
}

fn main() {
    let myDirection = Direction::North;
    println!("{:?}", myDirection);
} */

enum Shape {
    Square(f64),
    Rectangle(f64, f64),
    Circle(f64),
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * 2.00,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side_length) => side_length * side_length,
    }
}

fn main() {
    let circle = Shape::Circle(7_f64);
    let area_circle = calculate_area(circle);
    let rect = calculate_area(Shape::Rectangle(50_f64, 20_f64));
    println!("circle area : {},\nrectangle area : {}", area_circle, rect);
}
