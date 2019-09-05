use ggez::graphics::{spritebatch, Image, DrawParam, Rect, draw};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};

pub struct MenuContainer {
  spritebatch: spritebatch::SpriteBatch,
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32
}

impl MenuContainer {
  pub fn new(ctx: &mut Context, x: f32, y: f32, width: f32, height: f32) -> MenuContainer {
    let image = Image::new(ctx, "/menu.png").unwrap();
    let batch = spritebatch::SpriteBatch::new(image);
    MenuContainer {
      spritebatch: batch,
      x,
      y,
      width,
      height
    }
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    let top_left = DrawParam::new()
      .src(Rect::new(0., 0., 0.5, 0.5))
      .dest(Point2::new(self.x, self.y));
    self.spritebatch.add(top_left);
    let top_right = DrawParam::new()
      .src(Rect::new(0.5, 0., 0.5, 0.5))
      .dest(Point2::new(self.x + self.width - 50., self.y));
    self.spritebatch.add(top_right);
    let bottom_left = DrawParam::new()
      .src(Rect::new(0., 0.5, 0.5, 0.5))
      .dest(Point2::new(self.x, self.y + self.height - 50.));
    self.spritebatch.add(bottom_left);
    let bottom_right = DrawParam::new()
      .src(Rect::new(0.5, 0.5, 0.5, 0.5))
      .dest(Point2::new(self.x + self.width - 50., self.y + self.height - 50.));
    self.spritebatch.add(bottom_right);
    let top = DrawParam::new()
      .src(Rect::new(0.3, 0., 0.3, 0.5))
      .dest(Point2::new(self.x + 50., self.y))
      .scale(Vector2::new((self.width - 100.) / 30., 1.));
    self.spritebatch.add(top);
    let bottom = DrawParam::new()
      .src(Rect::new(0.3, 0.5, 0.3, 0.5))
      .dest(Point2::new(self.x + 50., self.y + self.height - 50.))
      .scale(Vector2::new((self.width - 100.) / 30., 1.));
    self.spritebatch.add(bottom);
    let left = DrawParam::new()
      .src(Rect::new(0., 0.3, 0.5, 0.3))
      .dest(Point2::new(self.x, self.y + 50.))
      .scale(Vector2::new(1., (self.height - 100.) / 30.));
    self.spritebatch.add(left);
    let right = DrawParam::new()
      .src(Rect::new(0.5, 0.3, 0.5, 0.3))
      .dest(Point2::new(self.x + self.width - 50., self.y + 50.))
      .scale(Vector2::new(1., (self.height - 100.) / 30.));
    self.spritebatch.add(right);
    let center = DrawParam::new()
      .src(Rect::new(0.3, 0.3, 0.3, 0.3))
      .dest(Point2::new(self.x + 50., self.y + 50.))
      .scale(Vector2::new((self.width - 100.) / 30., (self.height - 100.) / 30.));
    self.spritebatch.add(center);
    draw(ctx, &self.spritebatch, DrawParam::new())?;
    self.spritebatch.clear();
    Ok(())
  }
}