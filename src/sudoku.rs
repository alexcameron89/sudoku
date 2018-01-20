const VALID_ROW: [isize; 9] = [1,2,3,4,5,6,7,8,9];

#[derive(Clone,Debug)]
pub struct Puzzle {
    pub grid: Vec<Vec<isize>>,
}

#[derive(Debug)]
pub struct Square {
    pub row: i32,
    pub column: i32,
    pub possible_values: Vec<isize>,
    pub set: bool,
}


impl Puzzle {
    pub fn set_square_value(&mut self, row: i32, column: i32, value: isize) {
        self.grid[row as usize][column as usize] = value;
    }

    pub fn unsolved_squares(&self) -> Vec<Square> {
        let mut squares = Vec::new();
        for row in 0..9 {
            for column in 0..9 {
                if !self.verify_set_number(row as usize, column as usize) {
                    let square = Square {
                        row: row,
                        column: column,
                        possible_values: vec![1,2,3,4,5,6,7,8,9],
                        set: false,
                    };

                    squares.push(square);
                }
            }
        }

        squares
    }

    pub fn possible_values(&self, row: i32, column: i32) -> Vec<isize> {
        let mut all_possible_values = VALID_ROW.to_vec();
        let row_numbers = &self.grid[row as usize];
        for number in row_numbers {
            all_possible_values.retain(|i| i != number);
        }
        for number in 0..9 {
            all_possible_values.retain(|i| i != &self.grid[number][column as usize]);
        }
        let expected_column_grid = column / 3;
        let expected_row_grid = row / 3;
        for row in 0..9 {
            for column in 0..9 {
                let current_row_grid = row / 3;
                let current_column_grid = column / 3;
                if (current_row_grid == expected_row_grid) &&
                    (current_column_grid == expected_column_grid) {
                        all_possible_values.retain(|i| i != &self.grid[row as usize][column as usize]);
                    }
            }
        }

        all_possible_values
    }

    fn verify_set_number(&self, row: usize, column: usize) -> bool {
        if &self.grid[row][column] > &(0 as isize) {
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
            let puzzle = Puzzle { grid: grid };
            assert_eq!(vec![3,9], puzzle.possible_values(4, 4));
    }
}
