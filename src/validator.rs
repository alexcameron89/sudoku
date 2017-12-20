use sudoku::Puzzle;

pub fn valid(puzzle: &Puzzle) -> bool {
    return true
}

#[cfg(test)]
mod tests {
    use validator::valid;
    use sudoku::Puzzle;

    #[test]
    fn it_returns_a_valid_puzzle_as_valid() {
      let valid_puzzle = Puzzle {};
      assert!(valid(&valid_puzzle));
    }
}
