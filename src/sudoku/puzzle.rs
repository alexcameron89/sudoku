use sudoku::Square;

#[derive(Clone,Debug)]
pub struct Puzzle {
    pub grid: Vec<Vec<i32>>,
    unsolved: Vec<Square>,
}

impl Puzzle {
    pub fn new(grid: Vec<Vec<i32>>) -> Puzzle {
        Self {
            unsolved: Self::build_unsolved_squares(&grid),
            grid,
        }
    }

    pub fn set_square_value(&mut self, square: &Square, value: i32) {
        self.grid[square.row as usize][square.column as usize] = value;
        self.remove_from_unsolved(square);
        self.update_other_unsolved_squares(square);
    }

    pub fn unsolved_squares(&self) -> Vec<Square> {
        self.unsolved.clone()
    }

    fn remove_from_unsolved(&mut self, square: &Square) {
        let square_position_from_list = self.unsolved.iter().position( |s| s == square ).unwrap();
        self.unsolved.remove(square_position_from_list);
    }

    fn update_other_unsolved_squares(&mut self, removed_square: &Square) {
        for square in &self.unsolved {
            if square.in_same_region_as(removed_square) {
                square.remove_from_values(&self.grid, removed_square);
            }
        }
    }

    fn build_unsolved_squares(grid: &[Vec<i32>]) -> Vec<Square> {
        let mut squares = Vec::new();
        for row in 0..9 {
            for column in 0..9 {
                if Self::not_set(row as usize, column as usize, grid) {
                    let square = Square {
                        row,
                        column,
                        possible_values: [1,2,3,4,5,6,7,8,9],
                        set: false,
                    };

                    squares.push(square);
                }
            }
        }

        squares
    }

    fn not_set(row: usize, column: usize, grid: &[Vec<i32>]) -> bool {
        if grid[row][column] == (0 as i32) {
            return true
        }

        false
    }

}
