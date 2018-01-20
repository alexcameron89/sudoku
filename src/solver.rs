extern crate rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use sudoku::{Puzzle, Square};
use validator::valid;

pub struct Solver {
    pub puzzle: Puzzle,
}

pub fn solve(puzzle: &Puzzle) -> Vec<Vec<isize>> {
    let mut solver = Solver { puzzle: puzzle.clone() };
    solver.solve_puzzle()
}

impl Solver {
    pub fn answer(&self) -> Vec<Vec<isize>> {
        return self.puzzle.grid.to_vec();
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
                    break;
                }
            }
        }

        return self.answer()
    }

    fn unsolved(&self) -> Vec<Square> {
        self.puzzle.unsolved_squares()
    }

    fn solve_square(&mut self, square: &Square, square_value: isize) {
        self.puzzle.set_square_value(square.row, square.column, square_value);
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
    use super::*;
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
}
