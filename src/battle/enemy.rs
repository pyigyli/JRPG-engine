use ggez::graphics::{spritebatch, Image, DrawParam, draw, Color};
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use ggez::timer::ticks;
use rand::{Rng, thread_rng};
use crate::battle::action::{ActionParameters, DamageType};
use crate::party::Party;
use crate::party::character::{Character, Animation as CharacterAnimation};
use crate::menu::notification::Notification;

pub enum Animation {
  StartTurn(u8, ActionParameters), // 60 frames
  EndTurn, // 30 frames
  Hurt, // 60 frames
  Dead // 20 frames
}

impl PartialEq for Animation {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Animation::StartTurn(_, _) => {match other {Animation::StartTurn(_, _) => true, _ => false}},
      Animation::EndTurn         => {match other {Animation::EndTurn         => true, _ => false}},
      Animation::Hurt            => {match other {Animation::Hurt            => true, _ => false}},
      Animation::Dead            => {match other {Animation::Dead            => true, _ => false}}
    }
  }
}

pub struct Enemy {
  id: u8,
  spritebatch: spritebatch::SpriteBatch,
  pub screen_pos: (f32, f32),
  pub selection_pos: (usize, usize),
  turn_active: bool,
  opacity: f32,
  pub animation: (Animation, usize, usize),
  pub x_offset: f32,
  pub name: String,
  level: u8,
  max_hp: u16,
  max_mp: u16,
  hp: u16,
  mp: u16,
  pub attack: u16,
  defence: u16,
  pub magic: u16,
  resistance: u16,
  agility: u8,
  pub experience: u32,
  atb: u8,
  atb_subtick: u8,
  pub dead: bool,
  poisoned: bool,
  sleeping: bool,
  turn_action: for<'r, 's, 't0, 't1> fn(&'r mut Context, &'s mut Enemy, &'t0 mut Party, &'t1 mut Option<Notification>) -> GameResult<()>
}

impl Enemy {
  pub fn new(
    ctx: &mut Context,
    id: u8,
    spritefile: String,
    screen_pos: (f32, f32),
    selection_pos: (usize, usize),
    name: String,
    level: u8,
    hp: u16,
    mp: u16,
    attack: u16,
    defence: u16,
    magic: u16,
    resistance: u16,
    agility: u8,
    experience: u32,
    turn_action: for<'r, 's, 't0, 't1> fn(&'r mut Context, &'s mut Enemy, &'t0 mut Party, &'t1 mut Option<Notification>) -> GameResult<()>,
  ) -> Enemy {
    let image = Image::new(ctx, spritefile).unwrap();
    let batch = spritebatch::SpriteBatch::new(image);
    Enemy {
      id,
      spritebatch: batch,
      screen_pos,
      selection_pos,
      turn_active: false,
      opacity: 1.,
      animation: (Animation::EndTurn, 0, 0),
      x_offset: 0.,
      name,
      level,
      max_hp: hp,
      max_mp: mp,
      hp,
      mp,
      attack,
      defence,
      magic,
      resistance,
      agility,
      experience,
      atb: 0,
      atb_subtick: 0,
      dead: false,
      poisoned: false,
      sleeping: false,
      turn_action
    }
  }

  pub fn update(
    &mut self,
    ctx: &mut Context,
    party: &mut Party,
    active_turns: &mut Vec<u8>,
    current_turn: &mut u8,
    notification: &mut Option<Notification>
  ) -> GameResult<()> {
    if *current_turn == 0 && !self.dead {
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
    } else if *current_turn == self.id && !self.turn_active {
      let turn_action = self.turn_action;
      turn_action(ctx, self, party, notification)?;
      self.turn_active = true;
    }
    if self.animation.1 > 0 {
      self.animation.1 -= 1;
      match self.animation.0 {
        Animation::StartTurn(_, _) => {
          let animation_time = ticks(ctx) - self.animation.2;
          if animation_time < 10 || (animation_time > 30 && animation_time < 50) {
            self.x_offset -= 2.;
          } else {
            self.x_offset += 2.;
          }
        },
        Animation::EndTurn => (),
        Animation::Hurt => {
          let animation_time = ticks(ctx) - self.animation.2;
          if animation_time > 10 && animation_time <= 30 {
            if animation_time % 10 == 0 {
              self.opacity = 1.;
            } else if animation_time % 5 == 0 {
              self.opacity = 0.;
            }
          }
        },
        Animation::Dead => self.opacity -= 0.05
      }
      if self.animation.1 == 0 {
        match &mut self.animation.0 {
          Animation::StartTurn(target_number, action_parameters) => {
            let parameters = action_parameters.clone();
            match target_number {
              0 => self.act_on_target(ctx, parameters, &mut party.first , notification)?,
              1 => self.act_on_target(ctx, parameters, &mut party.second, notification)?,
              2 => self.act_on_target(ctx, parameters, &mut party.third , notification)?,
              _ => self.act_on_target(ctx, parameters, &mut party.fourth, notification)?
            }
          },
          Animation::EndTurn => {
            self.turn_active = false;
            *current_turn = 0
          },
          Animation::Hurt => {
            self.opacity = 1.;
            if self.hp == 0 {
              self.animation = (Animation::Dead, 20, ticks(ctx));
              *notification = Some(Notification::new(ctx, format!("{} dead", self.name)));
            }
          },
          Animation::Dead => {
            self.opacity = 0.;
            self.dead = true;
          }
        }
      }
    }
    Ok(())
  }

  pub fn act_on_target(
    &mut self,
    ctx: &mut Context,
    action_parameters: ActionParameters,
    character: &mut Character,
    notification: &mut Option<Notification>
  ) -> GameResult<()> {
    self.animation = (Animation::EndTurn, 30, ticks(ctx));
    match action_parameters.damage_type {
      DamageType::Healing => {character.receive_healing()},
      _ => {
        character.animation = (CharacterAnimation::Hurt, 60, ticks(ctx));
        character.receive_damage(ctx, action_parameters, notification)
      }
    }   
  }

  pub fn receive_damage(&mut self, ctx: &mut Context, action_parameters: &mut ActionParameters, notification: &mut Option<Notification>) -> GameResult<()> {
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
    if rng.gen::<f32>() < action_parameters.dead_change   {self.hp = 0;}
    if rng.gen::<f32>() < action_parameters.poison_change {self.poisoned = true;}
    if rng.gen::<f32>() < action_parameters.sleep_change  {self.sleeping = true;}
    self.animation = (Animation::Hurt, 60, ticks(ctx));
    *notification = Some(Notification::new(ctx, format!("{} takes {} dmg", self.name, damage)));
    Ok(())
  }

  pub fn receive_healing() -> GameResult<()> {
    Ok(())
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.spritebatch.add(
      match self.animation.0 {
        Animation::Hurt | Animation::Dead => DrawParam::new().color(Color::new(1., 1., 1., self.opacity)),
        _ => DrawParam::new()
      }
    );
    let param = DrawParam::new()
      .dest(Point2::new(700. + self.x_offset + self.screen_pos.0 * 70., 200. + self.screen_pos.1 * 65.));
    draw(ctx, &self.spritebatch, param)?;
    self.spritebatch.clear();
    Ok(())
  }
}