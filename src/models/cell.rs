use crate::models::player::Player;

#[derive(Debug)]
pub enum Cell {
    Marker(Player),
    Empty,
}

impl Cell {
    pub fn character(&self) -> &str {
        match self {
            Cell::Marker(player) => player.character(),
            Cell::Empty => " ",
        }
    }
}
