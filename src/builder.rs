use sudoku::Puzzle;
use solver::Solver;

fn build_complete_puzzle() -> Vec<Vec<isize>> {
    let grid = vec![vec![0;9];9];
    let puzzle = Puzzle { grid: grid };
    let mut solver = Solver { puzzle: puzzle };

    solver.solve_puzzle()
}

#[cfg(test)]
mod tests {
    use validator::valid;
    use builder::build_complete_puzzle;

    #[test]
    fn it_builds_a_valid_puzzle() {
        let puzzle = build_complete_puzzle();
        assert!(valid(&puzzle));
    }
}
