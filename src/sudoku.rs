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
    pub possible_values: Vec<isize>,
    pub set: bool,
}


impl Puzzle {
    pub fn new(grid: Vec<Vec<isize>>) -> Puzzle {
        Self {
            unsolved: Self::build_unsolved_squares(&grid),
            grid: grid,
        }
    }

    pub fn set_square_value(&mut self, row: i32, column: i32, value: isize) {
        self.grid[row as usize][column as usize] = value;
        let mut unsolved: Vec<Square> = self.unsolved.take().unwrap();
        let square = unsolved.iter().position( |s| s.row == row && s.column == column ).unwrap();
        unsolved.remove(square);
        self.unsolved = match unsolved.len() {
            0 => None,
            _ => Some(unsolved)
        };
    }

    pub fn unsolved_squares(&self) -> Vec<Square> {
        match self.unsolved {
            Some(ref squares) => squares.clone(),
            None => Vec::new(),
        }
    }

    fn build_unsolved_squares(grid: &Vec<Vec<isize>>) -> Option<Vec<Square>> {
        let mut squares = Vec::new();
        for row in 0..9 {
            for column in 0..9 {
                if Self::not_set(row as usize, column as usize, grid) {
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
            let puzzle = Puzzle::new(grid);
            assert_eq!(vec![3,9], puzzle.possible_values(4, 4));
    }
}
