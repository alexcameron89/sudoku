use sudoku::Puzzle;

pub struct Solver {
    puzzle: Puzzle,
    solution: Vec<Vec<isize>>,
}

impl Solver {
    fn new(puzzle: Puzzle) -> Solver {
        let grid = puzzle.grid.clone();

        Solver {
            puzzle: puzzle,
            solution: grid,
        }
    }

    fn answer(&self) -> Vec<Vec<isize>> {
        return self.puzzle.grid.to_vec();
    }

    fn solve_puzzle(&self) {
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use sudoku::Puzzle;
    #[test]
    fn it_solves_an_incomplete_puzzle_correctly() {
        let grid = vec![
            vec![0,0,3,0,2,0,6,0,0],
            vec![9,0,0,3,0,5,0,0,1],
            vec![0,0,1,8,0,6,4,0,0],
            vec![0,0,8,1,0,2,9,0,0],
            vec![7,0,0,0,0,0,0,0,8],
            vec![0,0,6,7,0,8,2,0,0],
            vec![0,0,2,6,0,9,5,0,0],
            vec![8,0,0,2,0,3,0,0,9],
            vec![0,0,5,0,1,0,3,0,0]
        ];
        let correct_answer = vec![
            vec![4,8,3,9,2,1,6,5,7],
            vec![9,6,7,3,4,5,8,2,1],
            vec![2,5,1,8,7,6,4,9,3],
            vec![5,4,8,1,3,2,9,7,6],
            vec![7,2,9,5,6,4,1,3,8],
            vec![1,3,6,7,9,8,2,4,5],
            vec![3,7,2,6,8,9,5,1,4],
            vec![8,1,4,2,5,3,7,6,9],
            vec![6,9,5,4,1,7,3,8,2]
        ];
            let puzzle = Puzzle { grid: grid };
            let solver = Solver::new(puzzle);
            solver.solve_puzzle();
            assert_eq!(correct_answer, solver.answer());
    }
}
