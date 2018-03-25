extern crate mazegame;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 8 && args.len() != 3 {
        println!("usage: mazegame <width> <height> <num_rooms> <room_min_w> <room_min_h> <room_max_w> <room_max_h>");
        println!("       mazegame <width> <height>");
        std::process::exit(1);
    }

    if args.len() == 3 {
        mazegame::run((args[1].parse().unwrap(), args[2].parse().unwrap(), 0, 0, 0, 0, 0));
    } else {
        mazegame::run((args[1].parse().unwrap(), 
                       args[2].parse().unwrap(),
                       args[3].parse().unwrap(),
                       args[4].parse().unwrap(),
                       args[5].parse().unwrap(),
                       args[6].parse().unwrap(),
                       args[7].parse().unwrap()
                      ));                   
    }
}