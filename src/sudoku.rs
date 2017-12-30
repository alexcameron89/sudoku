const VALID_ROW: [isize; 9] = [1,2,3,4,5,6,7,8,9];

pub struct Puzzle {
    pub grid: Vec<Vec<isize>>,
}

#[derive(Debug)]
pub struct Square {
    pub position: Vec<i32>,
    pub possible_values: Vec<isize>,
    pub set: bool,
}


impl Puzzle {
    pub fn squares(&self) -> Vec<Square> {
        let mut squares = Vec::new();
        for row in 0..9 {
            for column in 0..9 {
                let square = Square {
                    position: vec![row, column],
                    possible_values: self.possible_values(row, column),
                    set: false,
                };

                squares.push(square);
            }
        }

        squares
    }

    fn possible_values(&self, row: i32, column: i32) -> Vec<isize> {
        let mut all_possible_values = vec![1,2,3,4,5,6,7,8,9];
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

impl PartialEq for Square {
    fn eq(&self, other: &Square) -> bool {
        self.position == other.position &&
            self.possible_values == other.possible_values
    }
}
