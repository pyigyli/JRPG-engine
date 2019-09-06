use ggez::{Context, GameResult};
use ggez::timer::ticks;
pub mod action;
use crate::GameMode;
use crate::menu::MenuScreen;
use crate::menu::container::MenuContainer;
pub mod enemy;
use enemy::Enemy;
use crate::party::Party;
use crate::party::character::Sprite;
use crate::menu::notification::Notification;

pub struct Battle {
  party_info_container: MenuContainer,
  pub enemies: Vec<Vec<Enemy>>,
  active_turns: Vec<u8>,
  pub current_turn: u8, // 0 = Noone's turn, 1..4 party member's turn, 5 >= enemy's turn
  pub notification: Option<Notification>,
  experience_gained: u32,
  battle_over: (bool, usize)
}

impl Battle {
  pub fn new(ctx: &mut Context, enemies: Vec<Vec<Enemy>>, party: &mut Party, menu: &mut MenuScreen) -> Battle {
    menu.open = false;
    party.first.sprite  = Sprite::StandRight;
    party.second.sprite = Sprite::StandRight;
    party.third.sprite  = Sprite::StandRight;
    party.fourth.sprite = Sprite::StandRight;
    Battle {
      party_info_container: MenuContainer::new(ctx, 300., 480., 750., 220.),
      enemies,
      active_turns: vec![],
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
  ) -> GameResult<()> {
    if *mode == GameMode::Battle {
      if self.battle_over.0 {
        if ticks(ctx) - self.battle_over.1 > 60 {
          party.won_battle(ctx, menu, self.battle_over.1, &mut self.experience_gained)?;
        }
      } else {
        if self.current_turn == 0 {
          if let Some(next_turn) = self.active_turns.pop() {
            self.current_turn = next_turn;
          }
        }
      }
      party.update(ctx, menu, &mut self.active_turns, &mut self.current_turn, &mut self.notification)?;
      let mut dead_enemies = vec![];
      for (i, enemy_column) in self.enemies.iter_mut().enumerate() {
        for (j, enemy) in enemy_column.iter_mut().enumerate() {
          enemy.update(ctx, party, &mut self.active_turns, &mut self.current_turn, &mut self.notification)?;
          if enemy.dead {
            dead_enemies.push((i, j));
            self.experience_gained += enemy.experience;
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

  pub fn draw(&mut self, ctx: &mut Context, party: &mut Party) -> GameResult<()> {
    self.party_info_container.draw(ctx)?;
    party.draw(ctx)?;
    for enemy in self.enemies.iter_mut().flatten() {
      enemy.draw(ctx)?;
    }
    if let Some(notification) = &mut self.notification {
      notification.draw(ctx)?;
    }
    Ok(())
  }
}