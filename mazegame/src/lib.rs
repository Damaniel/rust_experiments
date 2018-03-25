//! A basic game using SDL.  Move a character through a maze, trying to find the
//! exit as quickly as possible.

// External crates
extern crate rand;

// Included modules
pub mod constants;
pub mod maze;

// Uses
pub use maze::Maze;

pub fn run((w, h, rooms, min_w, min_h, max_w, max_h): (u32, u32, u32, u32, u32, u32, u32)) {
    let mut m = Maze::new(w, h);
    match (rooms, min_w, min_h, max_w, max_h) {
        (0, 0, 0, 0, 0) => m.generate_perfect().unwrap(),
        _ => m.generate((rooms, min_w, min_h, max_w, max_h)).unwrap()
    }

    m.print();
}
