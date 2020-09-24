use crate::models::cell::Cell;
use crate::models::player::Player;
use crate::models::row::Row;

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
            .map(|row| row.cols.iter().collect())
            .collect()
    }

    pub fn get_cols(&self) -> Vec<Vec<&Cell>> {
        self.rows[0]
            .cols
            .iter()
            .enumerate()
            .map(|(i, _)| self.rows.iter().map(|row| &row.cols[i]).collect())
            .collect()
    }

    pub fn get_crosses(&self) -> Vec<Vec<&Cell>> {
        vec![
            vec![
                &self.rows[0].cols[0],
                &self.rows[1].cols[1],
                &self.rows[2].cols[2],
            ],
            vec![
                &self.rows[0].cols[2],
                &self.rows[1].cols[1],
                &self.rows[2].cols[0],
            ],
        ]
    }

    pub fn get_cell_at(&self, row_index: usize, col_index: usize) -> &Cell {
        &self.rows[row_index].cols[col_index]
    }

    pub fn set_cell_at(&mut self, row_index: usize, col_index: usize, cell: Cell) {
        self.rows[row_index].cols[col_index] = cell;
    }

    pub fn is_full(&self) -> bool {
        self.rows.iter().all(|row| {
            row.cols.iter().all(|cell| match cell {
                Cell::Marker(_) => true,
                Cell::Empty => false,
            })
        })
    }

    pub fn winner(&self) -> Option<Player> {
        let all_x = |cells: &Vec<&Cell>| -> bool {
            cells.iter().all(|cell| match cell {
                Cell::Marker(Player::X) => true,
                _ => false,
            })
        };

        let all_o = |cells: &Vec<&Cell>| -> bool {
            cells.iter().all(|cell| match cell {
                Cell::Marker(Player::O) => true,
                _ => false,
            })
        };

        let all_same_player = |cells: &Vec<&Cell>| -> Option<Player> {
            if all_x(cells) {
                Some(Player::X)
            } else if all_o(cells) {
                Some(Player::O)
            } else {
                None
            }
        };

        if let Some(player) = self
            .get_rows()
            .iter()
            .find_map(|cells| all_same_player(cells))
        {
            return Some(player);
        };

        if let Some(player) = self.get_cols().iter().find_map(all_same_player) {
            return Some(player);
        };

        if let Some(player) = self.get_crosses().iter().find_map(all_same_player) {
            return Some(player);
        };

        None
    }

    pub fn is_complete(&self) -> bool {
        if self.is_full() {
            true
        } else if let Some(_) = self.winner() {
            true
        } else {
            false
        }
    }
}
