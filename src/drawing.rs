use macroquad::prelude::*;
use crate::Game;
use crate::colors::game_colors;

impl Game {
    pub fn draw_game_elements(&self) { // Renomeado para evitar conflito com draw em game.rs
        clear_background(Color::new(0.05,0.05,0.07,1.0));

        self.draw_table_and_elements();
        self.draw_hud();
        self.draw_main_message();
    }

    fn draw_table_and_elements(&self) {
        draw_rectangle(
            self.table_offset.x,
            self.table_offset.y,
            self.table_width + self.table_border_thickness * 2.0,
            self.table_height + self.table_border_thickness * 2.0,
            game_colors::TABLE_BORDER_COLOR,
        );
        draw_rectangle(
            self.game_area_offset.x,
            self.game_area_offset.y,
            self.table_width,
            self.table_height,
            game_colors::TABLE_BG,
        );

        let baulk_line_x = self.game_area_offset.x + self.table_width * 0.25;
        draw_line(
            baulk_line_x, self.game_area_offset.y,
            baulk_line_x, self.game_area_offset.y + self.table_height,
            1.0, Color::new(1.0, 1.0, 1.0, 0.15),
        );
        draw_circle(
            baulk_line_x, self.game_area_offset.y + self.table_height / 2.0,
            self.ball_radius / 3.0, Color::new(1.0, 1.0, 1.0, 0.2),
        );
        let head_spot_x = self.game_area_offset.x + self.table_width * 0.7;
        let head_spot_y = self.game_area_offset.y + self.table_height / 2.0;
        draw_circle(head_spot_x, head_spot_y, self.ball_radius / 3.0, Color::new(1.0,1.0,1.0,0.1));

        for pocket in &self.pockets {
            pocket.draw(self.game_area_offset);
        }
        for ball in &self.balls {
            ball.draw(self.font.clone(), self.game_area_offset);
        }

        if let Some(cb_idx) = self.cue_ball_idx {
            if let Some(cb) = self.balls.get(cb_idx) {
                if !cb.in_pocket && self.cue.visible && self.game_state != crate::types::GameState::GameOver &&
                   self.game_state != crate::types::GameState::Initializing && self.game_state != crate::types::GameState::RepositionCueBall {
                    let cue_ball_abs_pos = cb.pos + self.game_area_offset;
                    self.cue.draw(cue_ball_abs_pos, cb.radius, self.table_width, self.cue_max_length, self.cue_width);
                }
            }
        }

        if self.game_state == crate::types::GameState::RepositionCueBall {
            let reposition_area_width = self.table_width * 0.25;
            draw_rectangle(
                self.game_area_offset.x, self.game_area_offset.y,
                reposition_area_width, self.table_height,
                game_colors::REPOSITION_AREA_FILL,
            );
            draw_rectangle_lines(
                self.game_area_offset.x + 1.0, self.game_area_offset.y + 1.0,
                reposition_area_width - 2.0, self.table_height - 2.0,
                2.0, game_colors::REPOSITION_AREA_STROKE,
            );
            let input_table_relative_pos = self.input_state.current_pos - self.game_area_offset;
             if input_table_relative_pos.x > 0.0 && input_table_relative_pos.x < reposition_area_width &&
                input_table_relative_pos.y > 0.0 && input_table_relative_pos.y < self.table_height {
                draw_circle(self.input_state.current_pos.x, self.input_state.current_pos.y, self.ball_radius, Color::new(1.0,1.0,1.0,0.3));
             }
        }
    }

    fn draw_hud(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();
        let hud_y_start = screen_h * 0.85;
        let hud_height = screen_h * 0.15;
        let padding = screen_h * 0.015;
        let text_font_size = (screen_h * 0.025).max(16.0) as u16;
        let ball_display_radius = (self.ball_radius * 0.7).max(5.0);

        draw_rectangle(0.0, hud_y_start, screen_w, hud_height, game_colors::UI_BG_COLOR);

        let p1_text = format!("P1: {}", self.player1_group);
        draw_text_ex(&p1_text, padding, hud_y_start + padding + text_font_size as f32 * 0.5, TextParams {
            font: Some(&self.font), font_size: text_font_size, color: game_colors::HUD_TEXT_COLOR, ..Default::default()
        });
        let p1_balls_y = hud_y_start + padding + text_font_size as f32 + padding * 0.5;
        for (i, ball_def) in self.player1_pocketed_balls.iter().enumerate() {
            let x = padding + i as f32 * (ball_display_radius * 2.5);
            draw_circle(x + ball_display_radius, p1_balls_y + ball_display_radius, ball_display_radius, ball_def.color);
            if ball_def.is_striped {
                draw_circle(x + ball_display_radius, p1_balls_y + ball_display_radius, ball_display_radius * 0.6, game_colors::STRIPE_PRIMARY);
                draw_circle(x + ball_display_radius, p1_balls_y + ball_display_radius, ball_display_radius * 0.3, ball_def.color);
            }
        }

        let p2_text = format!("P2: {}", self.player2_group);
        let p2_text_dims = measure_text(&p2_text, Some(&self.font), text_font_size, 1.0);
        draw_text_ex(&p2_text, screen_w - padding - p2_text_dims.width, hud_y_start + padding + text_font_size as f32 * 0.5, TextParams {
            font: Some(&self.font), font_size: text_font_size, color: game_colors::HUD_TEXT_COLOR, ..Default::default()
        });
        let p2_balls_y = hud_y_start + padding + text_font_size as f32 + padding * 0.5;
        for (i, ball_def) in self.player2_pocketed_balls.iter().enumerate() {
            let x = screen_w - padding - (self.player2_pocketed_balls.len() as f32 - i as f32) * (ball_display_radius * 2.5);
             draw_circle(x + ball_display_radius, p2_balls_y + ball_display_radius, ball_display_radius, ball_def.color);
            if ball_def.is_striped {
                draw_circle(x + ball_display_radius, p2_balls_y + ball_display_radius, ball_display_radius * 0.6, game_colors::STRIPE_PRIMARY);
                draw_circle(x + ball_display_radius, p2_balls_y + ball_display_radius, ball_display_radius * 0.3, ball_def.color);
            }
        }
        
        let turn_text = format!("Vez de: {}", self.current_player);
        let turn_text_dims = measure_text(&turn_text, Some(&self.font), text_font_size, 1.0);
        let turn_text_y = hud_y_start + padding + text_font_size as f32 * 0.5;
        draw_text_ex(&turn_text, screen_w / 2.0 - turn_text_dims.width / 2.0, turn_text_y, TextParams {
            font: Some(&self.font), font_size: text_font_size, color: game_colors::HUD_TEXT_COLOR, ..Default::default()
        });

        let (mouse_x, mouse_y) = mouse_position();
        let btn_color = if self.reset_button_rect.contains(vec2(mouse_x, mouse_y)) { game_colors::BUTTON_HOVER_BG } else { game_colors::BUTTON_BG };
        draw_rectangle(self.reset_button_rect.x, self.reset_button_rect.y, self.reset_button_rect.w, self.reset_button_rect.h, btn_color);
        let btn_text = "Reiniciar";
        let btn_font_size = (self.reset_button_rect.h * 0.5).max(14.0) as u16;
        let btn_text_dims = measure_text(btn_text, Some(&self.font), btn_font_size, 1.0);
        draw_text_ex(btn_text,
            self.reset_button_rect.x + (self.reset_button_rect.w - btn_text_dims.width) / 2.0,
            self.reset_button_rect.y + (self.reset_button_rect.h - btn_text_dims.height) / 2.0 + btn_text_dims.offset_y * 0.8,
            TextParams { font: Some(&self.font), font_size: btn_font_size, color: game_colors::BUTTON_TEXT, ..Default::default() });
    }
    
    fn draw_main_message(&self) {
        let screen_w = screen_width();
        let msg_font_size = (screen_height() * 0.035).max(18.0) as u16;
        let msg_text_dims = measure_text(&self.message, Some(&self.font), msg_font_size, 1.0);
        let msg_y_pos = self.table_offset.y / 2.0 - msg_text_dims.height / 2.0;
        
        draw_text_ex(&self.message, screen_w / 2.0 - msg_text_dims.width / 2.0, msg_y_pos.max(msg_font_size as f32 * 0.5), TextParams {
            font: Some(&self.font), font_size: msg_font_size, color: WHITE, ..Default::default()
        });
    }
}

