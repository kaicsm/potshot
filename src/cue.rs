use macroquad::prelude::*;
use crate::colors::game_colors;

pub struct Cue {
    pub angle: f32,
    pub power: f32,
    pub visible: bool,
    pub is_dragging: bool,
}

impl Cue {
    pub fn new() -> Self {
        Cue { angle: 0.0, power: 0.0, visible: true, is_dragging: false }
    }

    pub fn draw(&self, cue_ball_abs_pos: Vec2, cue_ball_radius: f32, table_width: f32, cue_max_len: f32, cue_w: f32) {
        if !self.visible { return; }

        let dir = Vec2::from_angle(self.angle);
        let hold_end_offset = self.power * cue_max_len + cue_ball_radius + 20.0;
        let hold_end_pos = cue_ball_abs_pos - dir * hold_end_offset;
        let tip_start_offset = cue_ball_radius + 2.0;
        let tip_start_pos = cue_ball_abs_pos - dir * tip_start_offset;

        let r_comp = (self.power * 2.0).min(1.0);
        let g_comp = (1.0 - self.power * 1.5).max(0.0);
        let dynamic_cue_color = Color::new(r_comp, g_comp, 0.1, 1.0);

        draw_line(tip_start_pos.x, tip_start_pos.y, hold_end_pos.x, hold_end_pos.y, cue_w, dynamic_cue_color);
        draw_circle(tip_start_pos.x, tip_start_pos.y, cue_w / 3.0, game_colors::CUE_TIP);

        if self.is_dragging {
            let aim_line_length = table_width;
            let aim_end_pos = cue_ball_abs_pos + dir * aim_line_length;
            draw_line(cue_ball_abs_pos.x, cue_ball_abs_pos.y, aim_end_pos.x, aim_end_pos.y, 1.0, game_colors::AIM_LINE_COLOR);
        }
    }
}

