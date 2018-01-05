extern crate rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use sudoku::{Puzzle, Square};
use validator::valid;

pub struct Solver {
    pub puzzle: Puzzle,
}

impl Solver {
    pub fn solve(puzzle: &Puzzle) -> Vec<Vec<isize>> {
        let mut solver = Solver { puzzle: puzzle.clone() };
        solver.solve_puzzle()
    }

    fn unsolved(&self) -> Vec<Square> {
        self.puzzle.unsolved_squares()
    }

    pub fn answer(&self) -> Vec<Vec<isize>> {
        return self.puzzle.grid.to_vec();
    }

    fn solve_square(&mut self, square: &Square, square_value: isize) {
        self.puzzle.set_square_value(square.row, square.column, square_value);
    }

    pub fn solve_puzzle(&mut self) -> Vec<Vec<isize>> {
        let unsolved = self.unsolved();
        if unsolved.len() == 0 {
            return self.answer()
        }
        let mut solved_any = false;
        for square in &unsolved {
            let possible_values = self.puzzle.possible_values(square.row, square.column);
            if possible_values.len() == 1 {
                &self.solve_square(&square, possible_values[0]);
                solved_any = true;
            }
        }

        if solved_any {
            &self.solve_puzzle();
        } else {
            let square = &self.choose_random_square(&unsolved);
            let values = &self.randomize(&square);
            for value in values {
                let mut new_puzzle = self.clone_and_set(&square.row, &square.column, value);
                new_puzzle.solve_puzzle();
                if valid(&new_puzzle.answer()) {
                    self.puzzle.grid = new_puzzle.puzzle.grid.to_vec();
                }
            }
        }

        return self.answer()
    }

    fn choose_random_square<'a>(&self, squares: &'a Vec<Square>) -> &'a Square {
        let mut rng = rand::thread_rng();
        let random_square = Range::new(0, squares.len());
        &squares[random_square.ind_sample(&mut rng)]
    }

    fn randomize(&self, square: &Square) -> Vec<isize> {
        let mut possible_values = self.puzzle.possible_values(square.row, square.column);
        rand::thread_rng().shuffle(&mut possible_values);

        possible_values
    }

    fn clone_and_set(&self, row: &i32, column: &i32, value: &isize) -> Solver {
        let mut new_puzzle = self.puzzle.clone();
        new_puzzle.set_square_value(row.abs(), column.abs(), value.abs());

        Solver {
            puzzle: new_puzzle,
        }
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use sudoku::{Puzzle,Square};
    use validator::valid;

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
            let solver = Solver {
                puzzle: Puzzle { grid: grid }
            };
            let unsolved = vec![
                Square { row: 0, column: 0, possible_values: vec![], set: false },
                Square { row: 1, column: 1, possible_values: vec![], set: false }
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
                let mut solver = Solver { puzzle: puzzle };
                assert_eq!(correct_answer, solver.solve_puzzle());
    }

    #[test]
    fn it_solves_an_easy_puzzle_correctly() {
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
                let mut solver = Solver { puzzle: puzzle };
                assert_eq!(correct_answer, solver.solve_puzzle());
    }

    #[test]
    fn it_solves_a_hard_puzzle_correctly() {
        let grid = vec![
            vec![0,0,3,0,0,0,0,0,0],
            vec![0,1,0,2,6,0,0,8,0],
            vec![2,8,0,0,0,4,9,0,0],
            vec![0,0,6,0,0,5,0,0,0],
            vec![9,0,8,0,0,0,5,0,3],
            vec![0,0,0,6,0,0,7,0,0],
            vec![0,0,9,7,0,0,0,4,8],
            vec![0,3,0,0,1,8,0,5,0],
            vec![0,0,0,0,0,0,2,0,0]
        ];
            let correct_answer = vec![
                vec![6,9,3,8,5,1,4,7,2],
                vec![4,1,7,2,6,9,3,8,5],
                vec![2,8,5,3,7,4,9,1,6],
                vec![1,7,6,9,3,5,8,2,4],
                vec![9,2,8,1,4,7,5,6,3],
                vec![3,5,4,6,8,2,7,9,1],
                vec![5,6,9,7,2,3,1,4,8],
                vec![7,3,2,4,1,8,6,5,9],
                vec![8,4,1,5,9,6,2,3,7]
            ];
                let puzzle = Puzzle { grid: grid };
                let mut solver = Solver { puzzle: puzzle };
                assert_eq!(correct_answer, solver.solve_puzzle());
    }
}
