//
// EPITECH PROJECT, 2018
// rust-talk
// File description:
// 9-loops
//

// Simple example of loops and break in Rust.

fn main() {
    println!("Before outer loop");
    loop {
        let mut i = 0;
        println!("Before while");
        while i < 5 {
            println!("Inside while {}", i);
            i += 1;
            if i == 3 {
                break;
            }
        }
        println!("After while");
    }
    // println!("After outer loop");
}
