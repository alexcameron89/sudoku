extern crate sudoku;

use sudoku::validator::valid;
use sudoku::builder;

#[ignore]
#[test]
fn it_builds_a_valid_puzzle() {
    let puzzle = builder::build_complete_puzzle();
    assert!(valid(&puzzle));
}
