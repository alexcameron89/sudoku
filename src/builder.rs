use sudoku::Puzzle;
use solver::Solver;

pub fn build_complete_puzzle() -> Vec<Vec<isize>> {
    let grid = vec![vec![0;9];9];
    let puzzle = Puzzle::new(grid);
    let mut solver = Solver { puzzle: puzzle };

    solver.solve_puzzle()
}
