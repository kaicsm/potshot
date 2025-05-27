use crate::Game; // Precisa de acesso a quase tudo de Game
use crate::types::{PlayerGroup, PlayerId, GameState};

impl Game {
    // handle_player_turn_end, check_eight_ball_pot_legality, update_player_groups_if_only_eight_ball_left
    // Movidos para cá e implementados como métodos de Game.
    // Atenção: eles precisam de acesso a `self`, então são `pub fn method_name(&mut self)`.

    pub fn handle_player_turn_end(&mut self) {
        let cue_ball_is_pocketed = self.cue_ball_idx.map_or(true, |idx| self.balls[idx].in_pocket);
        let mut player_continues_turn = false;
        let mut game_over_by_eight_ball = false;

        let mut current_player_group = if self.current_player == PlayerId::Player1 { self.player1_group } else { self.player2_group };

        if self.potted_ball_numbers_this_turn.contains(&8) {
            game_over_by_eight_ball = true;
            let (winner, loser_message) = self.check_eight_ball_pot_legality(current_player_group, cue_ball_is_pocketed);
            if let Some(winning_player) = winner {
                self.message = format!("{} VENCEU!", winning_player);
            } else {
                self.message = loser_message;
            }
            self.game_state = GameState::GameOver;
            self.cue.visible = false;
        }

        if game_over_by_eight_ball {
            self.potted_ball_numbers_this_turn.clear();
            return;
        }

        if cue_ball_is_pocketed {
            self.message = format!("Branca na caçapa! {} reposiciona.", self.current_player.next());
            self.game_state = GameState::RepositionCueBall;
            self.cue.visible = false;
        } else {
            let mut solid_potted_count = 0;
            let mut stripe_potted_count = 0;
            let mut first_potted_ball_type: Option<PlayerGroup> = None;

            for &ball_num in &self.potted_ball_numbers_this_turn {
                if ball_num == 8 { continue; }
                let def = self.ball_definitions_map.get(&ball_num);
                if let Some(d) = def {
                    if d.is_striped {
                        stripe_potted_count +=1;
                        if first_potted_ball_type.is_none() { first_potted_ball_type = Some(PlayerGroup::Stripes); }
                    } else {
                        solid_potted_count +=1;
                        if first_potted_ball_type.is_none() { first_potted_ball_type = Some(PlayerGroup::Solids); }
                    }
                }
            }

            if current_player_group == PlayerGroup::Undecided {
                if let Some(potted_type) = first_potted_ball_type {
                    if self.current_player == PlayerId::Player1 {
                        self.player1_group = potted_type;
                        self.player2_group = if potted_type == PlayerGroup::Solids { PlayerGroup::Stripes } else { PlayerGroup::Solids };
                    } else {
                        self.player2_group = potted_type;
                        self.player1_group = if potted_type == PlayerGroup::Solids { PlayerGroup::Stripes } else { PlayerGroup::Solids };
                    }
                    current_player_group = potted_type;
                    self.message = format!("{} é {}! Jogue novamente.", self.current_player, current_player_group);
                    player_continues_turn = true;
                } else if self.is_break_shot && !self.potted_ball_numbers_this_turn.is_empty() {
                     self.message = format!("Mesa aberta! {} joga novamente.", self.current_player);
                     player_continues_turn = true;
                } else if !self.potted_ball_numbers_this_turn.is_empty() {
                    self.message = format!("Mesa aberta! {} joga novamente.", self.current_player);
                    player_continues_turn = true;
                } else {
                    self.message = format!("Nenhuma bola encaçapada. Vez de {}.", self.current_player.next());
                }
            } else {
                let potted_own_ball = match current_player_group {
                    PlayerGroup::Solids => solid_potted_count > 0,
                    PlayerGroup::Stripes => stripe_potted_count > 0,
                    _ => false,
                };

                if potted_own_ball {
                    self.message = format!("Boa jogada! {} joga novamente.", self.current_player);
                    player_continues_turn = true;
                } else {
                     let next_player_msg = self.current_player.next().to_string();
                    if !self.potted_ball_numbers_this_turn.is_empty() {
                        self.message = format!("Encaçapou bola errada. Vez de {}.", next_player_msg);
                    } else {
                        self.message = format!("Nenhuma bola do seu grupo. Vez de {}.", next_player_msg);
                    }
                }
            }
        }
        
        self.is_break_shot = false;

        if !player_continues_turn || cue_ball_is_pocketed {
            self.current_player = self.current_player.next();
        }

        if self.game_state != GameState::RepositionCueBall {
            self.game_state = GameState::Aiming;
            self.cue.visible = true;
            if !player_continues_turn && !cue_ball_is_pocketed {
                 self.message = format!("{} mira.", self.current_player);
            }
        }
        
        if self.game_state != GameState::GameOver {
             self.update_player_groups_if_only_eight_ball_left();
        }
    }
    
    pub fn check_eight_ball_pot_legality(&self, player_group_before_pot: PlayerGroup, cue_ball_was_pocketed: bool) -> (Option<PlayerId>, String) {
        if cue_ball_was_pocketed {
            return (None, format!("FIM! Bola 8 E branca na caçapa. {} perde.", self.current_player));
        }

        if player_group_before_pot == PlayerGroup::Undecided {
            return (None, format!("FIM! Bola 8 com mesa aberta. {} perde.", self.current_player));
        }
        
        if player_group_before_pot == PlayerGroup::EightBall {
            return (Some(self.current_player), String::new());
        }

        let mut player_balls_of_own_group_remaining_on_table = 0;
        for ball in &self.balls {
            if ball.in_pocket || ball.number == 0 || ball.number == 8 { continue; }
            let def = self.ball_definitions_map.get(&ball.number);
            if let Some(d) = def {
                 if player_group_before_pot == PlayerGroup::Solids && !d.is_striped {
                    player_balls_of_own_group_remaining_on_table +=1;
                } else if player_group_before_pot == PlayerGroup::Stripes && d.is_striped {
                    player_balls_of_own_group_remaining_on_table +=1;
                }
            }
        }

        if player_balls_of_own_group_remaining_on_table == 0 {
            return (Some(self.current_player), String::new());
        } else {
            return (None, format!("FIM! Bola 8 prematuramente. {} perde.", self.current_player));
        }
    }

    pub fn update_player_groups_if_only_eight_ball_left(&mut self) {
        if self.player1_group == PlayerGroup::Solids || self.player1_group == PlayerGroup::Stripes {
            let mut p1_balls_left = 0;
            for ball in &self.balls {
                if ball.in_pocket || ball.number == 0 || ball.number == 8 { continue; }
                let def = self.ball_definitions_map.get(&ball.number);
                 if let Some(d) = def {
                    if self.player1_group == PlayerGroup::Solids && !d.is_striped { p1_balls_left += 1; }
                    else if self.player1_group == PlayerGroup::Stripes && d.is_striped { p1_balls_left += 1; }
                }
            }
            if p1_balls_left == 0 { self.player1_group = PlayerGroup::EightBall; }
        }

        if self.player2_group == PlayerGroup::Solids || self.player2_group == PlayerGroup::Stripes {
            let mut p2_balls_left = 0;
            for ball in &self.balls {
                if ball.in_pocket || ball.number == 0 || ball.number == 8 { continue; }
                 let def = self.ball_definitions_map.get(&ball.number);
                 if let Some(d) = def {
                    if self.player2_group == PlayerGroup::Solids && !d.is_striped { p2_balls_left += 1; }
                    else if self.player2_group == PlayerGroup::Stripes && d.is_striped { p2_balls_left += 1; }
                }
            }
            if p2_balls_left == 0 { self.player2_group = PlayerGroup::EightBall; }
        }
        
        let current_player_actual_group = if self.current_player == PlayerId::Player1 { self.player1_group } else { self.player2_group };
        if current_player_actual_group == PlayerGroup::EightBall && 
           (self.message.ends_with("mira.") || self.message.contains("joga novamente")) {
            self.message = format!("{}: Encaçape a BOLA 8 para ganhar!", self.current_player);
        }
    }
}

