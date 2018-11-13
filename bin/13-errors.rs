//
// EPITECH PROJECT, 2018
// rust-talk
// File description:
// 13-errors
//

use std::fs::File;
use std::io::Read;

// Example of using the Option and Result types to handle errors.

// Option<T>:    Some(T) or None
// Result<T, E>: Ok(T)   or Err(E)

fn main() {
    let v = vec![1, 2, 3];

    let elem = v.get(2); // elem: Option<&i32>

    match elem {
        Some(n) => println!("{}", n * 2), // n: &i32
        None => {}
    };

    let elem = v.get(2).map(|n| n * 2); // or .map(double);

    if let Some(n) = elem {
        println!("{}", n);
    }

    // content: Result<String, io::Error>
    let content = File::open("test").and_then(|mut file| {
        let mut content = String::new();
        file.read_to_string(&mut content).map(|_| content)
    });

    match content {
        Ok(content) => println!("Content: {}", content),
        Err(err) => println!("Error: {}", err),
    };
}
