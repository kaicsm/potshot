use macroquad::prelude::Color;
use crate::colors::game_colors; // Atualizado para o novo nome do módulo de cores

#[derive(Clone, Debug)] // Adicionado Debug para permitir imprimir
pub struct BallDefinition {
    pub number: u8,
    pub color: Color,
    pub is_striped: bool,
}

pub fn get_ball_definitions() -> Vec<BallDefinition> {
    vec![
        BallDefinition { number: 1, color: game_colors::SOLID_YELLOW, is_striped: false },
        BallDefinition { number: 2, color: game_colors::SOLID_BLUE, is_striped: false },
        BallDefinition { number: 3, color: game_colors::SOLID_RED, is_striped: false },
        BallDefinition { number: 4, color: game_colors::SOLID_PURPLE, is_striped: false },
        BallDefinition { number: 5, color: game_colors::SOLID_ORANGE, is_striped: false },
        BallDefinition { number: 6, color: game_colors::SOLID_GREEN, is_striped: false },
        BallDefinition { number: 7, color: game_colors::SOLID_MAROON, is_striped: false },
        BallDefinition { number: 8, color: game_colors::EIGHT_BALL, is_striped: false },
        BallDefinition { number: 9, color: game_colors::SOLID_YELLOW, is_striped: true },
        BallDefinition { number: 10, color: game_colors::SOLID_BLUE, is_striped: true },
        BallDefinition { number: 11, color: game_colors::SOLID_RED, is_striped: true },
        BallDefinition { number: 12, color: game_colors::SOLID_PURPLE, is_striped: true },
        BallDefinition { number: 13, color: game_colors::SOLID_ORANGE, is_striped: true },
        BallDefinition { number: 14, color: game_colors::SOLID_GREEN, is_striped: true },
        BallDefinition { number: 15, color: game_colors::SOLID_MAROON, is_striped: true },
    ]
}

