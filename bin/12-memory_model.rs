//
// EPITECH PROJECT, 2018
// rust-talk
// File description:
// 12-memory_model
//

// Simple example of how Rust achieves automatic memory management without garbage collection.
// (Hint: Static analysis FTW !)

use std::fs::File;
use std::io::{self, Read};

fn do_stuff(file: File) -> File {
    let _ = file.metadata();
    return file;
}

fn read_all_file(mut file: File) -> io::Result<String> {
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() -> io::Result<()> {
    let mut file = File::open("Cargo.toml")?;

    file = do_stuff(file);

    let content = read_all_file(file)?;

    println!("Content:");
    println!("{}", content);

    Ok(())
}
