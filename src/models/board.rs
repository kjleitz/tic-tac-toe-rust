use crate::models::cell::Cell;
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

    pub fn from(board: &Board) -> Self {
        Self {
            rows: board.rows.clone(),
        }
    }

    pub fn cell_positions_to_cells<'a>(
        cell_positions: &[(&'a Cell, usize, usize)],
    ) -> Vec<&'a Cell> {
        cell_positions
            .iter()
            .map(|(cell, _row_index, _col_index)| *cell)
            .collect()
    }

    pub fn get_cell_positions(&self) -> Vec<(&Cell, usize, usize)> {
        self.rows
            .iter()
            .enumerate()
            .flat_map(|(row_index, row)| {
                row.cells
                    .iter()
                    .enumerate()
                    .map(|(col_index, cell)| (cell, row_index, col_index))
                    .collect::<Vec<(&Cell, usize, usize)>>()
            })
            .collect()
    }

    pub fn get_cells(&self) -> Vec<&Cell> {
        Board::cell_positions_to_cells(&self.get_cell_positions())
    }

    pub fn get_cell_position_rows(&self) -> Vec<Vec<(&Cell, usize, usize)>> {
        self.get_cell_positions().iter().fold(
            vec![vec![(&Cell::Empty, 0, 0); 3]; 3],
            |mut rows, (cell, row_index, col_index)| {
                rows[*row_index][*col_index] = (cell, *row_index, *col_index);
                rows
            },
        )
    }

    pub fn get_cell_position_cols(&self) -> Vec<Vec<(&Cell, usize, usize)>> {
        self.get_cell_positions().iter().fold(
            vec![vec![(&Cell::Empty, 0, 0); 3]; 3],
            |mut cols, (cell, row_index, col_index)| {
                cols[*col_index][*row_index] = (cell, *row_index, *col_index);
                cols
            },
        )
    }

    pub fn get_cell_position_crosses(&self) -> Vec<Vec<(&Cell, usize, usize)>> {
        vec![
            vec![
                (&self.rows[0].cells[0], 0, 0),
                (&self.rows[1].cells[1], 1, 1),
                (&self.rows[2].cells[2], 2, 2),
            ],
            vec![
                (&self.rows[0].cells[2], 0, 2),
                (&self.rows[1].cells[1], 1, 1),
                (&self.rows[2].cells[0], 2, 0),
            ],
        ]
    }

    pub fn get_center_cell_position(&self) -> (&Cell, usize, usize) {
        (&self.rows[1].cells[1], 1, 1)
    }

    pub fn get_corner_cell_positions(&self) -> Vec<(&Cell, usize, usize)> {
        let rows = self.get_cell_position_rows();
        let first_row = rows.first().unwrap();
        let last_row = rows.last().unwrap();
        vec![
            *first_row.first().unwrap(),
            *first_row.last().unwrap(),
            *last_row.first().unwrap(),
            *last_row.last().unwrap(),
        ]
    }

    pub fn get_corner_cell_position_opposite(
        &self,
        cell_position: (&Cell, usize, usize),
    ) -> (&Cell, usize, usize) {
        let rows = self.get_cell_position_rows();

        let opposite_row = if cell_position.1 == 0 {
            rows.last().unwrap()
        } else {
            rows.first().unwrap()
        };

        if cell_position.2 == 0 {
            *opposite_row.last().unwrap()
        } else {
            *opposite_row.first().unwrap()
        }
    }

    pub fn get_side_cell_positions(&self) -> Vec<(&Cell, usize, usize)> {
        vec![
            (&self.rows[0].cells[1], 0, 1),
            (&self.rows[1].cells[0], 1, 0),
            (&self.rows[1].cells[2], 1, 2),
            (&self.rows[2].cells[1], 2, 1),
        ]
    }

    pub fn get_cell_at(&self, row_index: usize, col_index: usize) -> &Cell {
        &self.rows[row_index].cells[col_index]
    }

    pub fn set_cell_at(&mut self, row_index: usize, col_index: usize, cell: Cell) {
        self.rows[row_index].cells[col_index] = cell;
    }
}
