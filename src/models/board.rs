use crate::models::cell::Cell;
use crate::models::player::Player;
use crate::models::row::Row;
use crate::utils::ai;

#[derive(Debug)]
pub struct Board {
    pub rows: [Row; 3],
}

impl Board {
    pub fn new() -> Self {
        Self {
            rows: [Row::new(), Row::new(), Row::new()],
        }
    }

    pub fn get_rows(&self) -> Vec<Vec<&Cell>> {
        self.rows
            .iter()
            .map(|row| row.cells.iter().collect())
            .collect()
    }

    pub fn get_cols(&self) -> Vec<Vec<&Cell>> {
        self.rows[0]
            .cells
            .iter()
            .enumerate()
            .map(|(i, _)| self.rows.iter().map(|row| &row.cells[i]).collect())
            .collect()
    }

    pub fn get_crosses(&self) -> Vec<Vec<&Cell>> {
        vec![
            vec![
                &self.rows[0].cells[0],
                &self.rows[1].cells[1],
                &self.rows[2].cells[2],
            ],
            vec![
                &self.rows[0].cells[2],
                &self.rows[1].cells[1],
                &self.rows[2].cells[0],
            ],
        ]
    }

    pub fn get_cells(&self) -> Vec<&Cell> {
        self.get_rows()
            .iter()
            .flat_map(|cells| cells.to_vec())
            .collect()
    }

    pub fn get_cell_at(&self, row_index: usize, col_index: usize) -> &Cell {
        &self.rows[row_index].cells[col_index]
    }

    pub fn set_cell_at(&mut self, row_index: usize, col_index: usize, cell: Cell) {
        self.rows[row_index].cells[col_index] = cell;
    }

    pub fn board_full(&self) -> bool {
        ai::board_full(self)
    }

    pub fn winner(&self) -> Option<Player> {
        ai::winning_player_on(self)
    }

    pub fn is_complete(&self) -> bool {
        ai::game_over(self)
    }
}
