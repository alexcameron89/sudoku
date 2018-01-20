extern crate sudoku;

use sudoku::solver;
use sudoku::sudoku::Puzzle;

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
    assert_eq!(correct_answer, solver::solve(&puzzle));
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
    assert_eq!(correct_answer, solver::solve(&puzzle));
}

#[test]
fn it_solves_an_insane_puzzle_correctly() {
    let grid = vec![
        vec![3,0,0,7,0,0,0,0,0],
        vec![9,0,0,0,0,8,0,7,4],
        vec![0,0,0,3,0,5,0,0,6],
        vec![1,0,0,0,0,0,0,4,2],
        vec![0,7,0,0,0,0,0,9,0],
        vec![6,8,0,0,0,0,0,0,5],
        vec![7,0,0,5,0,6,0,0,0],
        vec![4,1,0,8,0,0,0,0,7],
        vec![0,0,0,0,0,7,0,0,9]
    ];
    let correct_answer = vec![
        vec![3,6,2,7,9,4,5,8,1],
        vec![9,5,1,2,6,8,3,7,4],
        vec![8,4,7,3,1,5,9,2,6],
        vec![1,9,5,6,8,3,7,4,2],
        vec![2,7,3,4,5,1,6,9,8],
        vec![6,8,4,9,7,2,1,3,5],
        vec![7,2,9,5,4,6,8,1,3],
        vec![4,1,6,8,3,9,2,5,7],
        vec![5,3,8,1,2,7,4,6,9]
    ];
    let puzzle = Puzzle { grid: grid };
    assert_eq!(correct_answer, solver::solve(&puzzle));
}

