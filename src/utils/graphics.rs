use crate::models::board::Board;
use crate::models::player::Player;
use std::io::{self, Write};

pub fn clear_screen() {
    print!("{clear_char}[2J{clear_char}[1;1H", clear_char = 27 as char);
    io::stdout().flush().unwrap();
}

pub fn render_empty_lines(count: usize) {
    for _ in 0..count {
        println!("");
    }
}

pub fn render_current_player(player: &Player) {
    println!("Current player: {}", player.character());
}

pub fn render_board(board: &Board) {
    println!("   ,---,---,---,");

    for (index, row) in board.rows.iter().enumerate() {
        if index > 0 {
            println!("   |---+---+---|");
        }

        let markers: Vec<&str> = row.cells.iter().map(|cell| cell.character()).collect();
        let row_letter = match index {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            _ => '?', // should never happen
        };

        println!(
            " {} | {} | {} | {} |",
            row_letter, markers[0], markers[1], markers[2]
        );
    }

    println!("   '---'---'---'");
    println!("     1   2   3");
}

pub fn render_winning_player(player: &Player) {
    println!("Player {} won!", player.character());
}

pub fn render_stalemate() {
    println!("STALEMATE!");
}
