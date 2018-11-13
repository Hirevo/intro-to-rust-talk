//
// EPITECH PROJECT, 2018
// rust-talk
// File description:
// 2-types
//

// Here are some Rust types in action.

fn main() {
    // This is a '&str'
    let static_str = "ℵ ∑ ½ ౽ ⅋ ! 🚀 🚀";
    println!("&str:\t\t\t{:?}", static_str);

    // This is an allocated 'String'
    let owned_str = String::from("The 💻 says: 'Hello, world ! 🎉 🎉'");
    println!("String:\t\t\t{:?}", owned_str);

    // This is an 'i32' storing 12
    let x = 12;
    println!("i32:\t\t\t{:?}", x);

    // This is an 'u32' storing 53
    let y = 53u32;
    println!("u32:\t\t\t{:?}", y);

    // This is a tuple: (i32, i32)
    let tuple = (4, 2);
    println!("tuple:\t\t\t{:?}", tuple);

    // This is a static array: [i32;3]
    let arr = [1, 2, 3];
    println!("[i32;3]:\t\t{:?}", arr);

    // This is an owned dynamic array: Vec<i32>
    let vector = vec![4, 5, 6];
    println!("Vec<i32>:\t\t{:?}", vector);

    // This is an enum called Option<T> with Some(T) and None being its variants
    let opt1: Option<i32> = Some(5);
    let opt2: Option<i32> = None;
    println!("Option<i32>:\t\t{:?}\t{:?}", opt1, opt2);

    // This is an enum called Result<T, E> with Ok(T) and Err(E) being its variants
    let ret1: Result<i32, &str> = Ok(5);
    let ret2: Result<i32, &str> = Err("Pas d'idées");
    println!("Result<i32, &str>:\t{:?}\t{:?}", ret1, ret2);

    // '_' means 'drop the value'
    let _ = 42;
}
