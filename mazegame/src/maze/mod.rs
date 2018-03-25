// The square module
pub mod square;

pub use constants;
pub use maze::square::Square;

extern crate rand;
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
    /// Creates a new Maze of the specified size.
    /// 
    /// # Example
    /// ```
    /// let maze = mazegame::Maze::new(10, 10);
    /// ``` 
    pub fn new(rows: u32, cols: u32) -> Maze {
        Maze {
            rows: rows as u32,
            cols: cols as u32,
            sq: vec![Square::new(); (rows * cols) as usize],
        }
    }

    /// Returns the number of rows in the Maze.
    /// 
    /// # Example
    /// ```
    /// let maze = mazegame::Maze::new(10, 10);
    /// assert_eq!(10, maze.get_rows());
    /// ```
    pub fn get_rows(&self) -> u32 {
        self.rows
    }

    /// Returns the number of columns in the Maze.
    /// 
    /// # Example
    /// ``` 
    /// let maze = mazegame::Maze::new(10, 10);
    /// assert_eq!(10, maze.get_cols());
    /// ```
    pub fn get_cols(&self) -> u32 {
        self.cols
    }

    // Internal: Calculates the offset into the Maze's Square vector
    // based on x and y position.
    fn get_offset(&self, x: u32, y: u32) -> usize {
        let u = (y * self.cols + x) as usize;
        u
    }

    /// Carves a Square in the Maze in a given direction.
    /// 
    /// # About
    /// This function, given a direction, will carve a pair of maze squares, effectively
    /// connecting the two together.  To accomplish this, the Square at position (x,y)
    /// in the Maze will be carved in the specified direction.  A second square, 
    /// adjacent to the first square in the direction carved, will also be carved,
    /// but in the opposite direction - connecting the two Squares together. 
    /// 
    /// # Example
    /// ``` 
    /// use mazegame::constants;
    /// 
    /// let mut maze = mazegame::Maze::new(10, 10);
    /// let result = maze.carve(5, 5, constants::DIR_NORTH);
    /// assert_eq!(Ok(()), result);
    /// ```
    pub fn carve(&mut self, x: u32, y: u32, dir: u32) -> Result<(), String> {
        // If the maze index is invalid, just return 
        if y >= self.rows { 
            return Err(format!("Can't carve outside of maze at ({}, {})", x, y)); 
        }
        if x >= self.cols { 
            return Err(format!("Can't carve outside of maze at ({}, {})", x, y)); 
        }

        // The position of the square on the other side of the wall
        let mut dest_x = x;        
        let mut dest_y = y;
        let dest_dir;

        // Prevent breaking of walls that would lead out of the maze
        match dir {
            constants::DIR_NORTH => {
                if y == 0 { 
                    return Err(format!("Can't build north wall at ({}, {})", x, y)); 
                }
                dest_y = dest_y - 1;
                dest_dir = constants::DIR_SOUTH;
            }
            constants::DIR_SOUTH => {
                if y == self.rows - 1 { 
                    return Err(format!("Can't build south wall at ({}, {})", x, y)); 
                }
                dest_y = dest_y + 1;
                dest_dir = constants::DIR_NORTH;
            }
            constants::DIR_EAST => {
                if x == self.cols -1 { 
                    return Err(format!("Can't build east wall at ({}, {})", x, y)); 
                }
                dest_x = dest_x + 1;
                dest_dir = constants::DIR_WEST;
            }
            constants::DIR_WEST => {
                if x == 0 { 
                    return Err(format!("Can't build west wall at ({}, {})", x, y)); 
                }
                dest_x = dest_x - 1;
                dest_dir = constants::DIR_EAST;
            }
            _ => { 
                return Err(format!("Can't build wall in illegal direction {}", dir)); 
            }
        }

        // Need mutable mutable references, but not at the same time.
        // Stick em in their own scopes.
        {
            let offset = self.get_offset(x, y);
            let square = &mut self.sq[offset];
            square.break_wall(dir);
        }

        {
            let offset = self.get_offset(dest_x, dest_y);
            let square = &mut self.sq[offset];
            square.break_wall(dest_dir);
        }

        return Ok(());
    }

    /// Generates a maze.
    /// 
    /// # Notes
    /// * Currently, this function only calls a maze generator.  Eventually, it will also 
    ///   generate rooms and fill in unnecessary passages.
    /// 
    /// # Example
    /// ```
    /// let mut maze = mazegame::Maze::new(10, 10);
    /// maze.generate();
    /// ```
    pub fn generate(&mut self) -> Result<(), String> {
        // For now:
        // - Call the algorithmic generator

        // Eventually:
        // - Generate some rooms
        // - Call the algorithmic generator
        // - Connect rooms to the maze
        // - Remove dead ends
        
        let result = self.generate_growing_tree(0, 0);
        if result != Ok(()) {
            return result;
        }

        return Ok(());
    }

    //
    // Internal - generates a perfect maze using the growing tree algorithm.
    //
    fn generate_growing_tree(&mut self, start_x: u32, start_y: u32) -> Result<(), String> {
        let mut visited: Vec<Coord> = Vec::new();
        let mut cur_coord = Coord { x: start_x, y: start_y };

        // Handle the initial square
        let (result, dir) = self.pick_direction(cur_coord.x, cur_coord.y);
        if result == false { 
            return Err(format!("Unable to pick initial direction in generator!")); 
        } else {
            self.carve(cur_coord.x, cur_coord.y, dir).unwrap();
            visited.push(cur_coord.clone());
            match dir {
                constants::DIR_NORTH => cur_coord.y = cur_coord.y - 1,
                constants::DIR_SOUTH => cur_coord.y = cur_coord.y + 1,
                constants::DIR_EAST => cur_coord.x = cur_coord.x + 1,
                constants::DIR_WEST => cur_coord.x = cur_coord.x - 1,
                _ => {
                    return Err(format!("Illegal direction in generate_growing_tree!"));
                }
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
                    None => {
                        return Err(format!("Pop failed in generate_growing_tree!"));
                    }
                }
            } else {
                self.carve(cur_coord.x, cur_coord.y, dir).unwrap();
                visited.push(cur_coord.clone());
                match dir {
                    constants::DIR_NORTH => cur_coord.y = cur_coord.y - 1,
                    constants::DIR_SOUTH => cur_coord.y = cur_coord.y + 1,
                    constants::DIR_EAST => cur_coord.x = cur_coord.x + 1,
                    constants::DIR_WEST => cur_coord.x = cur_coord.x - 1,
                    _ => {
                        return Err(format!("Illegal direction in generate_growing_tree!"));
                    }
                }
            }
        }

        Ok(())
    }

    //
    // Internal - picks a random direction to tunnel a new maze square
    //
    fn pick_direction(&self, x: u32, y: u32) -> (bool, u32) {
        let mut directions: Vec<u32> = Vec::new();
        let mut rng = thread_rng();

        if y > 0 {
            let sq = &self.sq[self.get_offset(x, y-1)];            
            if sq.is_carved() == false {
                directions.push(constants::DIR_NORTH);
            }
        }
        if y < self.rows - 1 {
            let sq = &self.sq[self.get_offset(x, y+1)];
            if sq.is_carved() == false {
                directions.push(constants::DIR_SOUTH);
            }
        }
        if x < self.cols - 1 {
            let sq = &self.sq[self.get_offset(x+1, y)];
            if sq.is_carved() == false {
                directions.push(constants::DIR_EAST);
            }
        }
        if x > 0 {
            let sq = &self.sq[self.get_offset(x-1, y)];
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

    /// Displays a reprentation of a maze to the console.
    /// 
    /// # Example:
    /// ```
    /// let mut m = mazegame::Maze::new(10, 10);
    /// m.generate();
    /// m.print();
    /// ```
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
                assert_eq!(true, maze.sq[maze.get_offset(row, col)].wall_present[constants::DIR_NORTH as usize]);
                assert_eq!(true, maze.sq[maze.get_offset(row, col)].wall_present[constants::DIR_SOUTH as usize]);
                assert_eq!(true, maze.sq[maze.get_offset(row, col)].wall_present[constants::DIR_EAST as usize]);
                assert_eq!(true, maze.sq[maze.get_offset(row, col)].wall_present[constants::DIR_WEST as usize]);        
            }
        }
    }

    #[test] 
    fn test_carver() {
        let mut maze = Maze::new(10, 10);

        // Note: In each of the carve phases, when a carve is legal, there are
        // *two* squares that need to be checked:
        // - carving east : east wall of current square, west wall of square to the east
        // - carving west : west wall of current square, east wall of square to the west
        // - carving north: north wall of current square, south wall of square to the north
        // - carving south: south wall of current square, north wall of square to the south

        // Carve south from the top left corner.  This should work.
        let result = maze.carve(0, 0, constants::DIR_SOUTH);
        assert_eq!([true, false, true, true], maze.sq[maze.get_offset(0, 0)].wall_present);
        assert_eq!([false, true, true, true], maze.sq[maze.get_offset(0, 1)].wall_present);
        assert_eq!(Ok(()), result);

        // Carve north from the top corner.  This *shouldn't* work (so the vec should be unchanged)
        let result = maze.carve(0, 0, constants::DIR_NORTH);
        assert_eq!([true, false, true, true], maze.sq[maze.get_offset(0, 0)].wall_present);
        assert_ne!(Ok(()), result);

        // Carve west from the top corner.  This *shouldn't* work.
        let result = maze.carve(0, 0, constants::DIR_WEST);
        assert_eq!([true, false, true, true], maze.sq[maze.get_offset(0, 0)].wall_present);
        assert_ne!(Ok(()), result);

        // Carve east from the top corner.  This should work.
        let result = maze.carve(0, 0, constants::DIR_EAST);
        assert_eq!([true, false, false, true], maze.sq[maze.get_offset(0, 0)].wall_present);
        assert_eq!([true, true, true, false], maze.sq[maze.get_offset(1, 0)].wall_present);
        assert_eq!(Ok(()), result);

        // Carve north from the bottom right corner.  This should work.
        let result = maze.carve(9, 9, constants::DIR_NORTH);
        assert_eq!([false, true, true, true], maze.sq[maze.get_offset(9, 9)].wall_present);
        assert_eq!([true, false, true, true], maze.sq[maze.get_offset(9, 8)].wall_present);
        assert_eq!(Ok(()), result);

        // Carve south from the bottom right corner.  This *shouldn't* work (so the vec should be unchanged)
        let result = maze.carve(9, 9, constants::DIR_SOUTH);
        assert_eq!([false, true, true, true], maze.sq[maze.get_offset(9, 9)].wall_present);
        assert_ne!(Ok(()), result);

        // Carve east from the bottom right corner.  This *shouldn't* work.
        let result = maze.carve(9, 9, constants::DIR_EAST);
        assert_eq!([false, true, true, true], maze.sq[maze.get_offset(9, 9)].wall_present);
        assert_ne!(Ok(()), result);

        // Carve west from the bottom right corner.  This should work.
        let result = maze.carve(9, 9, constants::DIR_WEST);
        assert_eq!([false, true, true, false], maze.sq[maze.get_offset(9, 9)].wall_present);
        assert_eq!([true, true, false, true] , maze.sq[maze.get_offset(8, 9)].wall_present);                
        assert_eq!(Ok(()), result);

        // Carve in each direction from a central square.  All these should work.
        let result = maze.carve(4, 5, constants::DIR_NORTH);
        assert_eq!([false, true, true, true], maze.sq[maze.get_offset(4, 5)].wall_present);
        assert_eq!([true, false, true, true], maze.sq[maze.get_offset(4, 4)].wall_present);        
        assert_eq!(Ok(()), result);

        let result = maze.carve(4, 5, constants::DIR_SOUTH);
        assert_eq!([false, false, true, true], maze.sq[maze.get_offset(4, 5)].wall_present);
        assert_eq!([false, true, true, true], maze.sq[maze.get_offset(4, 6)].wall_present);        
        assert_eq!(Ok(()), result);

        let result = maze.carve(4, 5, constants::DIR_EAST);
        assert_eq!([false, false, false, true], maze.sq[maze.get_offset(4, 5)].wall_present);
        assert_eq!([true, true, true, false], maze.sq[maze.get_offset(5, 5)].wall_present);        
        assert_eq!(Ok(()), result);

        let result = maze.carve(4, 5, constants::DIR_WEST);
        assert_eq!([false, false, false, false], maze.sq[maze.get_offset(4, 5)].wall_present);
        assert_eq!([true, true, false, true], maze.sq[maze.get_offset(3, 5)].wall_present);        
        assert_eq!(Ok(()), result);

        // Try carving out of bounds.  This shouldn't work.
        let result = maze.carve(100, 100, constants::DIR_NORTH);
        assert_ne!(Ok(()), result);
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
        let result = maze.carve(3, 2, constants::DIR_NORTH);
        assert_eq!([false, true, true, true], maze.sq[maze.get_offset(3, 2)].wall_present);
        assert_eq!(Ok(()), result);
        for _i in 1..20 {
            let (result, value) = maze.pick_direction(3, 3);
            assert_eq!(true, result);
            assert_ne!(constants::DIR_NORTH, value);
        }

        // Carve a second adjacent location.  Since there are now two locations
        // carved, only two possible directions should be returned.
        let result = maze.carve(2, 3, constants::DIR_WEST);
        assert_eq!([true, true, true, false], maze.sq[maze.get_offset(2, 3)].wall_present);
        assert_eq!(Ok(()), result);        
        for _i in 1..20 {
            let (result, value) = maze.pick_direction(3, 3);
            assert_eq!(true, result);
            assert_ne!(constants::DIR_NORTH, value);
            assert_ne!(constants::DIR_WEST, value);
        }        

        // Carve a third adjacent location.  Since there are now three locations
        // carved, only one possible direction should be returned.
        let result = maze.carve(4, 3, constants::DIR_SOUTH);
        assert_eq!([true, false, true, true], maze.sq[maze.get_offset(4, 3)].wall_present);
        assert_eq!(Ok(()), result);
        for _i in 1..20 {
            let (result, value) = maze.pick_direction(3, 3);
            assert_eq!(true, result);
            assert_ne!(constants::DIR_NORTH, value);
            assert_ne!(constants::DIR_WEST, value);
            assert_ne!(constants::DIR_EAST, value);
        }      

        // Carve the last adjacent location.  Since there are now four locations
        // carved, no directions should be returned.
        let result = maze.carve(3, 4, constants::DIR_WEST);
        assert_eq!([true, true, true, false], maze.sq[maze.get_offset(3, 4)].wall_present);
        assert_eq!(Ok(()), result);
        for _i in 1..20 {
            let (result, _value) = maze.pick_direction(3, 3);            
            assert_eq!(false, result);
        }              
    }
}