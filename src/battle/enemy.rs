use ggez::graphics::{spritebatch, Image, DrawParam, draw, Color};
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use ggez::timer::ticks;
use crate::party::Party;
use crate::party::character::{Character, Animation as CharacterAnimation};
use crate::menu::notification::Notification;

pub enum Animation {
  StartTurn(u8), // 60 frames
  EndTurn, // 30 frames
  Hurt, // 60 frames
  Dead // 20 frames
}

impl PartialEq for Animation {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Animation::StartTurn(_) => {match other {Animation::StartTurn(_) => true, _ => false}},
      Animation::EndTurn      => {match other {Animation::EndTurn      => true, _ => false}},
      Animation::Hurt         => {match other {Animation::Hurt         => true, _ => false}},
      Animation::Dead         => {match other {Animation::Dead         => true, _ => false}}
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
  hp: u16,
  mp: u16,
  attack: u16,
  defence: u16,
  magic: u16,
  resistance: u16,
  agility: u8,
  pub experience: u32,
  atb: u8,
  atb_subtick: u8,
  turn_action: for<'r, 's, 't0, 't1> fn(&'r mut Context, &'s mut Enemy, &'t0 mut Party, &'t1 mut Option<Notification>) -> GameResult<()>,
  pub dead: bool
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
      turn_action,
      dead: false
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
        Animation::StartTurn(_) => {
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
        match self.animation.0 {
          Animation::StartTurn(target_number) => {
            match target_number {
              0 => self.attack_target(ctx, &mut party.first , notification)?,
              1 => self.attack_target(ctx, &mut party.second, notification)?,
              2 => self.attack_target(ctx, &mut party.third , notification)?,
              _ => self.attack_target(ctx, &mut party.fourth, notification)?
            }
            self.animation = (Animation::EndTurn, 30, ticks(ctx));
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

  pub fn attack_target(&mut self, ctx: &mut Context, character: &mut Character, notification: &mut Option<Notification>) -> GameResult<()> {
    character.animation = (CharacterAnimation::Hurt, 60, ticks(ctx));
    character.receive_physical_damage(ctx, character.attack, notification)
  }

  pub fn receive_physical_damage(&mut self, ctx: &mut Context, attack: u16, notification: &mut Option<Notification>) -> GameResult<()> {
    let damage = attack * 3 / self.defence;
    if let Some(hp) = self.hp.checked_sub(damage) {
      self.hp = hp;
    } else {
      self.hp = 0;
    }
    self.animation = (Animation::Hurt, 60, ticks(ctx));
    *notification = Some(Notification::new(ctx, format!("{} takes {} dmg", self.name, damage)));
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