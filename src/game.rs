use crate::models::board::Board;
use crate::models::cell::Cell;
use crate::models::player::Player;
use crate::utils::ai::best_next_move;
use crate::utils::ai::current_player;
use crate::utils::ai::game_over;
use crate::utils::ai::winning_player_on;
use crate::utils::graphics::clear_screen;
use crate::utils::graphics::render_board;
use crate::utils::graphics::render_current_player;
use crate::utils::graphics::render_empty_lines;
use crate::utils::graphics::render_stalemate;
use crate::utils::graphics::render_winning_player;
use crate::utils::input::ask_for_cell_position;
use crate::utils::input::ask_for_player_character;
use crate::utils::input::confirm;

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

    if game_over(board) {
        match winning_player_on(board) {
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
            board.set_cell_at(row_index, col_index, Cell::Marker(player));
        } else {
            take_turn(board, &player);
        }

        next_turn(board, player_character, first_player, against_computer);
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
