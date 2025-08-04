use std::{io, slice::Windows};

enum Shape {
    Circle,
    Rectangle,
    Square,
    Triangle,
}

trait area {
    fn calculateArea(&self) -> f64;
}
struct Rectangle {
    width: f64,
    height: f64,
}
impl area for Rectangle {
    fn calculateArea(&self) -> f64 {
        self.width * self.height
    }
}
struct Circle {
    radius: f64,
}
impl area for Circle {
    fn calculateArea(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

struct Square {
    width: f64,
}
impl area for Square {
    fn calculateArea(&self) -> f64 {
        self.width * self.width
    }
}

struct Triangle {
    height: f64,
    base: f64,
}

impl area for Triangle {
    fn calculateArea(&self) -> f64 {
        (self.height + self.base) / 2.0
    }
}

fn read_f64() -> f64 {

    let mut userInput = String::new();
    io::stdin()
        .read_line(&mut userInput)
        .expect("Radius is required");
    if userInput.trim().is_empty() {
        panic!("Radius is required");
    }
    let data: f64 = match userInput.trim().parse() {
        Ok(input) => input,
        Err(_) => {
            panic!("Invalid Number");
        }
    };

return data;
}

fn main() {
    let mut userInput = String::new();
    println!("Enter a shape: circle,square,triangle,rectangle: ");
    io::stdin()
        .read_line(&mut userInput)
        .expect("User input is required");
    if userInput.trim().is_empty() {
        panic!("User input is required");
    }

    let shape = match userInput.trim().to_lowercase().as_str() {
        "circle" => Shape::Circle,
        "rectangle" => Shape::Rectangle,
        "square" => Shape::Square,
        "triangle" => Shape::Triangle,
        _ => {
            println!("Invalid Input");
            return;
        }
    };

    match shape {
        Shape::Circle => {
            println!("Enter radius: ");
            let radius = read_f64();
            let newCircle = Circle { radius: radius };
            println!("Calculated area: {}", newCircle.calculateArea());
        }

        Shape::Rectangle => {
            println!("Enter width: ");
            let width = read_f64();
            println!("Enter height: ");
            let height = read_f64();
            let newRect = Rectangle{
                width:width,
                height:height
            };
            println!("Calculated area: {}", newRect.calculateArea());
        }
        Shape::Square => {
            println!("Enter width: ");
            let width = read_f64();
            let newSquare = Square {
                width:width
            };
            println!("Calculated area: {}", newSquare.calculateArea());
        }
        Shape::Triangle => {
            println!("Enter height: ");
            let height = read_f64();
            println!("Enter base: ");
            let base = read_f64();
            let newTri = Triangle {
                height:height,
                base:base
            };
            println!("Calculated area: {}", newTri.calculateArea());
        }
        _=> println!("Invalid operation")
    }
}
