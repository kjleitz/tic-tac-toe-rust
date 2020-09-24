use crate::models::cell::Cell;

#[derive(Debug)]
pub struct Row {
    pub cols: [Cell; 3],
}

impl Row {
    pub fn new() -> Self {
        Self {
            cols: [Cell::Empty, Cell::Empty, Cell::Empty],
        }
    }
}
