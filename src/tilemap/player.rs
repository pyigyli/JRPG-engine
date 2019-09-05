use ggez::graphics::{spritebatch, Image, DrawParam, Rect, draw};
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::timer::ticks;
use rand::{Rng, thread_rng};
use crate::globals::WINDOW_SIZE;
use crate::tilemap::tile::{Tile, EntityOnTile};
use crate::GameMode;
use crate::transition::{Transition, TransitionStyle};
use crate::party::Party;
use crate::battle::Battle;
use crate::battle::enemy::Enemy;
use crate::menu::MenuScreen;

enum PlayerAnimation {
  StandLeft,
  WalkLeft,
  StandRight,
  WalkRight,
  StandUp,
  WalkUp,
  StandDown,
  WalkDown
}

pub struct Player {
  pub position: (f32, f32),
  spritebatch: spritebatch::SpriteBatch,
  animation: PlayerAnimation,
  frame: f32,
  finish_animation: (PlayerAnimation, usize, usize)
}

impl Player {
  pub fn new(ctx: &mut Context, spritefile: String, position: (f32, f32)) -> Player {
    let image = Image::new(ctx, spritefile).unwrap();
    let batch = spritebatch::SpriteBatch::new(image);
    Player {
      position,
      spritebatch: batch,
      animation: PlayerAnimation::StandDown,
      frame: 0.,
      finish_animation: (PlayerAnimation::StandDown, 0, 0)
    }
  }

  pub fn update(
    &mut self,
    ctx: &mut Context,
    tiles: &mut Vec<Vec<Tile>>,
    mode: &mut GameMode,
    party: &mut Party,
    battle: &mut Battle,
    transition: &mut Transition,
    menu: &mut MenuScreen,
    encounter_rate: f32,
    enemy_formations: for<'r> fn(&'r mut Context) -> Vec<Vec<Enemy>>
  ) -> GameResult<()> {
    if *mode == GameMode::Map {
      if self.finish_animation.1 == 0 {
        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
          self.animation = PlayerAnimation::WalkUp;
          if tiles[self.position.1 as usize - 1][self.position.0 as usize].entity == EntityOnTile::None {
            tiles[self.position.1 as usize - 1][self.position.0 as usize].entity = EntityOnTile::Player;
            self.finish_animation = (PlayerAnimation::WalkUp, 12, ticks(ctx));
          }
        } else if keyboard::is_key_pressed(ctx, KeyCode::Down) {
          self.animation = PlayerAnimation::WalkDown;
          if tiles[self.position.1 as usize + 1][self.position.0 as usize].entity == EntityOnTile::None {
            tiles[self.position.1 as usize + 1][self.position.0 as usize].entity = EntityOnTile::Player;
            self.finish_animation = (PlayerAnimation::WalkDown, 12, ticks(ctx));
          }
        } else if keyboard::is_key_pressed(ctx, KeyCode::Left) {
          self.animation = PlayerAnimation::WalkLeft;
          if tiles[self.position.1 as usize][self.position.0 as usize - 1].entity == EntityOnTile::None {
            tiles[self.position.1 as usize][self.position.0 as usize - 1].entity = EntityOnTile::Player;
            self.finish_animation = (PlayerAnimation::WalkLeft, 12, ticks(ctx));
          }
        } else if keyboard::is_key_pressed(ctx, KeyCode::Right) {
          self.animation = PlayerAnimation::WalkRight;
          if tiles[self.position.1 as usize][self.position.0 as usize + 1].entity == EntityOnTile::None {
            tiles[self.position.1 as usize][self.position.0 as usize + 1].entity = EntityOnTile::Player;
            self.finish_animation = (PlayerAnimation::WalkRight, 12, ticks(ctx));
          }
        }
      }
      if self.finish_animation.1 > 0 {
        self.finish_animation.1 -= 1;
        match (ticks(ctx) - self.finish_animation.2) % 12 {
          2  => self.frame = 1.,
          5  => self.frame = 0.,
          8  => self.frame = 2.,
          11 => self.frame = 0.,
          _  => ()
        }
        match self.finish_animation.0 {
          PlayerAnimation::WalkUp    => self.position.1 -= 1. / 12.,
          PlayerAnimation::WalkDown  => self.position.1 += 1. / 12.,
          PlayerAnimation::WalkLeft  => self.position.0 -= 1. / 12.,
          PlayerAnimation::WalkRight => self.position.0 += 1. / 12.,
          _ => ()
        };
        if self.finish_animation.1 == 0 {
          self.position.0 = self.position.0.round();
          self.position.1 = self.position.1.round();
          self.frame = 0.;
          match self.animation {
            PlayerAnimation::WalkRight => {
              tiles[self.position.1 as usize][self.position.0 as usize - 1].entity = EntityOnTile::None;
              self.animation = PlayerAnimation::StandRight;
            },
            PlayerAnimation::WalkLeft => {
              tiles[self.position.1 as usize][self.position.0 as usize + 1].entity = EntityOnTile::None;
              self.animation = PlayerAnimation::StandLeft;
            },
            PlayerAnimation::WalkUp => {
              tiles[self.position.1 as usize + 1][self.position.0 as usize].entity = EntityOnTile::None;
              self.animation = PlayerAnimation::StandUp;
            },
            PlayerAnimation::WalkDown => {
              tiles[self.position.1 as usize - 1][self.position.0 as usize].entity = EntityOnTile::None;
              self.animation = PlayerAnimation::StandDown;
            },
            _ => ()
          };
          if thread_rng().gen::<f32>() < encounter_rate {
            let enemy_formations = enemy_formations(ctx);
            *battle = Battle::new(ctx, enemy_formations, party, menu);
            transition.set(TransitionStyle::WhiteInFast(GameMode::Battle))?;
          }
        }
      }
    }
    Ok(())
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    let (spritesheet_x, spritesheet_y, anim_loop_len) = match self.animation {
      PlayerAnimation::StandLeft  => (0., 1., 1.),
      PlayerAnimation::WalkLeft   => (0., 1., 3.),
      PlayerAnimation::StandRight => (0., 2., 1.),
      PlayerAnimation::WalkRight  => (0., 2., 3.),
      PlayerAnimation::StandUp    => (0., 3., 1.),
      PlayerAnimation::WalkUp     => (0., 3., 3.),
      PlayerAnimation::StandDown  => (0., 0., 1.),
      PlayerAnimation::WalkDown   => (0., 0., 3.)
    };
    let p = DrawParam::new()
      .src(Rect::new(0.333333333333333 * (spritesheet_x + (self.frame % anim_loop_len)), 0.2 * spritesheet_y, 0.333333333333333, 0.2));
    self.spritebatch.add(p);
    let param = DrawParam::new()
      .dest(Point2::new(WINDOW_SIZE.0 / 2., WINDOW_SIZE.1 / 2.));
    draw(ctx, &self.spritebatch, param)?;
    self.spritebatch.clear();
    Ok(())
  }
}