//
// EPITECH PROJECT, 2018
// rust-talk
// File description:
// 14-traits
//

// This shows the use of traits to unify and define a common capability for multiple types.

use std::ops::Add;

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

impl Add for Vec2u {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2u {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let pos = Vec2u::new(10, 40);
    let dir = Vec2u::new(3, 2);

    println!("{:?}", pos + dir);
}
