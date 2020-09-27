use crate::utils::general::passes_or;
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

pub fn ask_for_string_where(
    prompt: &str,
    condition: fn(&String) -> bool,
) -> Result<String, io::Error> {
    ask_for_string(prompt)
        .and_then(|input| passes_or(input, condition, invalid_input("String condition not met")))
}

pub fn ask_for_number(prompt: &str) -> Result<isize, io::Error> {
    ask_for_string(prompt).and_then(|input| {
        input
            .parse::<isize>()
            .or(Err(invalid_input("Not a number")))
    })
}

pub fn ask_for_number_where(
    prompt: &str,
    condition: fn(&isize) -> bool,
) -> Result<isize, io::Error> {
    ask_for_number(prompt)
        .and_then(|num| passes_or(num, condition, invalid_input("Numeric condition not met")))
}

pub fn ask_for_character(prompt: &str) -> Result<char, io::Error> {
    // ask_for_string(prompt).and_then(|input| match input.chars().next() {
    //     Some(character) => Ok(character),
    //     None => Err(invalid_input("Not a character")),
    // })
    ask_for_string(prompt)
        .and_then(|input| input.chars().next().ok_or(invalid_input("Not a character")))
}

pub fn ask_for_char_where(prompt: &str, condition: fn(&char) -> bool) -> Result<char, io::Error> {
    ask_for_character(prompt).and_then(|character| {
        passes_or(
            character,
            condition,
            invalid_input("Character condition not met"),
        )
    })
}

pub fn ask_for_bool(prompt: &str, default: bool) -> Result<bool, io::Error> {
    let yes_regex = Regex::new(YES_PATTERN).unwrap();
    let no_regex = Regex::new(NO_PATTERN).unwrap();

    ask_for_string(prompt).and_then(|input| {
        if yes_regex.is_match(&input) {
            Ok(true)
        } else if no_regex.is_match(&input) {
            Ok(false)
        } else {
            Ok(default)
        }
    })
}

pub fn ask_for_alpha_num(prompt: &str) -> Result<(String, usize), io::Error> {
    let alpha_num_regex = Regex::new(r"^\s*([A-Za-z]+)\W*(\d+)\s*$").unwrap();
    ask_for_string(prompt).and_then(|input| {
        alpha_num_regex
            .captures(&input)
            .ok_or(invalid_input("Not a '<letter><number>'"))
            .and_then(|captures| {
                let alpha = String::from(&captures[1]);
                let num = captures[2].parse::<usize>().unwrap();
                Ok((alpha, num))
            })
    })
}
