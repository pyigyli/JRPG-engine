use ggez::graphics::{spritebatch, Image, DrawParam, Rect, draw};
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use crate::globals::{WINDOW_SIZE, GRID_SIZE};
pub mod tile;
pub mod player;
use crate::GameMode;
use crate::transition::{Transition, TransitionStyle};
use crate::party::Party;
use crate::battle::Battle;
use crate::battle::enemy::Enemy;
use crate::menu::MenuScreen;

pub struct Tilemap {
  spritebatch: spritebatch::SpriteBatch,
  pub tiles:  Vec<Vec<tile::Tile>>,
  pub encounter_rate: f32,
  player: player::Player,
  enemy_formations: for<'r> fn(&'r mut Context) -> Vec<Vec<Enemy>>
}

impl Tilemap {
  pub fn new(
    ctx: &mut Context,
    spritefile: String,
    tiles: Vec<Vec<tile::Tile>>,
    encounter_rate: f32,
    enemy_formations: for<'r> fn(&'r mut Context) -> Vec<Vec<Enemy>>
  ) -> Tilemap {
    let image = Image::new(ctx, spritefile).unwrap();
    let batch = spritebatch::SpriteBatch::new(image);
    let mut player_pos = (1., 1.);
    for (j, row) in tiles.iter().enumerate() {
      for (i, tile) in row.iter().enumerate() {
        if tile.entity == tile::EntityOnTile::Player {
          player_pos = (i as f32, j as f32);
        }
      }
    }
    Tilemap {
      spritebatch: batch,
      tiles,
      encounter_rate,
      player: player::Player::new(ctx, "/characters/Darrel_Deen.png".to_owned(), player_pos),
      enemy_formations
    }
  }

  pub fn update(
    &mut self,
    ctx: &mut Context,
    mode: &mut GameMode,
    party: &mut Party,
    battle: &mut Battle,
    menu: &mut MenuScreen,
    transition: &mut Transition
  ) -> GameResult<()> {
    if *mode == GameMode::Map && transition.style == TransitionStyle::None {
      self.player.update(ctx, &mut self.tiles, mode, party, battle, transition, menu, self.encounter_rate, self.enemy_formations)?;
    }
    Ok(())
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    let mut j = 0.;
    for row in self.tiles.iter() {
      let mut i = 0.;
      for tile in row.iter() {
        let p = DrawParam::new()
          .src(Rect::new(tile.spritesheet_pos.0 * 0.25, tile.spritesheet_pos.1 * 0.333333333333333, 0.25, 0.333333333333333))
          .dest(Point2::new((i - self.player.position.0) * GRID_SIZE, (j - self.player.position.1) * GRID_SIZE));
        self.spritebatch.add(p);
        i += 1.;
      }
      j += 1.;
    }
    let param = DrawParam::new()
      .dest(Point2::new(WINDOW_SIZE.0 / 2., WINDOW_SIZE.1 / 2.));
    draw(ctx, &self.spritebatch, param)?;
    self.player.draw(ctx)?;
    self.spritebatch.clear();
    Ok(())
  }
}