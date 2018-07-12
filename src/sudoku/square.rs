const VALID_ROW: [i32; 9] = [1,2,3,4,5,6,7,8,9];

#[derive(Clone, Debug)]
pub struct Square {
    pub row: i32,
    pub column: i32,
    pub possible_values: [i32; 9],
    pub set: bool,
}

impl PartialEq for Square {
    fn eq(&self, other: &Square) -> bool {
        self.row == other.row &&
            self.column == other.column
    }
}

impl Square {
    pub fn possible_values(&self, grid: &[Vec<i32>]) -> Vec<i32> {
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

    pub fn in_same_region_as(&self, _other_square: &Square) -> bool {
        false
    }

    pub fn remove_from_values(&self, _grid: &[Vec<i32>], _removed_square: &Square) {
    }
}
