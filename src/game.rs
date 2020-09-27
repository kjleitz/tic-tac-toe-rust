use crate::models::board::Board;
use crate::models::cell::Cell;
use crate::models::player::Player;
use crate::utils::ai::current_player;
use crate::utils::graphics::clear_screen;
use crate::utils::graphics::render_board;
use crate::utils::graphics::render_current_player;
use crate::utils::graphics::render_empty_lines;
use crate::utils::graphics::render_stalemate;
use crate::utils::graphics::render_winning_player;
use crate::utils::input::ask_for_alpha_num;
use crate::utils::input::ask_for_bool;
use crate::utils::input::ask_for_character;
// use crate::utils::input::ask_for_number_where;

pub fn init() {
    let mut board = Board::new();

    clear_screen();
    render_empty_lines(1);
    let first_player = ask_for_first_player("Wanna be X or O?");

    next_turn(&mut board, &first_player);
}

pub fn next_turn(board: &mut Board, first_player: &Player) {
    let player = current_player(&board, first_player);

    clear_screen();
    render_empty_lines(1);
    render_current_player(&player);
    render_empty_lines(1);
    render_board(&board);
    render_empty_lines(1);

    if board.is_complete() {
        match board.winner() {
            Some(winner) => render_winning_player(&winner),
            None => render_stalemate(),
        }

        render_empty_lines(1);
        if confirm("Would you like to play again? (Y/n)") {
            init()
        }
    } else {
        take_turn(board, &player);
        next_turn(board, first_player);
    }
}

pub fn ask_for_cell_position(prompt: &str) -> (usize, usize) {
    if let Ok((alpha, num)) = ask_for_alpha_num(prompt) {
        if num == 0 || num > 3 {
            println!("{} is not a column. Try again!", num);
            return ask_for_cell_position(prompt);
        }

        let col_index = num - 1;

        let row_index = match alpha.as_str() {
            "A" | "a" => 0,
            "B" | "b" => 1,
            "C" | "c" => 2,
            _ => 99,
        };

        if row_index == 99 {
            println!("{} is not a row. Try again!", alpha);
            return ask_for_cell_position(prompt);
        }

        (row_index, col_index)
    } else {
        println!("That's not a valid position! Try again.");
        ask_for_cell_position(prompt)
    }
}

pub fn ask_for_first_player(prompt: &str) -> Player {
    match ask_for_character(prompt) {
        Ok('X') | Ok('x') => Player::X,
        Ok('O') | Ok('o') => Player::O,
        _ => {
            println!("That's not a valid choice! Try again.");
            ask_for_first_player(prompt)
        }
    }
}

pub fn confirm(prompt: &str) -> bool {
    match ask_for_bool(prompt, true) {
        Ok(answer) => answer,
        _ => {
            println!("Valid choices are, like, 'yes' and 'no'. Get it? All right, try again.");
            confirm(prompt)
        }
    }
}

pub fn take_turn(board: &mut Board, player: &Player) {
    let (row_index, col_index) = ask_for_cell_position("Which cell? (e.g., A1, C2, etc.)");
    let cell = board.get_cell_at(row_index, col_index);

    match cell {
        Cell::Empty => board.set_cell_at(row_index, col_index, Cell::Marker(player.clone())),
        Cell::Marker(_) => {
            println!("Player {} has already taken this spot.", cell.character());
            take_turn(board, player)
        }
    }
}
