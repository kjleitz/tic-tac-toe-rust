use crate::models::board::Board;
use crate::models::cell::Cell;
use crate::models::player::Player;

pub fn all_are_player(cells: &Vec<&Cell>, player: &Player) -> bool {
    cells.iter().all(|cell| match cell {
        Cell::Marker(p) => p == player,
        Cell::Empty => false,
    })
}

pub fn same_player_in(cells: &Vec<&Cell>) -> Option<Player> {
    if all_are_player(cells, &Player::X) {
        Some(Player::X)
    } else if all_are_player(cells, &Player::O) {
        Some(Player::O)
    } else {
        None
    }
}

pub fn win_arrangements(board: &Board) -> Vec<Vec<&Cell>> {
    vec![board.get_rows(), board.get_cols(), board.get_crosses()]
        .iter()
        .flat_map(|cells| cells.to_vec())
        .collect()
}

pub fn winning_player_on(board: &Board) -> Option<Player> {
    win_arrangements(board)
        .iter()
        .find_map(|cells| same_player_in(cells))
}

pub fn board_full(board: &Board) -> bool {
    board.rows.iter().all(|row| {
        row.cells.iter().all(|cell| match cell {
            Cell::Marker(_) => true,
            Cell::Empty => false,
        })
    })
}

pub fn board_empty(board: &Board) -> bool {
    board.rows.iter().all(|row| {
        row.cells.iter().all(|cell| match cell {
            Cell::Marker(_) => false,
            Cell::Empty => true,
        })
    })
}

pub fn game_over(board: &Board) -> bool {
    board_full(board)
        || match winning_player_on(board) {
            Some(_) => true,
            None => false,
        }
}

pub fn current_player(board: &Board, starting_player: &Player) -> Player {
    let cells = board.get_cells();

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

// pub fn best_next_move(board: &Board) -> [usize; 2] {

// }
