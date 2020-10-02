use crate::models::player::Player;
use regex::Regex;
use std::io::{self, Write};

const YES_PATTERN: &str = r"(?i)^\s*(y+|y+e+s+|y+e+a+h+|y+e+p+|y+u+p+|y+e+|y+a+r+|m+h+m+|true|1)\s*(pls|please|thx|thanks|man|dude|dawg|bro|bruh)?\s*$";
const NO_PATTERN: &str = r"(?i)^\s*(n+|n+o+|n+o+p+e+|n+a+h+|false|0)\s*(pls|please|thx|thanks|man|dude|dawg|bro|bruh)?\s*$";

pub fn invalid_input(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

pub fn ask_for_string(prompt: &str) -> Result<String, io::Error> {
    print!("{} ", prompt.trim());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(String::from(input.trim())),
        Err(e) => Err(e),
    }
}

#[allow(dead_code)]
pub fn ask_for_number(prompt: &str) -> Result<isize, io::Error> {
    ask_for_string(prompt).and_then(|input| {
        input
            .parse::<isize>()
            .map_err(|_| invalid_input("Not a number"))
    })
}

pub fn ask_for_character(prompt: &str) -> Result<char, io::Error> {
    ask_for_string(prompt).and_then(|input| {
        input
            .chars()
            .next()
            .ok_or_else(|| invalid_input("Not a character"))
    })
}

pub fn ask_for_bool(prompt: &str, default: bool) -> Result<bool, io::Error> {
    let yes_regex = Regex::new(YES_PATTERN).unwrap();
    let no_regex = Regex::new(NO_PATTERN).unwrap();

    ask_for_string(prompt).map(|input| {
        if yes_regex.is_match(&input) {
            true
        } else if no_regex.is_match(&input) {
            false
        } else {
            default
        }
    })
}

pub fn ask_for_alpha_num(prompt: &str) -> Result<(String, usize), io::Error> {
    let alpha_num_regex = Regex::new(r"^\s*([A-Za-z]+)\W*(\d+)\s*$").unwrap();
    ask_for_string(prompt).and_then(|input| {
        alpha_num_regex
            .captures(&input)
            .ok_or_else(|| invalid_input("Not a '<letter><number>'"))
            .map(|captures| {
                let alpha = String::from(&captures[1]);
                let num = captures[2].parse::<usize>().unwrap();
                (alpha, num)
            })
    })
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
