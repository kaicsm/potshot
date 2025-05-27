use macroquad::prelude::*;
use std::collections::HashMap;

use crate::types::{InputState, PlayerId, PlayerGroup, GameState};
use crate::constants::*;
use crate::colors::game_colors;
use crate::config::{BallDefinition, get_ball_definitions};
use crate::ball::Ball;
use crate::pocket::Pocket;
use crate::cue::Cue;

pub struct Game {
    pub ball_radius: f32,
    pub table_border_thickness: f32,
    pub cue_max_length: f32,
    pub cue_width: f32,
    pub max_power_shot: f32,
    pub pocket_radius: f32,

    pub table_width: f32,
    pub table_height: f32,
    pub table_offset: Vec2,
    pub game_area_offset: Vec2,

    pub balls: Vec<Ball>,
    pub pockets: Vec<Pocket>,
    pub cue: Cue,
    pub cue_ball_idx: Option<usize>,
    pub game_state: GameState,
    pub message: String,
    pub font: Font,
    pub reset_button_rect: Rect,
    pub input_state: InputState,

    pub current_player: PlayerId,
    pub player1_group: PlayerGroup,
    pub player2_group: PlayerGroup,
    pub potted_ball_numbers_this_turn: Vec<u8>,
    pub is_break_shot: bool,

    pub player1_pocketed_balls: Vec<BallDefinition>,
    pub player2_pocketed_balls: Vec<BallDefinition>,
    pub ball_definitions_map: HashMap<u8, BallDefinition>,
}

impl Game {
    pub async fn new() -> Self {
        let font = load_ttf_font("assets/Inter-Regular.ttf")
            .await
            .unwrap_or_else(|e| panic!("Falha ao carregar a fonte 'assets/Inter-Regular.ttf'. Certifique-se de que o arquivo existe na pasta 'assets' (no mesmo nível que 'src'). Erro: {}", e));

        let ball_defs_vec = get_ball_definitions();
        let ball_definitions_map = ball_defs_vec.into_iter()
            .map(|def| (def.number, def))
            .collect::<HashMap<_, _>>();

        let mut game = Game {
            ball_radius: 0.0,
            table_border_thickness: 0.0,
            cue_max_length: 0.0,
            cue_width: 0.0,
            max_power_shot: 0.0,
            pocket_radius: 0.0,
            table_width: 0.0,
            table_height: 0.0,
            table_offset: Vec2::ZERO,
            game_area_offset: Vec2::ZERO,
            balls: Vec::new(),
            pockets: Vec::new(),
            cue: Cue::new(),
            cue_ball_idx: None,
            game_state: GameState::Initializing,
            message: String::new(),
            font,
            reset_button_rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            input_state: InputState::default(),
            current_player: PlayerId::Player1,
            player1_group: PlayerGroup::Undecided,
            player2_group: PlayerGroup::Undecided,
            potted_ball_numbers_this_turn: Vec::new(),
            is_break_shot: true,
            player1_pocketed_balls: Vec::new(),
            player2_pocketed_balls: Vec::new(),
            ball_definitions_map,
        };
        game.resize_and_init();
        game
    }

    pub fn resize_and_init(&mut self) {
        self.calculate_and_set_dimensions();
        self.setup_pockets();
        self.initialize_game_logic();

        let screen_w = screen_width();
        let screen_h = screen_height();
        let button_width = screen_w * 0.2;
        let button_height = screen_h * 0.05;
        let hud_bottom_y = screen_h * 0.85 + screen_h * 0.025;

        self.reset_button_rect = Rect::new(
            screen_w / 2.0 - button_width / 2.0,
            hud_bottom_y + button_height * 0.5,
            button_width,
            button_height,
        );
        self.game_state = GameState::Aiming;
    }
    
    fn calculate_and_set_dimensions(&mut self) {
        let screen_w = screen_width();
        let screen_h = screen_height();
        let min_dimension = screen_w.min(screen_h);

        self.ball_radius = (min_dimension * 0.017).max(8.0);
        self.table_border_thickness = (min_dimension * 0.02).max(5.0);
        self.cue_max_length = (min_dimension * 0.25).max(80.0);
        self.cue_width = (min_dimension * 0.012).max(4.0);
        self.max_power_shot = (min_dimension * 0.05).max(15.0);
        self.pocket_radius = self.ball_radius * POCKET_RADIUS_MULTIPLIER;
        
        let hud_height = screen_h * 0.15;
        let available_height_for_table_and_message = screen_h - hud_height;
        let message_area_height = screen_h * 0.08;
        let table_area_max_width = screen_w - (self.table_border_thickness * 2.0) - screen_w * 0.02;
        let table_area_max_height = available_height_for_table_and_message - message_area_height - (self.table_border_thickness * 2.0);

        let width_from_h = table_area_max_height * 2.0;
        self.table_width = table_area_max_width.min(width_from_h);
        self.table_height = self.table_width / 2.0;

        if self.table_width < screen_w * 0.3 {
            self.table_width = screen_w * 0.3;
            self.table_height = self.table_width / 2.0;
        }
        
        self.table_offset = vec2(
            (screen_w - self.table_width - self.table_border_thickness * 2.0) / 2.0,
            message_area_height, 
        );
        self.game_area_offset = vec2(
            self.table_offset.x + self.table_border_thickness,
            self.table_offset.y + self.table_border_thickness,
        );
    }

    fn setup_pockets(&mut self) {
        self.pockets.clear();
        let pr = self.pocket_radius;
        let off = pr * 0.1;

        self.pockets.push(Pocket::new(off, off, pr));
        self.pockets.push(Pocket::new(self.table_width - off, off, pr));
        self.pockets.push(Pocket::new(off, self.table_height - off, pr));
        self.pockets.push(Pocket::new(self.table_width - off, self.table_height - off, pr));
        
        let mid_pr = pr * 0.9;
        self.pockets.push(Pocket::new(self.table_width / 2.0, off - pr * 0.4, mid_pr));
        self.pockets.push(Pocket::new(self.table_width / 2.0, self.table_height - off + pr * 0.4, mid_pr));
    }

    fn setup_balls(&mut self) {
        self.balls.clear();
        let r = self.ball_radius;
        let start_x = self.table_width * 0.7;
        let start_y = self.table_height / 2.0;

        self.balls.push(Ball::new(
            self.table_width * 0.25,
            self.table_height / 2.0,
            game_colors::CUE, 0, false, r,
        ));
        self.cue_ball_idx = Some(0);

        let ball_defs = get_ball_definitions();
        let rack_order_indices = [0, 8, 1, 10, 7, 2, 3, 12, 9, 14, 4, 6, 13, 5, 11];
        
        let mut current_ball_in_rack = 0;
        for row in 0..5 {
            for col in 0..=row {
                if current_ball_in_rack >= rack_order_indices.len() { break; }
                let def_idx = rack_order_indices[current_ball_in_rack];
                let def = &ball_defs[def_idx];
                let x = start_x + row as f32 * (r * 2.0 * 0.8660254);
                let y = start_y + col as f32 * r * 2.0 - row as f32 * r;
                self.balls.push(Ball::new(x, y, def.color, def.number, def.is_striped, r));
                current_ball_in_rack += 1;
            }
        }
    }
    
    fn initialize_game_logic(&mut self) {
        self.game_state = GameState::Initializing; // Set to initializing first
        self.setup_balls();

        if self.balls.is_empty() || self.balls[0].number != 0 {
             self.balls.insert(0,Ball::new(
                    self.table_width * 0.25,
                    self.table_height / 2.0,
                    game_colors::CUE,0,false,self.ball_radius));
            self.cue_ball_idx = Some(0);
        }
        
        self.current_player = PlayerId::Player1;
        self.player1_group = PlayerGroup::Undecided;
        self.player2_group = PlayerGroup::Undecided;
        self.player1_pocketed_balls.clear();
        self.player2_pocketed_balls.clear();
        self.potted_ball_numbers_this_turn.clear();
        self.is_break_shot = true;

        self.game_state = GameState::Aiming; // Now ready for aiming
        self.cue.visible = true;
        self.message = format!("{} para iniciar a partida!", self.current_player);
    }

    // Renomeado de handle_input para process_input para o loop principal em main.rs
    pub fn process_input(&mut self) {
        let input_pos = if let Some(touch) = touches().get(0) {
            vec2(touch.position.x, touch.position.y)
        } else {
            mouse_position().into()
        };

        if is_mouse_button_pressed(MouseButton::Left) || (touches().len() == 1 && !self.input_state.is_dragging) {
            self.input_state.is_dragging = true;
            self.input_state.start_pos = Some(input_pos);
        } else if !(is_mouse_button_down(MouseButton::Left) || !touches().is_empty()) && self.input_state.is_dragging {
            if self.input_state.start_pos.is_some() {
                if self.reset_button_rect.contains(input_pos) && self.input_state.start_pos.unwrap_or(Vec2::ZERO).distance(input_pos) < 5.0 {
                    self.resize_and_init(); // Reinicia o jogo
                    self.input_state.is_dragging = false;
                    self.input_state.start_pos = None;
                    return;
                }
            }
            self.input_state.is_dragging = false;
        }
        self.input_state.current_pos = input_pos;

        if self.game_state == GameState::Initializing || self.game_state == GameState::GameOver {
            return;
        }

        let input_table_relative_pos = input_pos - self.game_area_offset;

        if self.game_state == GameState::RepositionCueBall {
            if let Some(cb_idx) = self.cue_ball_idx {
                let reposition_area_width = self.table_width * 0.25;
                let is_within_reposition_area = input_table_relative_pos.x > 0.0
                    && input_table_relative_pos.x < reposition_area_width
                    && input_table_relative_pos.y > 0.0
                    && input_table_relative_pos.y < self.table_height;

                if is_within_reposition_area {
                    if is_mouse_button_pressed(MouseButton::Left) || (touches().len() == 1 && self.input_state.start_pos == Some(input_pos)) {
                        let mut valid_pos = true;
                        for (i, ball) in self.balls.iter().enumerate() {
                            if Some(i) == self.cue_ball_idx || ball.in_pocket { continue; }
                            if ball.pos.x < reposition_area_width + self.ball_radius * 2.0 {
                                if ball.pos.distance_squared(input_table_relative_pos) < (self.ball_radius * 2.0).powi(2) {
                                    valid_pos = false;
                                    break;
                                }
                            }
                        }
                        if valid_pos {
                            let cb = &mut self.balls[cb_idx];
                            cb.pos = input_table_relative_pos;
                            cb.vel = Vec2::ZERO;
                            cb.in_pocket = false;
                            self.game_state = GameState::Aiming;
                            self.cue.visible = true;
                            self.message = format!("{} mira.", self.current_player);
                        } else {
                            self.message = "Posição inválida (sobrepõe outra bola).".to_string();
                        }
                    }
                } else if is_mouse_button_pressed(MouseButton::Left) {
                     self.message = "Posicione a branca na área de saída (à esquerda).".to_string();
                }
            }
            return;
        }

        if self.game_state == GameState::Aiming {
            if let Some(cb) = self.cue_ball_idx.and_then(|idx| self.balls.get(idx)) {
                if cb.in_pocket { return; }

                let cue_ball_screen_pos = cb.pos + self.game_area_offset;

                if self.input_state.is_dragging && self.input_state.start_pos.is_some() {
                    self.cue.is_dragging = true;
                    let dx = input_pos.x - cue_ball_screen_pos.x;
                    let dy = input_pos.y - cue_ball_screen_pos.y;
                    let dist_from_ball = (dx * dx + dy * dy).sqrt();

                    self.cue.angle = dy.atan2(dx);
                    self.cue.power = (dist_from_ball / self.cue_max_length).min(1.0);
                } else if !self.input_state.is_dragging && self.cue.is_dragging { // Released
                    self.cue.is_dragging = false;
                    if let Some(cb_mut) = self.cue_ball_idx.and_then(|idx| self.balls.get_mut(idx)) {
                        if self.cue.power > 0.05 { // Min power threshold
                            let shot_vec = Vec2::from_angle(self.cue.angle) * self.cue.power * self.max_power_shot;
                            cb_mut.vel = shot_vec * BALL_ELASTICITY; // Aplicar elasticidade aqui pode ser melhor
                            self.game_state = GameState::Shooting;
                            self.cue.visible = false;
                            self.message = "Bolas em movimento...".to_string();
                            self.potted_ball_numbers_this_turn.clear();
                        } else {
                             self.message = format!("{} mira. (Força muito baixa)", self.current_player);
                        }
                        self.cue.power = 0.0; // Reset power after shot
                    }
                }
            }
        }
    }

    // update é chamado em cada frame para lógica do jogo que não é input ou renderização
    pub fn update(&mut self) {
        if self.game_state != GameState::Initializing {
            self.update_physics_objects(); // Chama a função de physics.rs
        }
    }

    // render é chamado para desenhar tudo
    pub fn render(&self) {
        self.draw_game_elements(); // Chama a função de drawing.rs
    }
}

