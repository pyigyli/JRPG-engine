use ggez::{Context, GameResult};
use ggez::timer::ticks;
pub mod character_info;
pub mod character;
use character::{Character, Animation, Sprite};
use crate::battle::Battle;
use crate::menu::MenuScreen;
use crate::menu::notification::Notification;
use crate::data::{characters, menus};

pub struct Party {
  pub first:  Character,
  pub second: Character,
  pub third:  Character,
  pub fourth: Character
}

impl Party {
  pub fn new(ctx: &mut Context) -> Party {
    Party {
      first:  characters::darrel_deen(ctx, 1),
      second: characters::nurse_seraphine(ctx, 2),
      third:  characters::none_character(ctx, 3),
      fourth: characters::none_character(ctx, 4)
    }
  }

  pub fn update(
    &mut self,
    ctx: &mut Context,
    menu: &mut MenuScreen,
    active_turns: &mut Vec<u8>,
    current_turn: &mut u8,
    notification: &mut Option<Notification>
  ) -> GameResult<()> {
    self.first .update(ctx, menu, active_turns, current_turn, notification)?;
    self.second.update(ctx, menu, active_turns, current_turn, notification)?;
    self.third .update(ctx, menu, active_turns, current_turn, notification)?;
    self.fourth.update(ctx, menu, active_turns, current_turn, notification)?;
    Ok(())
  }

  pub fn won_battle(&mut self, ctx: &mut Context, menu: &mut MenuScreen, ending_tick: usize, experience: &mut u32) -> GameResult<()> {
    if ticks(ctx) - ending_tick < 120 {
      self.first .sprite = Sprite::Victory;
      self.second.sprite = Sprite::Victory;
      self.third .sprite = Sprite::Victory;
      self.fourth.sprite = Sprite::Victory;
    } else if ticks(ctx) - ending_tick == 120 {
      *menu = menus::battle_won(ctx, self, experience);
    }
    Ok(())
  }

  pub fn battle_turn_action(
    &mut self,
    ctx: &mut Context,
    battle: &mut Battle,
    target_pos: (usize, usize),
    menu: &mut MenuScreen,
  ) -> GameResult<()> {
    menu.open = false;
    let mut character = self.get_active();
    character.animation = (Animation::Attack, 60, ticks(ctx));
    character.sprite = Sprite::Attack;
    let attack_power = character.attack;
    match target_pos.0 {
      0 => match target_pos.1 {
        0 => self.first .receive_physical_damage(ctx, attack_power, &mut battle.notification),
        1 => self.second.receive_physical_damage(ctx, attack_power, &mut battle.notification),
        2 => self.third .receive_physical_damage(ctx, attack_power, &mut battle.notification),
        _ => self.fourth.receive_physical_damage(ctx, attack_power, &mut battle.notification),
      }
      _ => battle.enemies[target_pos.0 - 1][target_pos.1].receive_physical_damage(ctx, character.attack, &mut battle.notification)
    }
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.first .draw(ctx, 0.)?;
    self.second.draw(ctx, 1.)?;
    self.third .draw(ctx, 2.)?;
    self.fourth.draw(ctx, 3.)?;
    Ok(())
  }

  pub fn get_active(&mut self) -> &mut Character {
    if self.first.turn_active {
      return &mut self.first;
    } else if self.second.turn_active {
      return &mut self.second;
    } else if self.third.turn_active {
      return &mut self.third;
    }
    &mut self.fourth
  }
}