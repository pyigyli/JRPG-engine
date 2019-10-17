use ggez::graphics::{spritebatch, Image, DrawParam, draw};
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use crate::GameMode;
use crate::data::font::font_param;
use crate::party::Party;
use crate::battle::action::ActionParameters;
use crate::battle::enemy::Enemy;
use crate::menu::MenuScreen;

pub enum OnClickEvent {
  None,
  ToMenuScreen(
    for<'r, 's, 't0, 't1> fn(&'r mut Context, &'s mut GameMode, &'t0 mut Party, &'t1 Vec<Vec<Enemy>>, (usize, usize)) -> MenuScreen,
    (usize, usize)
  ),
  ToTargetSelection(for<'r, 's, 't0, 't1> fn(&'r mut Context, &'s mut Party, &'t0 Vec<Vec<Enemy>>, &'t1 ActionParameters) -> MenuScreen, ActionParameters),
  ActOnTarget((usize, usize), ActionParameters),
  MutateMenu(for<'r, 's> fn(&'r mut MenuScreen, &'s mut Party) -> GameResult<()>),
  Transition(GameMode),
  MenuTransition(for<'r, 's, 't0, 't1> fn(&'r mut Context, &'s mut GameMode, &'t0 mut Party, &'t1 Vec<Vec<Enemy>>) -> MenuScreen),
  UseItemInMenu(
    for<'r, 's, 't0, 't1, 't2> fn(
      &'r mut Context,
      &'s mut MenuScreen,
      &'t0 mut GameMode,
      &'t1 mut Party,
      &'t2 Vec<Vec<Enemy>>,
      (usize, usize),
      Vec<u8>
    ) -> GameResult<()>,
    Vec<u8>,
    (usize, usize)
  )
}

impl Clone for OnClickEvent {
  fn clone(&self) -> Self {
    match self {
      OnClickEvent::None                                                   => OnClickEvent::None,
      OnClickEvent::ToMenuScreen(new_menu, cursor_start)                   => OnClickEvent::ToMenuScreen(*new_menu, *cursor_start),
      OnClickEvent::ToTargetSelection(target_selection, action_parameters) => OnClickEvent::ToTargetSelection(*target_selection, action_parameters.clone()),
      OnClickEvent::ActOnTarget(position, action_parameters)               => OnClickEvent::ActOnTarget(*position, action_parameters.clone()),
      OnClickEvent::MutateMenu(mutation)                                   => OnClickEvent::MutateMenu(*mutation),
      OnClickEvent::Transition(new_mode)                                   => OnClickEvent::Transition(new_mode.clone()),
      OnClickEvent::MenuTransition(new_menu)                               => OnClickEvent::MenuTransition(*new_menu),
      OnClickEvent::UseItemInMenu(new_menu, targets, item_cursor_pos)      => OnClickEvent::UseItemInMenu(*new_menu, targets.to_vec(), *item_cursor_pos)
    }
  }
}

pub struct MenuItem {
  spritebatch: spritebatch::SpriteBatch,
  pub text: String,
  pub screen_pos: (f32, f32),
  pub sprite_height: f32,
  pub on_click: OnClickEvent
}

impl MenuItem {
  pub fn new(ctx: &mut Context, spritefile: String, text: String, screen_pos: (f32, f32), sprite_height: f32, on_click: OnClickEvent) -> MenuItem {
    if text.len() > 0 {
      let image = Image::new(ctx, "/font.png").unwrap();
      let batch = spritebatch::SpriteBatch::new(image);
      MenuItem {
        spritebatch: batch,
        text,
        screen_pos,
        sprite_height,
        on_click
      }
    } else {
      let image = Image::new(ctx, spritefile).unwrap();
      let batch = spritebatch::SpriteBatch::new(image);
      MenuItem {
        spritebatch: batch,
        text,
        screen_pos,
        sprite_height,
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