//
// EPITECH PROJECT, 2018
// rust-talk
// File description:
// 5-references
//

// Here is how to use borrows in Rust.

#[derive(Debug, Clone, PartialEq, Eq)]
struct Vec2u {
    x: u32,
    y: u32,
}

impl Vec2u {
    fn new(x: u32, y: u32) -> Vec2u {
        Vec2u { x, y }
    }
}

fn double_vec1(v: Vec2u) -> Vec2u {
    Vec2u::new(v.x * 2, v.y * 2)
}

fn double_vec2(v: &Vec2u) -> Vec2u {
    Vec2u::new(v.x * 2, v.y * 2)
}

fn double_vec3(v: &mut Vec2u) {
    v.x *= 2;
    v.y *= 2;
}

// ! NEVER EVER EVER DO THIS !
// ! THIS IS JUST TO SHOWCASE THE FACT THAT POINTERS EXISTS.
// ! BUT IT IS FOR C FFI COMPATIBILITY ONLY.
// ! EVERYTHING IN RUST IS DOABLE WITHOUT POINTERS
unsafe fn double_vec4(v: *mut Vec2u) {
    (*v).x *= 2;
    (*v).y *= 2;
}

fn main() {
    let x = Vec2u::new(10, 20);
    let y = double_vec1(x);

    // Can't print x: we moved it into double_vec1()
    // println!("Before: {:?}", x);
    println!("After: {:?}", y);

    println!("");

    let x = Vec2u::new(10, 20);
    let y = double_vec2(&x);

    // We can print x: it never left us, we just borrowed it to double_vec2()
    println!("Before: {:?}", x);
    println!("After: {:?}", y);

    println!("");

    let mut x = Vec2u::new(10, 20);
    double_vec3(&mut x);

    // We can print x: it never left us, we just mutably borrowed it to double_vec2()
    println!("Mutated: {:?}", x);

    let mut x = Vec2u::new(10, 20);

    // ! AGAIN: NEVER EVER EVER DO THIS
    unsafe { double_vec4(&mut x); };

    println!("Mutated: {:?}", x);

}