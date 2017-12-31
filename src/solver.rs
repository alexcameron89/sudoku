use sudoku::{Puzzle, Square};

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

    fn unsolved(&self) -> Vec<Square> {
        self.puzzle.squares().into_iter().filter(|ref square| !square.set).collect::<Vec<Square>>()
    }

    fn answer(&self) -> Vec<Vec<isize>> {
        return self.puzzle.grid.to_vec();
    }

    fn solve_square(&mut self, square: Square) {
        let square_value = square.possible_values[0];
        self.puzzle.set_square_value(square.row, square.column, square_value);
    }

    fn solve_puzzle(&mut self) {
        let mut solved_any = false;
        for square in self.unsolved() {
            if square.possible_values.len() == 1 {
                &self.solve_square(square);
                solved_any = true;
            }
        }

        if solved_any {
            &self.solve_puzzle();
        }
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use sudoku::{Puzzle,Square};

    #[test]
    fn it_keeps_track_of_unsolved_squares() {
        let grid = vec![
            vec![0,8,3,9,2,1,6,5,7],
            vec![9,0,7,3,4,5,8,2,1],
            vec![2,5,1,8,7,6,4,9,3],
            vec![5,4,8,1,3,2,9,7,6],
            vec![7,2,9,5,6,4,1,3,8],
            vec![1,3,6,7,9,8,2,4,5],
            vec![3,7,2,6,8,9,5,1,4],
            vec![8,1,4,2,5,3,7,6,9],
            vec![6,9,5,4,1,7,3,8,2]
        ];
            let mut solver = Solver::new(Puzzle { grid: grid });
            let unsolved = vec![
                Square { row: 0, column: 0, possible_values: vec![4], set: false },
                Square { row: 1, column: 1, possible_values: vec![6], set: false }
            ];
            assert_eq!(unsolved, solver.unsolved());
    }

    #[test]
    fn it_solves_squares_with_only_one_option() {
        let grid = vec![
            vec![0,8,3,9,2,1,6,5,7],
            vec![0,6,7,3,4,5,8,2,1],
            vec![0,5,1,8,7,6,4,9,3],
            vec![0,4,8,1,3,2,9,7,6],
            vec![0,2,9,5,6,4,1,3,8],
            vec![0,3,6,7,9,8,2,4,5],
            vec![0,7,2,6,8,9,5,1,4],
            vec![0,1,4,2,5,3,7,6,9],
            vec![0,9,5,4,1,7,3,8,2]
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
            let mut solver = Solver::new(puzzle);
            solver.solve_puzzle();
            assert_eq!(correct_answer, solver.answer());
    }

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
            let mut solver = Solver::new(puzzle);
            solver.solve_puzzle();
            assert_eq!(correct_answer, solver.answer());
    }
}