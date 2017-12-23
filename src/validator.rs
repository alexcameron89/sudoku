use sudoku::Puzzle;

pub fn valid(puzzle: &Puzzle) -> bool {
    let valid_sorted_row = vec![1,2,3,4,5,6,7,8,9];
    let rows_are_valid = rows_are_valid(&valid_sorted_row, &puzzle.grid);
    let columns_are_valid = columns_are_valid(&valid_sorted_row, &puzzle.grid);
    let grids_are_valid = grids_are_valid(&valid_sorted_row, &puzzle.grid);

    return rows_are_valid
        && columns_are_valid
        && grids_are_valid
}

fn rows_are_valid(valid_sorted_row: &Vec<isize>, puzzle_grid: &Vec<Vec<isize>>) -> bool {
    for row in puzzle_grid {
        let mut row_to_sort = row.to_vec();
        row_to_sort.sort();
        for (i, number) in row_to_sort.iter().enumerate() {
            if number != &valid_sorted_row[i] {
                return false
            }
        }
    }

    return true
}

fn columns_are_valid(valid_sorted_row: &Vec<isize>, puzzle_grid: &Vec<Vec<isize>>) -> bool {
  for column in 0..9 {
      let mut sorted_row = vec![0; 9];
      for row in 0..9 {
          let number = puzzle_grid[row][column];
          sorted_row[(number - 1) as usize] = number;
      }
      for (i, number) in sorted_row.iter().enumerate() {
          if number != &valid_sorted_row[i] {
              return false
          }
      }
  }

  return true
}

fn grids_are_valid(valid_sorted_row: &Vec<isize>, puzzle_grid: &Vec<Vec<isize>>) -> bool {
    let mut grids = vec![];
    let mut grid_group_number = 0;
    let mut grid_group = Vec::new();
    let mut new_grid = Vec::new();
    for group in 0..9 {
        grid_group_number = group % 3;
        new_grid = Vec::new();
        grid_group.push(new_grid);
        if grid_group_number == 2 {
            grids.push(grid_group);
            grid_group = Vec::new();
        }
    }
    for row in 0..9 {
        for column in 0..9 {
            let number = puzzle_grid[row][column];
            grids[(row / 3)][(column / 3)].push(number);
        }
    }

    for group in 0..3 {
        for grid in 0..3 {
            let mut row_to_sort = grids[group][grid].to_vec();
            row_to_sort.sort();
            for (i, number) in row_to_sort.iter().enumerate() {
                if number != &valid_sorted_row[i] {
                    return false
                }
            }
        }
    }

    return true
}

#[test]
fn duplicate_numbers_in_a_grid_are_invalid() {
    let valid_sorted_row = vec![1,2,3,4,5,6,7,8,9];
    let invalid_grid = vec![
        vec![1,2,3,4,5,6,7,8,9],
        vec![2,3,4,5,6,7,8,9,1],
        vec![3,4,5,6,7,8,9,1,2],
        vec![4,5,6,7,8,9,1,2,3],
        vec![5,6,7,8,9,1,2,3,4],
        vec![6,7,8,9,1,2,3,4,5],
        vec![7,8,9,1,2,3,4,5,6],
        vec![8,9,1,2,3,4,5,6,7],
        vec![9,1,2,3,4,5,6,7,8]
    ];

    assert!(!grids_are_valid(&valid_sorted_row, &invalid_grid));
}

#[cfg(test)]
mod tests {
    use validator::valid;
    use sudoku::Puzzle;

    #[test]
    fn it_returns_a_valid_puzzle_as_valid() {
        let grid = vec![
            vec![4,2,9,8,1,3,5,6,7],
            vec![5,1,6,4,7,2,9,3,8],
            vec![7,8,3,6,5,9,2,4,1],
            vec![6,7,2,1,3,4,8,5,9],
            vec![3,9,5,2,8,6,1,7,4],
            vec![8,4,1,7,9,5,6,2,3],
            vec![1,5,8,3,6,7,4,9,2],
            vec![9,3,4,5,2,8,7,1,6],
            vec![2,6,7,9,4,1,3,8,5]
        ];
            let valid_puzzle = Puzzle { grid: grid };
            assert!(valid(&valid_puzzle));
    }

    #[test]
    fn it_returns_an_invalid_puzzle_as_invalid() {
        let grid = vec![
            vec![1,2,3,4,5,6,7,8,9],
            vec![1,2,3,4,5,6,7,8,9],
            vec![1,2,3,4,5,6,7,8,9],
            vec![1,2,3,4,5,6,7,8,9],
            vec![1,2,3,4,5,6,7,8,9],
            vec![1,2,3,4,5,6,7,8,9],
            vec![1,2,3,4,5,6,7,8,9],
            vec![1,2,3,4,5,6,7,8,9],
            vec![1,2,3,4,5,6,7,8,9]
        ];
            let invalid_puzzle = Puzzle { grid: grid };
            assert!(!valid(&invalid_puzzle));
    }

    #[test]
    fn it_returns_a_second_invalid_puzzle_as_invalid() {
        let grid = vec![
            vec![1,2,3,4,5,6,7,8,9],
            vec![2,3,4,5,6,7,8,9,1],
            vec![3,4,5,6,7,8,9,1,2],
            vec![4,5,6,7,8,9,1,2,3],
            vec![5,6,7,8,9,1,2,3,4],
            vec![6,7,8,9,1,2,3,4,5],
            vec![7,8,9,1,2,3,4,5,6],
            vec![8,9,1,2,3,4,5,6,7],
            vec![9,1,2,3,4,5,6,7,8]
        ];
            let invalid_puzzle = Puzzle { grid: grid };
            assert!(!valid(&invalid_puzzle));
    }
}
