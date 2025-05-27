use macroquad::prelude::*;
use crate::ball::Ball;
use crate::pocket::Pocket;
use crate::constants::{MIN_SPEED, BALL_ELASTICITY};
use crate::Game; // Para acessar self.balls, self.pockets
use std::collections::HashMap; // Para ball_definitions_map
use crate::config::BallDefinition; // Para BallDefinition
use crate::types::PlayerId; // Para current_player


impl Game { // Adicionando métodos de física à struct Game
    pub fn update_physics_objects(&mut self) { // Renomeado para evitar conflito com update em game.rs
        if self.game_state == crate::types::GameState::Shooting || self.game_state == crate::types::GameState::BallsMoving {
            let mut still_moving = false;
            for i in 0..self.balls.len() {
                if self.balls[i].in_pocket { continue; }

                self.balls[i].update_position(); // Ball 자체의 update_position 호출
                self.balls[i].check_wall_collision(self.table_width, self.table_height); // Ball 자체의 check_wall_collision 호출
                self.check_pocket_collision_for_ball(i);

                if self.balls[i].vel.length_squared() > MIN_SPEED.powi(2) {
                    still_moving = true;
                }
            }

            let num_balls = self.balls.len();
            for i in 0..num_balls {
                if self.balls[i].in_pocket { continue; }
                let (first_half, second_half) = self.balls.split_at_mut(i + 1);
                let ball1 = &mut first_half[i];
                for ball2 in second_half.iter_mut() {
                    if ball2.in_pocket { continue; }
                    Self::handle_ball_to_ball_collision(ball1, ball2);
                }
            }

            if self.game_state == crate::types::GameState::Shooting && still_moving {
                self.game_state = crate::types::GameState::BallsMoving;
            } else if self.game_state == crate::types::GameState::BallsMoving && !still_moving {
                self.handle_player_turn_end(); // Esta função está em rules.rs agora, chamada por game.rs
            } else if self.game_state == crate::types::GameState::Shooting && !still_moving { // Shot too weak
                 self.handle_player_turn_end();
            }
        }
    }

    pub fn check_pocket_collision_for_ball(&mut self, ball_idx: usize) {
        if self.balls[ball_idx].in_pocket { return; }

        for pocket in &self.pockets {
            let dist_sq = self.balls[ball_idx].pos.distance_squared(pocket.pos);
            let effective_pocket_radius = pocket.radius * 0.9;

            if dist_sq < effective_pocket_radius.powi(2) {
                if !self.balls[ball_idx].in_pocket {
                    self.balls[ball_idx].in_pocket = true;
                    self.balls[ball_idx].vel = Vec2::ZERO;
                    
                    let ball_number = self.balls[ball_idx].number;
                    if ball_number != 0 {
                        self.potted_ball_numbers_this_turn.push(ball_number);
                        
                        if let Some(def) = self.ball_definitions_map.get(&ball_number).cloned() {
                            if self.current_player == PlayerId::Player1 {
                                self.player1_pocketed_balls.push(def);
                            } else {
                                self.player2_pocketed_balls.push(def);
                            }
                        }
                    }
                }
                break; 
            }
        }
    }
    
    fn handle_ball_to_ball_collision(b1: &mut Ball, b2: &mut Ball) {
        if b1.in_pocket || b2.in_pocket { return; }
        let delta = b2.pos - b1.pos;
        let dist_sq = delta.length_squared();
        let sum_radii = b1.radius + b2.radius;

        if dist_sq < sum_radii * sum_radii && dist_sq > 0.001 {
            let dist = dist_sq.sqrt();
            let overlap = (sum_radii - dist) / 2.0;
            let correction_vec = delta.normalize_or_zero() * overlap;
            b1.pos -= correction_vec;
            b2.pos += correction_vec;

            let normal = delta.normalize_or_zero();
            let tangent = vec2(-normal.y, normal.x);
            let dp_tan1 = b1.vel.dot(tangent);
            let dp_tan2 = b2.vel.dot(tangent);
            let dp_norm1 = b1.vel.dot(normal);
            let dp_norm2 = b2.vel.dot(normal);

            b1.vel = tangent * dp_tan1 + normal * dp_norm2 * BALL_ELASTICITY;
            b2.vel = tangent * dp_tan2 + normal * dp_norm1 * BALL_ELASTICITY;
        }
    }
}

