use ggez::{Context, GameResult};
use ggez::timer::ticks;
pub mod action;
use crate::GameMode;
use crate::menu::MenuScreen;
use crate::menu::container::MenuContainer;
pub mod enemy;
use enemy::Enemy;
pub mod print_damage;
pub mod state;
use crate::party::Party;
use crate::party::character::Sprite;
use crate::menu::notification::Notification;
use crate::transition::Transition;

pub struct Battle {
  party_info_container: MenuContainer,
  pub enemies: Vec<Vec<Enemy>>,
  pub enemies_start_draw_height: f32,
  active_turns: Vec<u8>,
  pub current_turn: u8, // 0 = Noone's turn, 1-4 party member's turn, 5 >= enemy's turn
  pub notification: Option<Notification>,
  experience_gained: u32,
  battle_over: (bool, usize)
}

impl Battle {
  pub fn new(ctx: &mut Context, enemies: Vec<Vec<Enemy>>, party: &mut Party, menu: &mut MenuScreen) -> Battle {
    menu.open = false;
    if party.first .state.hp > 0 {party.first.sprite  = Sprite::StandRight;}
    if party.second.state.hp > 0 {party.second.sprite = Sprite::StandRight;}
    if party.third .state.hp > 0 {party.third.sprite  = Sprite::StandRight;}
    if party.fourth.state.hp > 0 {party.fourth.sprite = Sprite::StandRight;}
    let mut max_enemy_column_length = 0.;
    for enemy_column in enemies.iter() {
      let mut column_length = 0.;
      for enemy in enemy_column.iter() {
        column_length += enemy.size;
      }
      if max_enemy_column_length < column_length {
        max_enemy_column_length = column_length;
      }
    }
    Battle {
      party_info_container: MenuContainer::new(ctx, 300., 400., 770., 300.),
      enemies,
      enemies_start_draw_height: 184. - max_enemy_column_length * 16.,
      active_turns: Vec::new(),
      current_turn: 0,
      notification: None,
      experience_gained: 0,
      battle_over: (false, 0)
    }
  }

  pub fn update(
    &mut self,
    ctx: &mut Context,
    mode: &mut GameMode,
    party: &mut Party,
    menu: &mut MenuScreen,
    battle_menu: &mut MenuScreen,
    transition: &mut Transition
  ) -> GameResult<()> {
    if *mode == GameMode::Battle {
      if self.battle_over.0 {
        if ticks(ctx) - self.battle_over.1 > 60 {
          party.won_battle(ctx, menu, self.battle_over.1, &mut self.experience_gained, transition)?;
        }
      } else {
        if self.current_turn == 0 {
          if let Some(next_turn) = self.active_turns.pop() {
            self.current_turn = next_turn;
          }
        }
        party.update(ctx, battle_menu, &mut self.active_turns, &mut self.current_turn, &mut self.notification, transition)?;
        battle_menu.update(ctx, mode, party, self, transition)?;
      }
      let mut dead_enemies = Vec::new();
      for (i, enemy_column) in self.enemies.iter_mut().enumerate() {
        for (j, enemy) in enemy_column.iter_mut().enumerate() {
          enemy.update(ctx, party, &mut self.active_turns, &mut self.current_turn, &mut self.notification, self.enemies_start_draw_height)?;
          if enemy.dead == true {
            dead_enemies.push((i, j));
            self.experience_gained += enemy.state.experience;
          } else if enemy.state.hp == 0 {
            enemy.selection_pos.0 = 0; // turns enemy untargetable, due to party occupying column 0
          }
        }
      }
      for position in dead_enemies {
        if self.enemies[position.0].len() > 1 {
          self.enemies[position.0].remove(position.1);
          for enemy in &mut self.enemies[position.0] {
            if enemy.selection_pos.1 > position.1 {
              enemy.selection_pos.1 -= 1;
            }
          }
        } else {
          self.enemies.remove(position.0);
          for enemy in self.enemies.iter_mut().flatten() {
            if enemy.selection_pos.0 > position.0 {
              enemy.selection_pos.0 -= 1;
            }
          }
        }
      }
      if self.enemies.len() == 0 && !self.battle_over.0 {
        self.battle_over = (true, ticks(ctx));
      }
      if let Some(notification) = &mut self.notification {
        if notification.show_time > 0 {
          notification.show_time -= 1;
        } else {
          self.notification = None;
        }
      }
    }
    Ok(())
  }

  pub fn draw(&mut self, ctx: &mut Context, party: &mut Party, battle_menu: &mut MenuScreen) -> GameResult<()> {
    self.party_info_container.draw(ctx)?;
    party.draw(ctx)?;
    for enemy_column in self.enemies.iter_mut() {
      for enemy in enemy_column.iter_mut() {
        enemy.draw(ctx, self.enemies_start_draw_height)?;
      }
    }
    battle_menu.draw(ctx)?;
    if let Some(notification) = &mut self.notification {
      notification.draw(ctx)?;
    }
    Ok(())
  }
}