use crate::models::cell::Cell;

#[derive(Debug)]
pub struct Row {
    pub cells: [Cell; 3],
}

impl Row {
    pub fn new() -> Self {
        Self {
            cells: [Cell::Empty, Cell::Empty, Cell::Empty],
        }
    }
}
