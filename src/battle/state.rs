use ggez::{Context, GameResult};
use rand::{Rng, thread_rng};
use crate::battle::action::{ActionParameters, DamageType};
use crate::battle::print_damage::PrintDamage;
use crate::party::character_info::CharacterInfo;

pub struct BattleState {
  pub id: u8,
  level: u8,
  pub experience: u32,
  max_hp: u16,
  max_mp: u16,
  pub hp: u16,
  mp: u16,
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
  character_info: Option<CharacterInfo>
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
      character_info
    }
  }

  pub fn atb_update(&mut self, current_turn: &mut u8, active_turns: &mut Vec<u8>) -> GameResult<()> {
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
    Ok(())
  }

  pub fn end_turn(&mut self) -> GameResult<()> {
    if self.poisoned > 0 && self.poisoned != 126 {
      self.poisoned -= 1;
      if let Some(info) = &mut self.character_info {
        if self.poisoned == 0 {info.remove_effect("poison".to_owned())?;}
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
    action_parameters: &mut ActionParameters,
    print_damage: &mut Option<PrintDamage>,
    screen_pos: (f32, f32)
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
    }
    if self.sleeping >= 0 && rng.gen::<f32>() < action_parameters.sleep_change {
      if let Some(info) = &mut self.character_info {
        if self.sleeping == 0 {
          info.set_effect(ctx, self.id, "sleep".to_owned())?;
        }
      }
      self.sleeping = 3;
    }
    *print_damage = Some(PrintDamage::new(ctx, damage, screen_pos));
    Ok(())
  }

  pub fn receive_healing(&mut self) -> GameResult<()> {
    Ok(())
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    if let Some(character_info) = &mut self.character_info {
      character_info.draw(ctx)?;
    }
    Ok(())
  }
}