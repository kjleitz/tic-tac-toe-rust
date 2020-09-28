#[derive(Debug, PartialEq, Clone)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn character(&self) -> &str {
        match self {
            Player::X => "X",
            Player::O => "O",
        }
    }

    pub fn opponent(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}
