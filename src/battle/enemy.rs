use ggez::graphics::{spritebatch, Image, DrawParam, draw, Color};
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use ggez::timer::ticks;
use crate::battle::action::{ActionParameters, DamageType};
use crate::battle::state::BattleState;
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
  spritebatch: spritebatch::SpriteBatch,
  pub screen_pos: (f32, f32),
  pub selection_pos: (usize, usize),
  pub size: f32,
  turn_active: bool,
  opacity: f32,
  pub animation: (Animation, usize, usize),
  pub x_offset: f32,
  pub name: String,
  pub state: BattleState,
  pub dead: bool,
  turn_action: for<'r, 's, 't0, 't1> fn(&'r mut Context, &'s mut Enemy, &'t0 mut Party, &'t1 mut Option<Notification>) -> GameResult<()>
}

impl Enemy {
  pub fn new(
    ctx: &mut Context,
    id: u8,
    spritefile: String,
    screen_pos: (f32, f32),
    selection_pos: (usize, usize),
    size: f32,
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
    poisoned: i8,
    sleeping: i8,
    turn_action: for<'r, 's, 't0, 't1> fn(&'r mut Context, &'s mut Enemy, &'t0 mut Party, &'t1 mut Option<Notification>) -> GameResult<()>,
  ) -> Enemy {
    let image = Image::new(ctx, spritefile).unwrap();
    let batch = spritebatch::SpriteBatch::new(image);
    Enemy {
      spritebatch: batch,
      screen_pos,
      selection_pos,
      size,
      turn_active: false,
      opacity: 1.,
      animation: (Animation::EndTurn, 0, 0),
      x_offset: 0.,
      name,
      state: BattleState::new(id, level, hp, mp, attack, defence, magic, resistance, agility, experience, poisoned, sleeping, None),
      dead: false,
      turn_action
    }
  }

  pub fn update(
    &mut self,
    ctx: &mut Context,
    party: &mut Party,
    active_turns: &mut Vec<u8>,
    current_turn: &mut u8,
    notification: &mut Option<Notification>,
    column_length: usize
  ) -> GameResult<()> {
    self.state.update(current_turn, active_turns)?;
    if *current_turn == self.state.id && !self.turn_active {
      self.turn_active = true;
      let turn_action = self.turn_action;
      turn_action(ctx, self, party, notification)?;
    }
    if self.animation.1 > 0 {
      self.animation.1 -= 1;
      match self.animation.0 {
        Animation::StartTurn(_, _) => {
          let animation_time = ticks(ctx) - self.animation.2;
          if animation_time <= 10 || (animation_time > 30 && animation_time < 50) {
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
            let mut parameters = action_parameters.clone();
            match target_number {
              0 => self.act_on_target(ctx, notification, &mut parameters, &mut party.first )?,
              1 => self.act_on_target(ctx, notification, &mut parameters, &mut party.second)?,
              2 => self.act_on_target(ctx, notification, &mut parameters, &mut party.third )?,
              _ => self.act_on_target(ctx, notification, &mut parameters, &mut party.fourth)?
            }
          },
          Animation::EndTurn => {
            self.turn_active = false;
            *current_turn = 0;
            self.state.end_turn(ctx, notification, &self.name, (
              700. + self.x_offset + self.screen_pos.0 * 70., 250. - column_length as f32 * 33. - self.size * 33. + self.screen_pos.1 * 66.
            ))?;
          },
          Animation::Hurt => {
            self.opacity = 1.;
            if self.state.hp == 0 {
              self.animation = (Animation::Dead, 20, ticks(ctx));
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
    notification: &mut Option<Notification>,
    action_parameters: &mut ActionParameters,
    character: &mut Character
  ) -> GameResult<()> {
    self.animation = (Animation::EndTurn, 30, ticks(ctx));
    match action_parameters.damage_type {
      DamageType::Healing => {character.receive_battle_action(ctx, notification, action_parameters)},
      _ => {
        character.animation = (CharacterAnimation::Hurt, 60, ticks(ctx));
        character.receive_battle_action(ctx, notification, action_parameters)
      }
    }
  }

  pub fn receive_battle_action(
    &mut self,
    ctx: &mut Context,
    notification: &mut Option<Notification>,
    action_parameters: &mut ActionParameters,
    column_length: usize
  ) -> GameResult<()> {
    match action_parameters.damage_type {
      DamageType::None(action) => self.state.receive_none_type_action(action_parameters, action),
      DamageType::Healing      => self.state.receive_healing(),
      _ => {
        self.animation = (Animation::Hurt, 60, ticks(ctx));
        self.state.receive_damage(ctx, notification, &self.name, action_parameters, (
          700. + self.x_offset + self.screen_pos.0 * 70., 250. - column_length as f32 * 33. - self.size * 33. + self.screen_pos.1 * 66.
        ))
      }
    }
  }

  pub fn draw(&mut self, ctx: &mut Context, column_length: usize) -> GameResult<()> {
    self.spritebatch.add(
      match self.animation.0 {
        Animation::Hurt | Animation::Dead => DrawParam::new().color(Color::new(1., 1., 1., self.opacity)),
        _ => DrawParam::new()
      }
    );
    let param = DrawParam::new()
      .dest(Point2::new(700. + self.x_offset + self.screen_pos.0 * 70., 250. - column_length as f32 * 33. - self.size * 33. + self.screen_pos.1 * 66.));
    draw(ctx, &self.spritebatch, param)?;
    self.spritebatch.clear();
    self.state.draw(ctx)
  }
}