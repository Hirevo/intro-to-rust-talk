//
// EPITECH PROJECT, 2018
// rust-talk
// File description:
// 11-pattern_matching
//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec2u {
    x: u32,
    y: u32,
}

impl Vec2u {
    fn new(x: u32, y: u32) -> Vec2u {
        Vec2u { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Event {
    WindowClosed,
    WindowResized,
    KeyPressed(char),
    MouseMoved { pos: Vec2u, from: Vec2u },
}

fn main() {
    let evts = [
        Event::KeyPressed('c'),
        Event::MouseMoved {
            pos: Vec2u::new(10, 53),
            from: Vec2u::new(20, 42),
        },
        Event::KeyPressed('A'),
        Event::WindowResized,
        Event::WindowClosed,
    ];

    for evt in evts.iter() {
        match evt {
            Event::WindowClosed => println!("The window has been closed."),
            Event::KeyPressed('c') => println!("Oh it's the 'c' key."),
            Event::KeyPressed(key) => println!("The '{}' key has been pressed.", key),
            Event::MouseMoved { pos, .. } => println!("The mouse has moved to: {:?}.", pos),
            _ => println!("Something else happened."),
        };
    }
}
