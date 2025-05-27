use macroquad::prelude::*;
use crate::constants::{FRICTION, MIN_SPEED, CUSHION_ELASTICITY};
use crate::colors::game_colors; // Atualizado

#[derive(Debug, Clone)]
pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub color: Color,
    pub number: u8,
    pub is_striped: bool,
    pub in_pocket: bool,
}

impl Ball {
    pub fn new(x: f32, y: f32, color: Color, number: u8, is_striped: bool, radius: f32) -> Self {
        Ball {
            pos: vec2(x, y),
            vel: Vec2::ZERO,
            radius,
            color,
            number,
            is_striped,
            in_pocket: false,
        }
    }

    pub fn draw(&self, font: Font, table_offset: Vec2) {
        if self.in_pocket {
            return;
        }
        let draw_pos = self.pos + table_offset;

        draw_circle(draw_pos.x, draw_pos.y, self.radius, self.color);

        let highlight_offset = self.radius * 0.4;
        draw_circle(
            draw_pos.x - highlight_offset * 0.7,
            draw_pos.y - highlight_offset * 0.7,
            self.radius * 0.3,
            Color::new(1.0, 1.0, 1.0, 0.6),
        );

        if self.number != 0 { // Not cue ball
            let text_x = draw_pos.x;
            let text_y = draw_pos.y;
            let circle_radius_for_number = self.radius * 0.55;

            if self.is_striped {
                draw_circle(text_x, text_y, self.radius, game_colors::STRIPE_PRIMARY);
                draw_circle(text_x, text_y, self.radius * 0.6, self.color);
                draw_circle(text_x, text_y, circle_radius_for_number, game_colors::STRIPE_PRIMARY);
            } else { // Solid or 8-ball
                draw_circle(text_x, text_y, circle_radius_for_number, WHITE);
            }

            let text_color = if self.number == 8 || !self.is_striped { BLACK } else { self.color };
            let text = self.number.to_string();
            let font_size = (self.radius * 1.1) as u16;
            let text_params = TextParams {
                font: Some(&font),
                font_size,
                color: text_color,
                ..Default::default()
            };
            let text_dimensions = measure_text(&text, Some(&font), font_size, 1.0);
            draw_text_ex(
                &text,
                text_x - text_dimensions.width / 2.0,
                text_y + text_dimensions.offset_y / 2.0 + text_dimensions.height * 0.3,
                text_params,
            );
        }
    }

    pub fn update_position(&mut self) {
        if self.in_pocket { return; }
        self.pos += self.vel;
        self.vel *= FRICTION;
        if self.vel.length_squared() < MIN_SPEED * MIN_SPEED {
            self.vel = Vec2::ZERO;
        }
    }

    pub fn check_wall_collision(&mut self, table_width: f32, table_height: f32) {
        if self.in_pocket { return; }
        if self.pos.x + self.radius > table_width {
            self.pos.x = table_width - self.radius;
            self.vel.x *= -CUSHION_ELASTICITY;
        } else if self.pos.x - self.radius < 0.0 {
            self.pos.x = self.radius;
            self.vel.x *= -CUSHION_ELASTICITY;
        }
        if self.pos.y + self.radius > table_height {
            self.pos.y = table_height - self.radius;
            self.vel.y *= -CUSHION_ELASTICITY;
        } else if self.pos.y - self.radius < 0.0 {
            self.pos.y = self.radius;
            self.vel.y *= -CUSHION_ELASTICITY;
        }
    }
}

