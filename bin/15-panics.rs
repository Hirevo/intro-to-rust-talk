//
// EPITECH PROJECT, 2018
// rust-talk
// File description:
// 15-panics
//

// Panics: controlled unwindings of the callstack.
// Used only for unrecoverable situations.
// Panics' behavior is 100% well-defined, graceful and controlled.
// Can be handled via `std::panic::catch_unwind()`

use std::panic;

fn get_i32() -> Option<i32> {
    None
}

fn main() {
    let _ret = panic::catch_unwind(|| get_i32().unwrap()); // Panics if None

    println!("");
    match _ret {
        Ok(n) => println!("Worked with {}", n),
        Err(_) => println!("Oh shoot, it panicked. This code must be real bad !"),
    };
}
