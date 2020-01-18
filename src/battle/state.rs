use ggez::{Context, GameResult};
use ggez::graphics::Color;
use rand::{Rng, thread_rng};
use crate::battle::action::{ActionParameters, DamageType};
use crate::battle::print_damage::PrintDamage;
use crate::party::InventoryElement;
use crate::party::character_info::CharacterInfo;
use crate::party::item::InventoryItem;
use crate::menu::notification::Notification;
use std::cmp::{min, max};

pub struct BattleState {
  pub id: u8,
  level: u8,
  pub experience: u32,
  pub max_hp: u16,
  pub max_mp: u16,
  pub hp: u16,
  pub mp: u16,
  pub attack: u16,
  defence: u16,
  magic: u16,
  resistance: u16,
  agility: u8,
  atb: u8,
  atb_subtick: u8,
  pub turn_active: bool,
  poisoned: i8, // effected for x turns, negative means immunity, 128 and -127 means effect for eternity
  sleeping: i8, // effected for x turns, negative means immunity, 128 and -127 means effect for eternity
  pub common_steal: Option<InventoryItem>,
  pub rare_steal: Option<InventoryItem>,
  pub character_info: Option<CharacterInfo>,
  pub print_damage: Option<PrintDamage>
}

impl BattleState {
  pub fn new(
    id: u8,
    level: u8,
    hp: u16,
    mp: u16,
    attack: u16,
    defence: u16,
    magic: u16,
    resistance: u16,
    agility: u8,
    experience: u32,
    common_steal: Option<InventoryItem>,
    rare_steal: Option<InventoryItem>,
    poisoned: i8,
    sleeping: i8,
    character_info: Option<CharacterInfo>
  ) -> BattleState {
    BattleState {
      id,
      level,
      experience,
      max_hp: hp,
      max_mp: mp,
      hp,
      mp,
      attack,
      defence,
      magic,
      resistance,
      agility,
      atb: 0,
      atb_subtick: 0,
      turn_active: false,
      poisoned,
      sleeping,
      common_steal,
      rare_steal,
      character_info,
      print_damage: None
    }
  }

  pub fn update(&mut self, current_turn: &mut u8, active_turns: &mut Vec<u8>) -> GameResult<()> {
    if self.hp == 0 {
      self.atb = 0;
      self.atb_subtick = 0;
    } else if *current_turn == 0 {
      self.atb_subtick += 1;
      if self.atb_subtick % 5 == 0 {
        self.atb_subtick = 0;
        if let Some(sum) = self.atb.checked_add(self.agility) {
          self.atb = sum;
        } else {
          active_turns.push(self.id);
          self.atb = 0;
        }
      }
    }
    if let Some(print_damage) = &mut self.print_damage {
      if print_damage.show_time > 0. {
        print_damage.update()?;
      } else {
        self.print_damage = None;
      }
    }
    Ok(())
  }

  pub fn end_turn(
    &mut self,
    ctx: &mut Context,
    notification: &mut Option<Notification>,
    name: &String,
    position: (f32, f32)
  ) -> GameResult<()> {
    if self.poisoned > 0 && self.poisoned != 126 {
      self.poisoned -= 1;
      let poison_damage = max(self.hp / 20, 1);
      *notification = Some(Notification::new(ctx, format!("{} took {} poison damage", name, poison_damage)));
      if let Some(hp) = self.hp.checked_sub(poison_damage) {
        self.hp = hp;
      } else {
        self.hp = 0;
      }
      self.print_damage = Some(PrintDamage::new(ctx, poison_damage, self.get_damage_position(position), Color::new(1., 1., 1., 1.)));
      if let Some(info) = &mut self.character_info {
        if self.poisoned == 0 {info.remove_effect("poison".to_owned())?;}
        info.hp.text = format!("{}/", self.hp);
      }
    } else if self.poisoned < 0 && self.poisoned != -125 {
      self.poisoned += 1;
      if let Some(info) = &mut self.character_info {
        if self.poisoned == 0 {info.remove_effect("poison".to_owned())?;}
      }
    }
    if self.sleeping > 0 && self.sleeping != 126 {
      self.sleeping -= 1;
      if let Some(info) = &mut self.character_info {
        if self.sleeping == 0 {info.remove_effect("poison".to_owned())?;}
      }
    } else if self.sleeping < 0 && self.sleeping != -125 {
      self.sleeping += 1;
      if let Some(info) = &mut self.character_info {
        if self.sleeping == 0 {info.remove_effect("poison".to_owned())?;}
      }
    }
    Ok(())
  }

  pub fn receive_damage(
    &mut self,
    ctx: &mut Context,
    notification: &mut Option<Notification>,
    name: &String,
    action_parameters: &ActionParameters,
    position: (f32, f32)
  ) -> GameResult<()> {
    let damage = match action_parameters.damage_type {
      DamageType::Physical => action_parameters.power * 3 / self.defence,
      DamageType::Magical  => action_parameters.power * 3 / self.resistance,
      DamageType::Pure     => action_parameters.power * 3,
      _ => 0
    };
    if let Some(hp) = self.hp.checked_sub(damage) {
      self.hp = hp;
    } else {
      self.hp = 0;
    }
    if let Some(info) = &mut self.character_info {
      info.hp.text = format!("{}/", self.hp);
    }
    let mut rng = thread_rng();
    if rng.gen::<f32>() < action_parameters.dead_change   {
      self.hp = 0;
      if let Some(info) = &mut self.character_info {
        info.hp.text = format!("{}", 0);
      }
    }
    if self.poisoned >= 0 && rng.gen::<f32>() < action_parameters.poison_change {
      if let Some(info) = &mut self.character_info {
        if self.poisoned == 0 {
          info.set_effect(ctx, self.id, "poison".to_owned())?;
        }
      }
      self.poisoned = 5;
      *notification = Some(Notification::new(ctx, format!("{} is poisoned", name)))
    }
    if self.sleeping >= 0 && rng.gen::<f32>() < action_parameters.sleep_change {
      if let Some(info) = &mut self.character_info {
        if self.sleeping == 0 {
          info.set_effect(ctx, self.id, "sleep".to_owned())?;
        }
      }
      self.sleeping = 3;
    }
    self.print_damage = Some(PrintDamage::new(ctx, damage, self.get_damage_position(position), Color::new(1., 1., 1., 1.)));
    Ok(())
  }

  pub fn receive_healing(
    &mut self,
    ctx: &mut Context,
    notification: &mut Option<Notification>,
    name: &String,
    action_parameters: &ActionParameters,
    position: (f32, f32)
  ) -> GameResult<()> {
    let heal_amount = action_parameters.power * self.magic;
    self.hp = min(self.hp + heal_amount, self.max_hp);
    if let Some(info) = &mut self.character_info {info.hp.text = format!("{}/", self.hp);}
    self.print_damage = Some(PrintDamage::new(ctx, heal_amount, self.get_damage_position(position), Color::new(0., 1., 0., 1.)));
    Ok(())
  }

  pub fn receive_none_type_action(
    &mut self,
    ctx: &mut Context,
    inventory: &mut Vec<InventoryElement>,
    action_parameters: &ActionParameters,
    action: for<'r, 's, 't1, 't2, 't3> fn(
      &'r mut Context,
      &'s mut Vec<InventoryElement>,
      &'t1 ActionParameters,
      &'t2 mut BattleState,
      &'t3 mut Option<Notification>
    ) -> GameResult<()>,
    notification: &mut Option<Notification>
  ) -> GameResult<()> {
    action(ctx, inventory, action_parameters, self, notification)
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    if let Some(character_info) = &mut self.character_info {
      character_info.draw(ctx)?;
    }
    if let Some(print_damage) = &mut self.print_damage {
      print_damage.draw(ctx)?;
    }
    Ok(())
  }

  pub fn get_damage_position(&self, position: (f32, f32)) -> (f32, f32) {
    let mut damage_position = position;
    if let Some(_) = &self.character_info {
      damage_position.0 += 40.;
    }
    damage_position
  }
}