use ggez::graphics::{spritebatch, Image, DrawParam, draw};
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use crate::GameMode;
use crate::data::font::font_param;
use crate::party::Party;
use crate::battle::enemy::Enemy;
use crate::battle::action::ActionParameters;
use crate::menu::MenuScreen;

pub enum OnClickEvent {
  None,
  ToMenuScreen(for<'r, 's, 't0, 't1> fn(&'r mut Context, &'s mut GameMode, &'t0 mut Party, &'t1 Vec<Vec<Enemy>>) -> MenuScreen),
  ToTargetSelection(for<'r, 's, 't0, 't1> fn(&'r mut Context, &'s mut Party, &'t0 Vec<Vec<Enemy>>, &'t1 ActionParameters) -> MenuScreen, ActionParameters),
  ActOnTarget((usize, usize), ActionParameters),
  MutateMenu(for<'r> fn(&'r mut MenuScreen) -> GameResult<()>),
  Transition(GameMode)
}

pub struct MenuItem {
  spritebatch: spritebatch::SpriteBatch,
  pub text: String,
  pub screen_pos: (f32, f32),
  pub on_click: OnClickEvent
}

impl MenuItem {
  pub fn new(ctx: &mut Context, spritefile: String, text: String, screen_pos: (f32, f32), on_click: OnClickEvent) -> MenuItem {
    if text.len() > 0 {
      let image = Image::new(ctx, "/font.png").unwrap();
      let batch = spritebatch::SpriteBatch::new(image);
      MenuItem {
        spritebatch: batch,
        text,
        screen_pos,
        on_click
      }
    } else {
      let image = Image::new(ctx, spritefile).unwrap();
      let batch = spritebatch::SpriteBatch::new(image);
      MenuItem {
        spritebatch: batch,
        text,
        screen_pos,
        on_click
      }
    }
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    if self.text.len() > 0 {
      font_param(&mut self.spritebatch, &self.text);
    } else {
      self.spritebatch.add(DrawParam::new());
    }
    let param = DrawParam::new()
      .dest(Point2::new(self.screen_pos.0, self.screen_pos.1));
    draw(ctx, &self.spritebatch, param)?;
    self.spritebatch.clear();
    Ok(())
  }
}