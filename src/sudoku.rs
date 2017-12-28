pub struct Puzzle {
    pub grid: Vec<Vec<isize>>,
}

impl Puzzle {
    pub fn squares(&self) -> Vec<Square> {
        vec![
                Square { position: vec![0,0], possible_values: vec![4], set: false }
        ]
    }
}

#[derive(Debug)]
pub struct Square {
    pub position: Vec<i32>,
    pub possible_values: Vec<i32>,
    pub set: bool,
}

impl PartialEq for Square {
    fn eq(&self, other: &Square) -> bool {
        self.position == other.position &&
            self.possible_values == other.possible_values
    }
}
