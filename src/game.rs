use crate::models::board::Board;
use crate::models::cell::Cell;
use crate::models::player::Player;
use crate::utils::graphics::clear_screen;
use crate::utils::graphics::render_board;
use crate::utils::graphics::render_current_player;
use crate::utils::graphics::render_empty_lines;
use crate::utils::input::ask_for_number;

pub fn run() {
    let mut board = Board::new();
    let mut turn_number = 0;

    loop {
        let player = if turn_number % 2 == 0 {
            Player::O
        } else {
            Player::X
        };

        clear_screen();

        render_empty_lines(1);
        render_current_player(&player);

        render_empty_lines(1);
        render_board(&board);

        render_empty_lines(1);
        take_turn(&mut board, player);

        if board.is_complete() {
            render_empty_lines(1);
            clear_screen();
            render_empty_lines(3);
            render_board(&board);
            render_empty_lines(1);
            match board.winner() {
                Some(winner) => println!("Player {} won!", winner.character()),
                None => println!("STALEMATE!"),
            }
            break;
        }

        turn_number += 1;
    }
}

pub fn ask_for_positional_index(prompt: &str) -> usize {
    match ask_for_number(prompt) {
        Ok(n) if 1 <= n && n <= 3 => (n - 1) as usize,
        _ => {
            println!("That's not a valid position! Try again.");
            ask_for_positional_index(prompt)
        }
    }
}

pub fn take_turn(board: &mut Board, player: Player) {
    // let row_index =
    let row_index = ask_for_positional_index("Row: ");
    let col_index = ask_for_positional_index("Col: ");
    let cell = board.get_cell_at(row_index, col_index);

    match cell {
        Cell::Empty => board.set_cell_at(row_index, col_index, Cell::Marker(player)),
        Cell::Marker(_) => {
            println!("Player {} has already taken this spot.", cell.character());
            take_turn(board, player)
        }
    }
}
