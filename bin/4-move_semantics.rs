//
// EPITECH PROJECT, 2018
// rust-talk
// File description:
// 4-move_semantics
//

// Move semantics and passing ownership.

fn do_stuff(string: String) {
    println!("{}", string);
}

fn main() {
    let string = String::from("Test");

    do_stuff(string);
    println!("{}", string);
}
