use ggez::graphics::{spritebatch, Image, DrawParam, Rect, draw, Color};
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use ggez::timer::ticks;
use crate::party::character_info::CharacterInfo;
use crate::menu::MenuScreen;
use crate::menu::item::{MenuItem, OnClickEvent};
use crate::menu::notification::Notification;
use crate::data;

pub enum Animation {
  StartTurn, // 12 frames
  EndTurn, // 12 frames
  Attack, // 60 frames
  Hurt, // 60 frames
  Dead // 20 frames
}

pub enum Sprite {
  StandLeft,
  WalkLeft,
  StandRight,
  WalkRight,
  StandUp,
  WalkUp,
  StandDown,
  WalkDown,
  Dead,
  Attack,
  Victory
}

pub struct Character {
  pub id: u8,
  spritebatch: spritebatch::SpriteBatch,
  opacity: f32,
  pub animation: (Animation, usize, usize), // (Animation, length, starting tick)
  pub sprite: Sprite,
  frame: f32,
  pub x_offset: f32,
  pub name: String,
  level: u8,
  pub experience: u32,
  hp: u16,
  mp: u16,
  pub attack: u16,
  defence: u16,
  magic: u16,
  resistance: u16,
  agility: u8,
  atb: u8,
  atb_subtick: u8,
  pub turn_active: bool,
  dead: bool,
  character_info: CharacterInfo
}

impl Character {
  pub fn new(
    ctx: &mut Context,
    id: u8,
    spritefile: String,
    name: String,
    level: u8,
    hp: u16,
    mp: u16,
    attack: u16,
    defence: u16,
    magic: u16,
    resistance: u16,
    agility: u8
  ) -> Character {
    let image = Image::new(ctx, spritefile).unwrap();
    let batch = spritebatch::SpriteBatch::new(image);
    let info_name = MenuItem::new(ctx, "/empty.png".to_owned(), name.to_owned(), (300., 480. + id as f32 * 30.), OnClickEvent::None);
    Character {
      id,
      spritebatch: batch,
      opacity: 1.,
      animation: (Animation::EndTurn, 0, 0),
      sprite: Sprite::StandRight,
      frame: 0.,
      x_offset: 0.,
      name,
      experience: 0,
      level,
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
      dead: false,
      character_info: CharacterInfo::new()
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
    if self.name.len() > 0 {
      if self.dead {
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
      } else if *current_turn == self.id && !self.turn_active {
        self.turn_active = true;
        self.sprite = Sprite::StandRight;
        self.sprite = Sprite::WalkRight;
        self.animation = (Animation::StartTurn, 12, ticks(ctx));
      }
      if self.animation.1 > 0 {
        self.animation.1 -= 1;
        match (ticks(ctx) - self.animation.2) % 12 {
          2  => self.frame = 1.,
          5  => self.frame = 0.,
          8  => self.frame = 2.,
          11 => self.frame = 0.,
          _  => ()
        }
        match self.animation.0 {
          Animation::StartTurn => self.x_offset += 3.,
          Animation::EndTurn   => self.x_offset -= 3.,
          Animation::Hurt => {
            let animation_time = ticks(ctx) - self.animation.2;
            if animation_time > 10 && animation_time <= 30 {
              if (animation_time) % 10 == 0 {
                self.opacity = 1.;
              } else if (animation_time) % 5 == 0 {
                self.opacity = 0.;
              }
            }
          },
          _ => ()
        };
        if self.animation.1 == 0 {
          self.frame = 0.;
          match self.animation.0 {
            Animation::StartTurn => {
              self.sprite = Sprite::StandRight;
              *menu = data::menus::battle_main(ctx);
            },
            Animation::EndTurn => {
              self.sprite = Sprite::StandRight;
              self.turn_active = false;
              *current_turn = 0;
            },
            Animation::Attack => {
              self.animation = (Animation::EndTurn, 12, ticks(ctx));
              self.sprite = Sprite::WalkLeft;
            },
            Animation::Hurt => {
              self.opacity = 1.;
              if self.hp == 0 {
                self.animation = (Animation::Dead, 20, ticks(ctx));
                *notification = Some(Notification::new(ctx, format!("{} dead", self.name)));
              } else if self.turn_active {
                self.animation = (Animation::EndTurn, 12, ticks(ctx));
                self.sprite = Sprite::WalkLeft;
              }
            },
            Animation::Dead => {
              self.sprite = Sprite::Dead;
              self.dead = true;
            }
          }
        }
      }
    }
    Ok(())
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

  pub fn draw(&mut self, ctx: &mut Context, party_pos: f32) -> GameResult<()> {
    if self.name.len() > 0 {
      let (spritesheet_x, spritesheet_y, anim_loop_len) = match self.sprite {
        Sprite::StandLeft  => (0., 1., 1.),
        Sprite::WalkLeft   => (0., 1., 3.),
        Sprite::StandRight => (0., 2., 1.),
        Sprite::WalkRight  => (0., 2., 3.),
        Sprite::StandUp    => (0., 3., 1.),
        Sprite::WalkUp     => (0., 3., 3.),
        Sprite::StandDown  => (0., 0., 1.),
        Sprite::WalkDown   => (0., 0., 3.),
        Sprite::Dead       => (0., 4., 1.),
        Sprite::Attack     => (1., 4., 1.),
        Sprite::Victory    => (2., 4., 1.)
      };
      let p = DrawParam::new()
        .src(Rect::new(0.333333333333333 * (spritesheet_x + (self.frame % anim_loop_len)), 0.2 * spritesheet_y, 0.333333333333333, 0.2))
        .color(Color::new(1., 1., 1., self.opacity));
      self.spritebatch.add(p);
      let param = DrawParam::new()
        .dest(Point2::new(200. + self.x_offset, 150. + party_pos * 65.));
      draw(ctx, &self.spritebatch, param)?;
      self.spritebatch.clear();
    }
    Ok(())
  }
}