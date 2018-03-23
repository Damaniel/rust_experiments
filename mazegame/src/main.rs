extern crate rand;

pub mod constants;
pub mod square;
pub mod maze;

pub use maze::Maze;

fn main() {
    let mut m = Maze::new(10, 10);
    m.generate();
    m.print();
}