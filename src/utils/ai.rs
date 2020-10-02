use crate::models::board::Board;
use crate::models::cell::Cell;
use crate::models::player::Player;

pub fn all_are_player(cells: &[&Cell], player: &Player) -> bool {
    cells.iter().all(|cell| match cell {
        Cell::Marker(p) => p == player,
        Cell::Empty => false,
    })
}

pub fn same_player_in(cells: &[&Cell]) -> Option<Player> {
    if all_are_player(cells, &Player::X) {
        Some(Player::X)
    } else if all_are_player(cells, &Player::O) {
        Some(Player::O)
    } else {
        None
    }
}

pub fn win_arrangements(board: &Board) -> Vec<Vec<(&Cell, usize, usize)>> {
    vec![
        board.get_cell_position_rows(),
        board.get_cell_position_cols(),
        board.get_cell_position_crosses(),
    ]
    .iter()
    .flat_map(|cell_position_collections| cell_position_collections.to_vec())
    .collect()
}

pub fn winning_player_on(board: &Board) -> Option<Player> {
    win_arrangements(board).iter().find_map(|cell_positions| {
        let cells = Board::cell_positions_to_cells(cell_positions);
        same_player_in(&cells)
    })
}

pub fn board_full(board: &Board) -> bool {
    board.get_cells().iter().all(|cell| match cell {
        Cell::Marker(_) => true,
        Cell::Empty => false,
    })
}

#[allow(dead_code)]
pub fn board_empty(board: &Board) -> bool {
    board.get_cells().iter().all(|cell| match cell {
        Cell::Marker(_) => false,
        Cell::Empty => true,
    })
}

pub fn game_over(board: &Board) -> bool {
    board_full(board) || winning_player_on(board).is_some()
}

pub fn current_player(board: &Board, starting_player: &Player) -> Player {
    let cells: Vec<&Cell> = board.get_cells();

    let (first_player, second_player) = if starting_player == &Player::X {
        (Player::X, Player::O)
    } else {
        (Player::O, Player::X)
    };

    let mut first_player_turns = 0;
    let mut second_player_turns = 0;

    cells.iter().for_each(|cell| {
        if let Cell::Marker(player) = cell {
            if player == &first_player {
                first_player_turns += 1;
            } else {
                second_player_turns += 1;
            }
        }
    });

    if first_player_turns > second_player_turns {
        second_player
    } else {
        first_player
    }
}

pub fn potential_arrangements<'a>(
    board: &'a Board,
    for_player: &Player,
    markers_placed: usize,
) -> Vec<Vec<(&'a Cell, usize, usize)>> {
    win_arrangements(board)
        .iter()
        .filter_map(|cell_positions| {
            let (player_count, empty_count) =
                cell_positions
                    .iter()
                    .fold(
                        (0, 0),
                        |(player_count, empty_count), cell_position| match cell_position {
                            (Cell::Marker(player), _, _) if player == for_player => {
                                (player_count + 1, empty_count)
                            }
                            (Cell::Empty, _, _) => (player_count, empty_count + 1),
                            _ => (player_count, empty_count),
                        },
                    );

            let desired_empty_count = cell_positions.len() - markers_placed;
            if player_count == markers_placed && empty_count == desired_empty_count {
                Some(cell_positions.to_vec())
            } else {
                None
            }
        })
        .collect()
}

pub fn potential_win_setup_arrangements<'a>(
    board: &'a Board,
    for_player: &Player,
) -> Vec<Vec<(&'a Cell, usize, usize)>> {
    potential_arrangements(board, for_player, 1)
}

pub fn potential_win_arrangements<'a>(
    board: &'a Board,
    for_player: &Player,
) -> Vec<Vec<(&'a Cell, usize, usize)>> {
    potential_arrangements(board, for_player, 2)
}

pub fn potential_win_setup_moves(board: &Board, for_player: &Player) -> Vec<(Cell, usize, usize)> {
    potential_win_setup_arrangements(board, for_player)
        .iter()
        .flat_map(|cell_positions| {
            cell_positions
                .iter()
                .filter_map(|cell_position| match cell_position {
                    (Cell::Empty, row_index, col_index) => {
                        Some((Cell::Empty, *row_index, *col_index))
                    }
                    _ => None,
                })
                .collect::<Vec<(Cell, usize, usize)>>()
        })
        .collect()
}

pub fn potential_win_moves(board: &Board, for_player: &Player) -> Vec<(Cell, usize, usize)> {
    potential_win_arrangements(board, for_player)
        .iter()
        .filter_map(|cell_positions| {
            cell_positions
                .iter()
                .find_map(|cell_position| match cell_position {
                    (Cell::Empty, row_index, col_index) => {
                        Some((Cell::Empty, *row_index, *col_index))
                    }
                    _ => None,
                })
        })
        .collect()
}

pub fn potential_fork_moves(board: &Board, for_player: &Player) -> Vec<(Cell, usize, usize)> {
    let mut potential_forks_scored = potential_win_setup_moves(board, for_player)
        .iter()
        .filter_map(|(cell, row_index, col_index)| {
            let mut imagined_board = Board::from(board);
            imagined_board.set_cell_at(*row_index, *col_index, Cell::Marker(for_player.clone()));
            let score = potential_win_moves(&imagined_board, for_player).len();
            if score > 1 {
                Some((cell.clone(), *row_index, *col_index, score))
            } else {
                None
            }
        })
        .collect::<Vec<(Cell, usize, usize, usize)>>();

    potential_forks_scored.sort_by(|a, b| (b.3).cmp(&a.3));
    potential_forks_scored
        .iter()
        .map(|(cell, row_index, col_index, _)| (cell.clone(), *row_index, *col_index))
        .collect()
}

pub fn potential_opposite_corner_moves(
    board: &Board,
    for_player: &Player,
) -> Vec<(Cell, usize, usize)> {
    board
        .get_corner_cell_positions()
        .iter()
        .filter_map(|(cell, row_index, col_index)| {
            let opponent = for_player.opponent();
            match cell {
                Cell::Marker(resident) if resident == &opponent => {
                    Some((*cell, *row_index, *col_index))
                }
                _ => None,
            }
        })
        .filter_map(|cell_position| {
            let (cell, row_index, col_index) =
                board.get_corner_cell_position_opposite(cell_position);

            match cell {
                Cell::Empty => Some((cell.clone(), row_index, col_index)),
                _ => None,
            }
        })
        .collect()
}

pub fn potential_empty_corner_moves(board: &Board) -> Vec<(Cell, usize, usize)> {
    board
        .get_corner_cell_positions()
        .iter()
        .filter_map(|(cell, row_index, col_index)| match cell {
            Cell::Empty => Some((Cell::Empty, *row_index, *col_index)),
            _ => None,
        })
        .collect()
}

pub fn potential_empty_side_moves(board: &Board) -> Vec<(Cell, usize, usize)> {
    board
        .get_side_cell_positions()
        .iter()
        .filter_map(|(cell, row_index, col_index)| match cell {
            Cell::Empty => Some((Cell::Empty, *row_index, *col_index)),
            _ => None,
        })
        .collect()
}

pub fn potential_empty_moves(board: &Board) -> Vec<(Cell, usize, usize)> {
    board
        .get_cell_positions()
        .iter()
        .filter_map(|(cell, row_index, col_index)| match cell {
            Cell::Empty => Some((Cell::Empty, *row_index, *col_index)),
            _ => None,
        })
        .collect()
}

pub fn first_cell_position_in(cells: &[(Cell, usize, usize)]) -> Option<(Cell, usize, usize)> {
    cells
        .iter()
        .next()
        .map(|(cell, row_index, col_index)| (cell.clone(), *row_index, *col_index))
}

pub fn potential_win_setup_move(
    board: &Board,
    for_player: &Player,
) -> Option<(Cell, usize, usize)> {
    first_cell_position_in(&potential_win_setup_moves(board, for_player))
}

pub fn potential_win_move(board: &Board, for_player: &Player) -> Option<(Cell, usize, usize)> {
    first_cell_position_in(&potential_win_moves(board, for_player))
}

pub fn best_potential_fork_move(
    board: &Board,
    for_player: &Player,
) -> Option<(Cell, usize, usize)> {
    first_cell_position_in(&potential_fork_moves(board, for_player))
}

pub fn potential_center_move(board: &Board) -> Option<(Cell, usize, usize)> {
    match board.get_center_cell_position() {
        (Cell::Empty, row_index, col_index) => Some((Cell::Empty, row_index, col_index)),
        _ => None,
    }
}

pub fn potential_opposite_corner_move(
    board: &Board,
    for_player: &Player,
) -> Option<(Cell, usize, usize)> {
    first_cell_position_in(&potential_opposite_corner_moves(board, for_player))
}

pub fn potential_empty_corner_move(board: &Board) -> Option<(Cell, usize, usize)> {
    first_cell_position_in(&potential_empty_corner_moves(board))
}

pub fn potential_empty_side_move(board: &Board) -> Option<(Cell, usize, usize)> {
    first_cell_position_in(&potential_empty_side_moves(board))
}

pub fn potential_empty_move(board: &Board) -> Option<(Cell, usize, usize)> {
    first_cell_position_in(&potential_empty_moves(board))
}

pub fn best_next_move(board: &Board, for_player: &Player) -> Option<(Cell, usize, usize)> {
    None.or_else(|| potential_win_move(board, for_player))
        .or_else(|| potential_win_move(board, &for_player.opponent()))
        .or_else(|| best_potential_fork_move(board, for_player))
        .or_else(|| best_potential_fork_move(board, &for_player.opponent()))
        .or_else(|| potential_win_setup_move(board, for_player))
        .or_else(|| potential_win_setup_move(board, &for_player.opponent()))
        .or_else(|| potential_center_move(board))
        .or_else(|| potential_opposite_corner_move(board, for_player))
        .or_else(|| potential_empty_corner_move(board))
        .or_else(|| potential_empty_side_move(board))
        .or_else(|| potential_empty_move(board))
}
