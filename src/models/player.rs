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
}
