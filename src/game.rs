use crate::models::board::Board;
use crate::models::cell::Cell;
use crate::models::player::Player;
use crate::utils::ai::best_next_move;
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

pub fn init() {
    let mut board = Board::new();

    clear_screen();
    render_empty_lines(1);
    let against_computer = confirm("Play against computer?", true);
    let player_character = ask_for_player_character("Wanna be X or O?");
    let computer_goes_first = against_computer && confirm("Can I go first?", true);
    let first_player = if computer_goes_first {
        player_character.opponent()
    } else {
        player_character.clone()
    };

    next_turn(
        &mut board,
        &player_character,
        &first_player,
        against_computer,
    );
}

pub fn next_turn(
    board: &mut Board,
    player_character: &Player,
    first_player: &Player,
    against_computer: bool,
) {
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

        if confirm("Would you like to play again?", true) {
            init()
        }
    } else {
        if against_computer && &player != player_character {
            let (_, row_index, col_index) = best_next_move(board, &player).unwrap();
            board.set_cell_at(row_index, col_index, Cell::Marker(player.clone()));
        } else {
            take_turn(board, &player);
        }

        next_turn(board, player_character, first_player, against_computer);
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

pub fn ask_for_player_character(prompt: &str) -> Player {
    match ask_for_character(prompt) {
        Ok('X') | Ok('x') => Player::X,
        Ok('O') | Ok('o') => Player::O,
        _ => {
            println!("That's not a valid choice! Try again.");
            ask_for_player_character(prompt)
        }
    }
}

pub fn confirm(prompt: &str, default: bool) -> bool {
    let choices = if default { "Y/n" } else { "y/N" };
    let prompt_with_yes_no = format!("{} ({})", prompt.trim(), choices);
    match ask_for_bool(prompt_with_yes_no.as_str(), true) {
        Ok(answer) => answer,
        _ => {
            println!("Valid choices are, like, 'yes' and 'no'. Get it? All right, try again.");
            confirm(prompt, default)
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
