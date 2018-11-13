//
// EPITECH PROJECT, 2018
// rust-talk
// File description:
// 8-condition_expr
//

// Simple example of a condition used as an expression.

fn main() {
    let x = 2;
    let y = 3;

    let message = if x == y {
        "equal to"
    } else if x < y {
        "less than"
    } else {
        "greater than"
    };

    println!("x is {} y", message);
}
