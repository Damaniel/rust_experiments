extern crate rand;

pub mod constants;
pub mod maze;

pub use maze::Maze;

fn main() {
    let mut m = Maze::new(10, 10);
    m.generate().unwrap();
    m.print();
}