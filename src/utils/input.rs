use std::io::{self, Write};

pub fn ask_for_number(prompt: &str) -> Result<isize, &str> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<isize>() {
        Ok(n) => Ok(n),
        _ => Err("Invalid number"),
    }
}
