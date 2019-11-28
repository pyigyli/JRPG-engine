use ggez::graphics::{spritebatch, Color, Image, DrawParam, draw};
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use crate::data::font::number_param;

pub struct PrintDamage {
  spritebatch: spritebatch::SpriteBatch,
  value: u16,
  screen_pos: (f32, f32),
  color: Color,
  pub show_time: f32
}

impl PrintDamage {
  pub fn new(ctx: &mut Context, value: u16, screen_pos: (f32, f32), color: Color) -> PrintDamage {
    let image = Image::new(ctx, "/numbers.png").unwrap();
    let batch = spritebatch::SpriteBatch::new(image);
    PrintDamage {
      spritebatch: batch,
      value,
      screen_pos,
      color,
      show_time: 120.
    }
  }

  pub fn update(&mut self) -> GameResult<()> {
    self.show_time -= 1.;
    self.screen_pos.1 -= 0.000035 * self.show_time * self.show_time;
    if self.show_time < 60. {
      self.color.a = (self.color.a - 0.001) * self.color.a;
    }
    Ok(())
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    number_param(&mut self.spritebatch, format!("{}", self.value), self.color);
    let param = DrawParam::new()
      .dest(Point2::new(self.screen_pos.0, self.screen_pos.1));
    draw(ctx, &self.spritebatch, param)?;
    self.spritebatch.clear();
    Ok(())
  }
}