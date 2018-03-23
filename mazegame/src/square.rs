pub use constants;

#[derive(Clone, Debug)]
pub struct Square {
    pub wall_present: [bool; constants::NUM_DIRECTIONS as usize],
    pub id: u32,
}

impl Square {

    pub fn new() -> Square {
        Square {
            wall_present: [true, true, true, true],
            id: 0,
        }
    }
    
    pub fn break_wall(&mut self, dir: u32) {
        self.set_wall_state(dir, false);
    }

    pub fn build_wall(&mut self, dir: u32) {
        self.set_wall_state(dir, true);
    }

    pub fn is_carved(&self) -> bool {
        // If any wall is missing, the square is considered 'carved'.
        if self.is_wall_present(constants::DIR_NORTH) == false { return true; }
        if self.is_wall_present(constants::DIR_SOUTH) == false { return true; }
        if self.is_wall_present(constants::DIR_EAST) == false { return true; }
        if self.is_wall_present(constants::DIR_WEST) == false { return true; }
        return false;
    }

    pub fn is_wall_present(&self, dir: u32) -> bool {
        match dir {
            constants::DIR_NORTH => {
                self.wall_present[constants::DIR_NORTH as usize]
            }
            constants::DIR_SOUTH => {
                self.wall_present[constants::DIR_SOUTH as usize]
            }
            constants::DIR_EAST => {
                self.wall_present[constants::DIR_EAST as usize]
            }
            constants::DIR_WEST => {
                self.wall_present[constants::DIR_WEST as usize]
            }
            _ => { false }  // Consider using Result?
        }
    }

    fn set_wall_state(&mut self, dir: u32, state: bool) {
        match dir {
            constants::DIR_NORTH => { 
                self.wall_present[constants::DIR_NORTH as usize] = state;
            },
            constants::DIR_SOUTH => {
                self.wall_present[constants::DIR_SOUTH as usize] = state;
                },
            constants::DIR_EAST => {
                self.wall_present[constants::DIR_EAST as usize] = state;
            },
            constants::DIR_WEST => {
                self.wall_present[constants::DIR_WEST as usize] = state;
            },
            _ => {
                // Ignore the request
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_square() {
        let square = Square::new();
        assert_eq!(square.id, 0);
    }

    #[test]
    fn test_build_wall() {
        // For each wall, break the wall, confirm it's broken,
        // rebuild and confirm it's fixed.  Also checks the
        // is_carved function.
        let mut square = Square::new();
        square.break_wall(constants::DIR_NORTH);
        assert_eq!([false, true, true, true], square.wall_present);
        assert_eq!(true, square.is_carved());
        square.build_wall(constants::DIR_NORTH);
        assert_eq!([true, true, true, true], square.wall_present);
        assert_eq!(false, square.is_carved());
    }

    #[test]
    fn test_break_wall() {
        // Test the standard behavior (break walls, check to see if they're 
        // broken)
        let mut square = Square::new();
        square.break_wall(constants::DIR_NORTH);
        assert_eq!([false, true, true, true], square.wall_present);
        square.break_wall(constants::DIR_SOUTH);
        assert_eq!([false, false, true, true], square.wall_present);
        square.break_wall(constants::DIR_EAST);
        assert_eq!([false, false, false, true], square.wall_present);
        square.break_wall(constants::DIR_WEST);
        assert_eq!([false, false, false, false], square.wall_present);
        
        // Try breaking an already broken wall
        square.break_wall(constants::DIR_NORTH);
        assert_eq!([false, false, false, false], square.wall_present);

        // Break a wall in an invalid direction and make sure
        // nothing has changed
        let mut square = Square::new();
        square.break_wall(17);
        assert_eq!([true, true, true, true], square.wall_present);
    }
}