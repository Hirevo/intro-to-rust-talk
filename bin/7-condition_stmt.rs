//
// EPITECH PROJECT, 2018
// rust-talk
// File description:
// 7-condition_stmt
//

// Simple example of a condition in Rust

use std::env;

fn main() {
    let args = env::args();
    let count = args.count();

    if count > 1 {
        println!("You've sent one or more arguments");
    } else if count == 1 {
        println!("You've sent no arguments");
    } else {
        println!("Is it even possible ?");
    }
}
