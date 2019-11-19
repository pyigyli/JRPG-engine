use ggez::{Context, GameResult};
use ggez::timer::ticks;
pub mod character_info;
pub mod character;
use character::{Character, Animation, Sprite};
pub mod item;
use item::{InventoryItem, ItemVariant};
use crate::battle::Battle;
use crate::battle::action::ActionParameters;
use crate::menu::MenuScreen;
use crate::menu::notification::Notification;
use crate::GameMode;
use crate::transition::{Transition, TransitionStyle};
use crate::data::{characters, menus};

pub enum InventoryElement {
  Item(InventoryItem, u8)
}

pub struct Party {
  pub first:  Character,
  pub second: Character,
  pub third:  Character,
  pub fourth: Character,
  pub inventory: Vec<InventoryElement>
}

impl Party {
  pub fn new(ctx: &mut Context) -> Party {
    Party {
      first:  characters::darrel_deen(ctx, 1),
      second: characters::nurse_seraphine(ctx, 2),
      third:  characters::none_character(ctx, 3),
      fourth: characters::none_character(ctx, 4),
      inventory: vec![ // Must have value 0 or more for every item
        InventoryElement::Item(InventoryItem::new(ItemVariant::Potion), 2),
        InventoryElement::Item(InventoryItem::new(ItemVariant::Ether) , 1)
      ]
    }
  }

  pub fn update(
    &mut self,
    ctx: &mut Context,
    battle_menu: &mut MenuScreen,
    active_turns: &mut Vec<u8>,
    current_turn: &mut u8,
    notification: &mut Option<Notification>
  ) -> GameResult<()> {
    self.first .update(ctx, battle_menu, active_turns, current_turn, notification)?;
    self.second.update(ctx, battle_menu, active_turns, current_turn, notification)?;
    self.third .update(ctx, battle_menu, active_turns, current_turn, notification)?;
    self.fourth.update(ctx, battle_menu, active_turns, current_turn, notification)?;
    Ok(())
  }

  pub fn won_battle(
    &mut self,
    ctx: &mut Context,
    menu: &mut MenuScreen,
    ending_tick: usize,
    experience: &mut u32,
    transition: &mut Transition
  ) -> GameResult<()> {
    if ticks(ctx) - ending_tick < 120 {
      if self.first .state.hp > 0 {self.first. sprite = Sprite::Victory}
      if self.second.state.hp > 0 {self.second.sprite = Sprite::Victory}
      if self.third .state.hp > 0 {self.third .sprite = Sprite::Victory}
      if self.fourth.state.hp > 0 {self.fourth.sprite = Sprite::Victory}
    } else if ticks(ctx) - ending_tick == 120 {
      *menu = menus::battle_won(ctx, self, experience);
      transition.set(TransitionStyle::BlackInFast(GameMode::Menu))?;
    }
    Ok(())
  }

  pub fn battle_turn_action(
    &mut self,
    ctx: &mut Context,
    battle: &mut Battle,
    target_pos: (usize, usize),
    action_parameters: &mut ActionParameters
  ) -> GameResult<()> {
    let mut character = self.get_active();
    character.animation = (Animation::Attack, 60, ticks(ctx));
    character.sprite = Sprite::Attack;
    match target_pos.0 {
      0 => match target_pos.1 {
        0 => self.first .receive_battle_action(ctx, &mut self.inventory, &mut battle.notification, action_parameters),
        1 => self.second.receive_battle_action(ctx, &mut self.inventory, &mut battle.notification, action_parameters),
        2 => self.third .receive_battle_action(ctx, &mut self.inventory, &mut battle.notification, action_parameters),
        _ => self.fourth.receive_battle_action(ctx, &mut self.inventory, &mut battle.notification, action_parameters),
      }
      _ => battle.enemies[target_pos.0 - 1][target_pos.1].receive_battle_action(
        ctx, &mut self.inventory, &mut battle.notification, action_parameters, battle.enemies_start_draw_height
      )
    }
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.first .draw(ctx)?;
    self.second.draw(ctx)?;
    self.third .draw(ctx)?;
    self.fourth.draw(ctx)?;
    Ok(())
  }

  pub fn get_alive_size(&mut self) -> u32 {
    let mut party_size = 0;
    if self.first .name.len() > 0 && self.first .state.hp > 0 {party_size += 1;}
    if self.second.name.len() > 0 && self.second.state.hp > 0 {party_size += 1;}
    if self.third .name.len() > 0 && self.third .state.hp > 0 {party_size += 1;}
    if self.fourth.name.len() > 0 && self.fourth.state.hp > 0 {party_size += 1;}
    party_size
  }

  pub fn get_active(&mut self) -> &mut Character {
    if self.first.state.turn_active {
      return &mut self.first;
    } else if self.second.state.turn_active {
      return &mut self.second;
    } else if self.third.state.turn_active {
      return &mut self.third;
    }
    &mut self.fourth
  }
}