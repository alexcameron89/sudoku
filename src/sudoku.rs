const VALID_ROW: [isize; 9] = [1,2,3,4,5,6,7,8,9];

#[derive(Clone,Debug)]
pub struct Puzzle {
    pub grid: Vec<Vec<isize>>,
    unsolved: Option<Vec<Square>>,
}

#[derive(Clone, Debug)]
pub struct Square {
    pub row: i32,
    pub column: i32,
    pub possible_values: Option<Vec<isize>>,
    pub set: bool,
}


impl Puzzle {
    pub fn new(grid: Vec<Vec<isize>>) -> Puzzle {
        Self {
            unsolved: Self::build_unsolved_squares(&grid),
            grid: grid,
        }
    }

    pub fn set_square_value(&mut self, square: &Square, value: isize) {
        self.grid[square.row as usize][square.column as usize] = value;
        self.remove_from_unsolved(square);
        self.update_other_unsolved_squares(square);
    }

    pub fn unsolved_squares(&self) -> Vec<Square> {
        match self.unsolved {
            Some(ref squares) => squares.clone(),
            None => Vec::new(),
        }
    }

    fn remove_from_unsolved(&mut self, square: &Square) {
        let mut unsolved: Vec<Square> = self.unsolved.take().unwrap();
        let square_position_from_list = unsolved.iter().position( |s| s == square ).unwrap();
        unsolved.remove(square_position_from_list);
        self.unsolved = match unsolved.len() {
            0 => None,
            _ => Some(unsolved)
        };
    }

    fn update_other_unsolved_squares(&mut self, removed_square: &Square) {
        let unsolved: Vec<Square> = match self.unsolved.take() {
            None => return,
            Some(squares) => squares,
        };

        for square in &unsolved {
            if square.in_same_region_as(removed_square) {
                square.remove_from_values(&self.grid, removed_square);
            }
        }

        self.unsolved = match unsolved.len() {
            0 => None,
            _ => Some(unsolved)
        };
    }

    fn build_unsolved_squares(grid: &Vec<Vec<isize>>) -> Option<Vec<Square>> {
        let mut squares = Vec::new();
        for row in 0..9 {
            for column in 0..9 {
                if Self::not_set(row as usize, column as usize, grid) {
                    let square = Square {
                        row: row,
                        column: column,
                        possible_values: None,
                        set: false,
                    };

                    squares.push(square);
                }
            }
        }

        match squares.len() {
            0 => None,
            _ => Some(squares)
        }
    }

    fn not_set(row: usize, column: usize, grid: &Vec<Vec<isize>>) -> bool {
        if grid[row][column] == (0 as isize) {
            return true
        }

        return false
    }

}

impl PartialEq for Square {
    fn eq(&self, other: &Square) -> bool {
        self.row == other.row &&
            self.column == other.column
    }
}

impl Square {
    pub fn possible_values(&self, grid: &Vec<Vec<isize>>) -> Vec<isize> {
        let mut all_possible_values = VALID_ROW.to_vec();
        let row_numbers = &grid[self.row as usize];
        for number in row_numbers {
            all_possible_values.retain(|i| i != number);
        }
        for number in 0..9 {
            all_possible_values.retain(|i| *i != grid[number][self.column as usize]);
        }
        let expected_column_grid = self.column / 3;
        let expected_row_grid = self.row / 3;
        for row_number in 0..9 {
            for column_number in 0..9 {
                let current_row_grid = row_number / 3;
                let current_column_grid = column_number / 3;
                if (current_row_grid == expected_row_grid) &&
                    (current_column_grid == expected_column_grid) {
                        all_possible_values.retain(|i| *i != grid[row_number as usize][column_number as usize]);
                    }
            }
        }

        all_possible_values
    }

    fn in_same_region_as(&self, other_square: &Square) -> bool {
        false
    }

    fn remove_from_values(&self, grid: &Vec<Vec<isize>>, removed_square: &Square) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_filters_used_values_in_rows_grids_and_columns() {
        let grid = vec![
            vec![0,0,0,0,1,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0],
            vec![0,0,0,7,0,8,0,0,0],
            vec![4,0,5,0,0,0,0,6,0],
            vec![0,0,9,0,0,0,0,0,0],
            vec![0,0,0,0,2,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,3,0,0]
        ];
            let square = Square {
                row: 4,
                column: 4,
                possible_values: None,
                set: false
            };
            assert_eq!(vec![3,9], square.possible_values(&grid));
    }
}
