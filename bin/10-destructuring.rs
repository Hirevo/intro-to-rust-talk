//
// EPITECH PROJECT, 2018
// rust-talk
// File description:
// 10-destructuring
//

// Example of destructuring values in Rust.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec2u {
    x: u32,
    y: u32,
}

impl Vec2u {
    fn new(x: u32, y: u32) -> Vec2u {
        Vec2u { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    Circle(u32),
    Rectangle(u32, u32),
    Square(u32),
}

fn main() {
    let v = (1, 2);
    let (x, y) = v;
    println!("x: {}, y: {}", x, y);

    let v = Vec2u::new(10, 42);
    let Vec2u { x, y } = v;
    println!("x: {}, y: {}", x, y);

    let shape = Shape::Circle(10);

    let Shape::Circle(radius) = shape;

    println!("radius: {}", radius);
}
