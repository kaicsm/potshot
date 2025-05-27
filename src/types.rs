use macroquad::prelude::Vec2;

// --- Estruturas de Input ---
#[derive(Default, Debug, Clone, Copy)]
pub struct InputState {
    pub is_dragging: bool,
    pub start_pos: Option<Vec2>,
    pub current_pos: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerId {
    Player1,
    Player2,
}

impl PlayerId {
    pub fn next(&self) -> Self {
        match self {
            PlayerId::Player1 => PlayerId::Player2,
            PlayerId::Player2 => PlayerId::Player1,
        }
    }
}

impl std::fmt::Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerId::Player1 => write!(f, "Jogador 1"),
            PlayerId::Player2 => write!(f, "Jogador 2"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerGroup {
    Solids,
    Stripes,
    Undecided,
    EightBall,
}

impl std::fmt::Display for PlayerGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerGroup::Solids => write!(f, "Liscas"),
            PlayerGroup::Stripes => write!(f, "Listradas"),
            PlayerGroup::Undecided => write!(f, "Indefinido"),
            PlayerGroup::EightBall => write!(f, "Bola 8"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Initializing,
    Aiming,
    Shooting,
    BallsMoving,
    RepositionCueBall,
    GameOver,
}
