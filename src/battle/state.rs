use ggez::{Context, GameResult};
use rand::{Rng, thread_rng};
use crate::battle::action::{ActionParameters, DamageType};
use crate::battle::print_damage::PrintDamage;

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
  poisoned: bool,
  sleeping: bool
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
    experience: u32
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
      poisoned: false,
      sleeping: false
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
    let mut rng = thread_rng();
    if rng.gen::<f32>() < action_parameters.dead_change {
      self.hp = 0;
    }
    if rng.gen::<f32>() < action_parameters.poison_change {self.poisoned = true;}
    if rng.gen::<f32>() < action_parameters.sleep_change  {self.sleeping = true;}
    *print_damage = Some(PrintDamage::new(ctx, damage, screen_pos));
    Ok(())
  }

  pub fn receive_healing(&mut self) -> GameResult<()> {
    Ok(())
  }
}