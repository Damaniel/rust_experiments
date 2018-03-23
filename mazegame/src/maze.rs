extern crate rand;

pub use constants;
pub use square::Square;
use rand::{Rng, thread_rng};

#[derive(Clone, Debug)]
pub struct Coord {
    x: u32,
    y: u32,
}

#[derive(Clone, Debug)]
pub struct Maze {
    rows: u32,
    cols: u32,
    pub sq: Vec<Square>,
}

impl Maze {
    pub fn new(rows: u32, cols: u32) -> Maze {
        Maze {
            rows: rows as u32,
            cols: cols as u32,
            sq: vec![Square::new(); (rows * cols) as usize],
        }
    }

    pub fn get_rows(&self) -> u32 {
        self.rows
    }

    pub fn get_cols(&self) -> u32 {
        self.cols
    }

    pub fn get_offset(&self, x: u32, y: u32) -> u32 {
        y * self.cols + x
    }

    pub fn carve(&mut self, x: u32, y: u32, dir: u32) -> bool {
        // If the maze index is invalid, just return 
        if y >= self.rows { return false; }
        if x >= self.cols { return false; }

        // The position of the square on the other side of the wall
        let mut dest_x = x;        
        let mut dest_y = y;
        let dest_dir;

        // Prevent breaking of walls that would lead out of the maze
        match dir {
            constants::DIR_NORTH => {
                if y == 0 { return false; }
                dest_y = dest_y - 1;
                dest_dir = constants::DIR_SOUTH;
            }
            constants::DIR_SOUTH => {
                if y == self.rows - 1 { return false; }
                dest_y = dest_y + 1;
                dest_dir = constants::DIR_NORTH;
            }
            constants::DIR_EAST => {
                if x == self.cols -1 { return false; }
                dest_x = dest_x + 1;
                dest_dir = constants::DIR_WEST;
            }
            constants::DIR_WEST => {
                if x == 0 { return false; }
                dest_x = dest_x - 1;
                dest_dir = constants::DIR_EAST;
            }
            _ => { return false; }
        }

        // Need two mutable references, but not at the same time.
        // Stick em in their own scopes.
        {
            let square = &mut self.sq[(y * self.cols + x) as usize];
            square.break_wall(dir);
        }
        {
            let square = &mut self.sq[(dest_y * self.cols + dest_x) as usize];
            square.break_wall(dest_dir);
        }

        return true;
    }

    pub fn generate(&mut self) {
        // For now:
        // - Call the algorithmic generator
        self.generate_growing_tree(0, 0);

        // Eventually:
        // - Generate some rooms
        // - Call the algorithmic generator
        // - Connect rooms to the maze
        // - Remove dead ends
    }

    // Needs to return a Result
    pub fn generate_growing_tree(&mut self, start_x: u32, start_y: u32) {
        let mut visited: Vec<Coord> = Vec::new();
        let mut cur_coord = Coord { x: start_x, y: start_y };

        // Handle the initial square
        let (result, dir) = self.pick_direction(cur_coord.x, cur_coord.y);
        if result == false { 
            return; 
        } else {
            self.carve(cur_coord.x, cur_coord.y, dir);
            visited.push(cur_coord.clone());
            match dir {
                constants::DIR_NORTH => cur_coord.y = cur_coord.y - 1,
                constants::DIR_SOUTH => cur_coord.y = cur_coord.y + 1,
                constants::DIR_EAST => cur_coord.x = cur_coord.x + 1,
                constants::DIR_WEST => cur_coord.x = cur_coord.x - 1,
                _ => panic!("Illegal direction in generate_growing_tree!"),
            }
        }

        // Handle all subsequent squares
        while visited.len() > 0 {
            let (result, dir) = self.pick_direction(cur_coord.x, cur_coord.y);
            // No directions available.  Pull a square from the stack.
            if result == false {
                let item = visited.pop();
                match item {
                    Some(i) => cur_coord = i,
                    None => panic!("Pop failed in generate_growing_tree!"),
                }
            } else {
                self.carve(cur_coord.x, cur_coord.y, dir);
                visited.push(cur_coord.clone());
                match dir {
                    constants::DIR_NORTH => cur_coord.y = cur_coord.y - 1,
                    constants::DIR_SOUTH => cur_coord.y = cur_coord.y + 1,
                    constants::DIR_EAST => cur_coord.x = cur_coord.x + 1,
                    constants::DIR_WEST => cur_coord.x = cur_coord.x - 1,
                    _ => panic!("Illegal direction in generate_growing_tree!"),
                }
            }
        }
    }

    pub fn pick_direction(&self, x: u32, y: u32) -> (bool, u32) {
        let mut directions: Vec<u32> = Vec::new();
        let mut rng = thread_rng();

        if y > 0 {
            let sq = &self.sq[((y-1) * self.cols + x) as usize];
            if sq.is_carved() == false {
                directions.push(constants::DIR_NORTH);
            }
        }
        if y < self.rows - 1 {
            let sq = &self.sq[((y+1) * self.cols + x) as usize];
            if sq.is_carved() == false {
                directions.push(constants::DIR_SOUTH);
            }
        }
        if x < self.cols - 1 {
            let sq = &self.sq[(y * self.cols + (x + 1)) as usize];
            if sq.is_carved() == false {
                directions.push(constants::DIR_EAST);
            }
        }
        if x > 0 {
            let sq = &self.sq[(y * self.cols + (x - 1)) as usize];
            if sq.is_carved() == false {
                directions.push(constants::DIR_WEST);
            }
        }
        
        // If no directions are found...
        if directions.len() == 0 {
            (false, 0)
        } else {
            let dir = rng.gen_range(0, directions.len());
            (true, directions[dir])
        }
    }

    pub fn print(&self) {
        // Print the first row
        print!("X");
        for _i in 0..(self.cols * 2) {
            print!("X");
        }
        println!("");
        for y in 0..self.rows {
            print!("X");
            for x in 0..self.cols {
                let sq = &self.sq[(y * self.cols + x) as usize];
                if sq.is_wall_present(constants::DIR_EAST) == true {
                    print!(" X");
                } else {
                    print!("  ");
                }
            }
            println!("");
            print!("X");
            for x in 0..self.cols {
                let sq = &self.sq[(y * self.cols + x) as usize];
                if sq.is_wall_present(constants::DIR_SOUTH) == true {
                    print!("XX");
                } else {
                    print!(" X");
                }
            }
            println!("");            
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_maze() {
        let maze = Maze::new(10, 10);
        for row in 0..maze.get_rows() {
            for col in 0..maze.get_cols() {
                assert_eq!(true, maze.sq[maze.get_offset(row, col) as usize].wall_present[constants::DIR_NORTH as usize]);
                assert_eq!(true, maze.sq[maze.get_offset(row, col) as usize].wall_present[constants::DIR_SOUTH as usize]);
                assert_eq!(true, maze.sq[maze.get_offset(row, col) as usize].wall_present[constants::DIR_EAST as usize]);
                assert_eq!(true, maze.sq[maze.get_offset(row, col) as usize].wall_present[constants::DIR_WEST as usize]);        
            }
        }
    }

    #[test] 
    fn test_carver() {
        let mut maze = Maze::new(10, 10);
        let mut result: bool;

        // Note: In each of the carve phases, when a carve is legal, there are
        // *two* squares that need to be checked:
        // - carving east : east wall of current square, west wall of square to the east
        // - carving west : west wall of current square, east wall of square to the west
        // - carving north: north wall of current square, south wall of square to the north
        // - carving south: south wall of current square, north wall of square to the south

        // Carve south from the top left corner.  This should work.
        result = maze.carve(0, 0, constants::DIR_SOUTH);
        assert_eq!([true, false, true, true], maze.sq[maze.get_offset(0, 0) as usize].wall_present);
        assert_eq!([false, true, true, true], maze.sq[maze.get_offset(0, 1) as usize].wall_present);
        assert_eq!(true, result);

        // Carve north from the top corner.  This *shouldn't* work (so the vec should be unchanged)
        result = maze.carve(0, 0, constants::DIR_NORTH);
        assert_eq!([true, false, true, true], maze.sq[maze.get_offset(0, 0) as usize].wall_present);
        assert_eq!(false, result);

        // Carve west from the top corner.  This *shouldn't* work.
        result = maze.carve(0, 0, constants::DIR_WEST);
        assert_eq!([true, false, true, true], maze.sq[maze.get_offset(0, 0) as usize].wall_present);
        assert_eq!(false, result);

        // Carve east from the top corner.  This should work.
        result = maze.carve(0, 0, constants::DIR_EAST);
        assert_eq!([true, false, false, true], maze.sq[maze.get_offset(0, 0) as usize].wall_present);
        assert_eq!([true, true, true, false], maze.sq[maze.get_offset(1, 0) as usize].wall_present);
        assert_eq!(true, result);

        // Carve north from the bottom right corner.  This should work.
        result = maze.carve(9, 9, constants::DIR_NORTH);
        assert_eq!([false, true, true, true], maze.sq[maze.get_offset(9, 9) as usize].wall_present);
        assert_eq!([true, false, true, true], maze.sq[maze.get_offset(9, 8) as usize].wall_present);
        assert_eq!(true, result);

        // Carve south from the bottom right corner.  This *shouldn't* work (so the vec should be unchanged)
        result = maze.carve(9, 9, constants::DIR_SOUTH);
        assert_eq!([false, true, true, true], maze.sq[maze.get_offset(9, 9) as usize].wall_present);
        assert_eq!(false, result);

        // Carve east from the bottom right corner.  This *shouldn't* work.
        result = maze.carve(9, 9, constants::DIR_EAST);
        assert_eq!([false, true, true, true], maze.sq[maze.get_offset(9, 9) as usize].wall_present);
        assert_eq!(false, result);

        // Carve west from the bottom right corner.  This should work.
        result = maze.carve(9, 9, constants::DIR_WEST);
        assert_eq!([false, true, true, false], maze.sq[maze.get_offset(9, 9) as usize].wall_present);
        assert_eq!([true, true, false, true] , maze.sq[maze.get_offset(8, 9) as usize].wall_present);                
        assert_eq!(true, result);

        // Carve in each direction from a central square.  All these should work.
        result = maze.carve(4, 5, constants::DIR_NORTH);
        assert_eq!([false, true, true, true], maze.sq[maze.get_offset(4, 5) as usize].wall_present);
        assert_eq!([true, false, true, true], maze.sq[maze.get_offset(4, 4) as usize].wall_present);        
        assert_eq!(true, result);
        result = maze.carve(4, 5, constants::DIR_SOUTH);
        assert_eq!([false, false, true, true], maze.sq[maze.get_offset(4, 5) as usize].wall_present);
        assert_eq!([false, true, true, true], maze.sq[maze.get_offset(4, 6) as usize].wall_present);        
        assert_eq!(true, result);
        result = maze.carve(4, 5, constants::DIR_EAST);
        assert_eq!([false, false, false, true], maze.sq[maze.get_offset(4, 5) as usize].wall_present);
        assert_eq!([true, true, true, false], maze.sq[maze.get_offset(5, 5) as usize].wall_present);        
        assert_eq!(true, result);
        result = maze.carve(4, 5, constants::DIR_WEST);
        assert_eq!([false, false, false, false], maze.sq[maze.get_offset(4, 5) as usize].wall_present);
        assert_eq!([true, true, false, true], maze.sq[maze.get_offset(3, 5) as usize].wall_present);        
        assert_eq!(true, result);

        // Try carving out of bounds.  This shouldn't work.
        result = maze.carve(100, 100, constants::DIR_NORTH);
        assert_eq!(false, result);

    }

    #[test]
    fn test_direction_picker() {
        let mut maze = Maze::new(10, 10);

        // Use the top left corner.  Since the maze is completely uncarved, 
        // if I request random directions multiple times, I should always get one 
        // of (DIR_SOUTH, DIR_EAST) - the other two directions are off the maze
        for _i in 0..10 {
            let (result, value) = maze.pick_direction(0, 0);
            assert_ne!(false, result);
            if value == constants::DIR_WEST || value == constants::DIR_NORTH {
                panic!("Received invalid direction at (0,0)");
            }
        }

        // Use the top right corner.  Since the maze is completely uncarved,
        // the returned directions should always be DIR_WEST or DIR_SOUTH.
        for _i in 0..10 {
            let (result, value) = maze.pick_direction(9, 0);
            assert_ne!(false, result);
            if value == constants::DIR_EAST || value == constants::DIR_NORTH {
                panic!("Received invalid direction at (9,0)");
            }
        }

        // Use the bottom left corner.  Since the maze is completely uncarved,
        // the returned directions should always be DIR_EAST or DIR_NORTH.
        for _i in 0..10 {
            let (result, value) = maze.pick_direction(0, 9);
            assert_ne!(false, result);
            if value == constants::DIR_WEST || value == constants::DIR_SOUTH {
                panic!("Received invalid direction at (0,9)");
            }
        }

        // Use the bottom right corner.  Since the maze is completely uncarved,
        // the returned directions should always be DIR_WEST or DIR_NORTH.
        for _i in 0..10 {
            let (result, value) = maze.pick_direction(9, 9);
            assert_ne!(false, result);
            if value == constants::DIR_EAST || value == constants::DIR_SOUTH {
                panic!("Received invalid direction at (9,9)")
            }
        }

        // Pick a center location.  Since the maze is completely uncarved,
        // any of the four directions should be returned
        for _i in 1..20 {
            let (result, _) = maze.pick_direction(3, 3);
            assert_ne!(false, result);
        }

        // Carve an adjacent location to the previous test.  Since that location
        // is carved, the direction of that location should not be returned
        let _result = maze.carve(3, 2, constants::DIR_NORTH);
        assert_eq!([false, true, true, true], maze.sq[maze.get_offset(3, 2) as usize].wall_present);
        for _i in 1..20 {
            let (result, value) = maze.pick_direction(3, 3);
            assert_eq!(true, result);
            assert_ne!(constants::DIR_NORTH, value);
        }

        // Carve a second adjacent location.  Since there are now two locations
        // carved, only two possible directions should be returned.
        let _result = maze.carve(2, 3, constants::DIR_WEST);
        assert_eq!([true, true, true, false], maze.sq[maze.get_offset(2, 3) as usize].wall_present);
        for _i in 1..20 {
            let (result, value) = maze.pick_direction(3, 3);
            assert_eq!(true, result);
            assert_ne!(constants::DIR_NORTH, value);
            assert_ne!(constants::DIR_WEST, value);
        }        

        // Carve a third adjacent location.  Since there are now three locations
        // carved, only one possible direction should be returned.
        let _result = maze.carve(4, 3, constants::DIR_SOUTH);
        assert_eq!([true, false, true, true], maze.sq[maze.get_offset(4, 3) as usize].wall_present);
        for _i in 1..20 {
            let (result, value) = maze.pick_direction(3, 3);
            assert_eq!(true, result);
            assert_ne!(constants::DIR_NORTH, value);
            assert_ne!(constants::DIR_WEST, value);
            assert_ne!(constants::DIR_EAST, value);
        }      

        // Carve the last adjacent location.  Since there are now four locations
        // carved, no directions should be returned.
        let _result = maze.carve(3, 4, constants::DIR_WEST);
        assert_eq!([true, true, true, false], maze.sq[maze.get_offset(3, 4) as usize].wall_present);
        for _i in 1..20 {
            let (result, _value) = maze.pick_direction(3, 3);            
            assert_eq!(false, result);
        }              

    }
}