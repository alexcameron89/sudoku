use sudoku::Puzzle;

pub fn valid(puzzle: &Puzzle) -> bool {
    let valid_sorted_row = vec![1,2,3,4,5,6,7,8,9];
    for row in &puzzle.grid {
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
}
