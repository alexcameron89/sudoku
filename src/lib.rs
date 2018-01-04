#![feature(test)]
extern crate rand;
extern crate test;

pub mod builder;
pub mod solver;
pub mod sudoku;
pub mod validator;

#[cfg(test)]
mod benchmarks {
    use solver::Solver;
    use sudoku::Puzzle;
    use builder::build_complete_puzzle;
    use test::Bencher;

    #[bench]
    fn bench_easy_puzzle(b: &mut Bencher) {
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
        let puzzle = Puzzle { grid: grid };
        b.iter(|| Solver::solve(&puzzle));
    }

    #[bench]
    fn bench_hard_puzzle(b: &mut Bencher) {
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
        let puzzle = Puzzle { grid: grid };
        let mut solver = Solver { puzzle: puzzle };
        b.iter(|| solver.solve_puzzle());
    }

    #[ignore] //This takes to long to benchmark
    #[bench]
    fn bench_building_puzzles(b: &mut Bencher) {
        b.iter(|| build_complete_puzzle());
    }
}
