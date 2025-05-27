use macroquad::prelude::*;
use crate::colors::game_colors; // Atualizado

#[derive(Debug, Clone, Copy)]
pub struct Pocket {
    pub pos: Vec2,
    pub radius: f32,
}

impl Pocket {
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        Pocket { pos: vec2(x, y), radius }
    }

    pub fn draw(&self, table_offset: Vec2) {
        let draw_pos = self.pos + table_offset;
        draw_circle(draw_pos.x, draw_pos.y, self.radius, game_colors::POCKET_BORDER);
        draw_circle(draw_pos.x, draw_pos.y, self.radius * 0.7, game_colors::POCKET_CENTER);
    }
}

