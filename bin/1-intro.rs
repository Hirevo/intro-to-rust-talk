//
// EPITECH PROJECT, 2018
// rust-talk
// File description:
// 1-intro
//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec3u {
    x: u32,
    y: u32,
    z: u32,
}

impl Vec3u {
    fn new(x: u32, y: u32, z: u32) -> Vec3u {
        Vec3u { x, y, z }
    }

    fn double(&mut self) {
        self.x *= 2;
        self.y *= 2;
        self.z *= 2;
    }
}

fn main() {
    let mut v = Vec3u::new(4, 2, 6);
    println!("{:?}", v);

    v.double();

    println!("{:?}", v);
}
