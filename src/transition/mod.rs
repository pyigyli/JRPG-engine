use ggez::graphics::{DrawParam, DrawMode, Rect, Color, Mesh, draw};
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use crate::globals::WINDOW_SIZE;
use crate::GameMode;

pub enum TransitionStyle {
  None, WhiteInFast(GameMode), WhiteOutFast, BlackInFast(GameMode), BlackOutFast
}

impl PartialEq for TransitionStyle {
  fn eq(&self, other: &Self) -> bool {
    match self {
      TransitionStyle::None           => {match other {TransitionStyle::None           => true, _ => false}},
      TransitionStyle::WhiteInFast(_) => {match other {TransitionStyle::WhiteInFast(_) => true, _ => false}},
      TransitionStyle::WhiteOutFast   => {match other {TransitionStyle::WhiteOutFast   => true, _ => false}},
      TransitionStyle::BlackInFast(_) => {match other {TransitionStyle::BlackInFast(_) => true, _ => false}},
      TransitionStyle::BlackOutFast   => {match other {TransitionStyle::BlackOutFast   => true, _ => false}}
    }
  }
}

pub struct Transition {
  pub style: TransitionStyle,
  opacity: f32
}

impl Transition {
  pub fn new() -> Transition {
    Transition {
      style: TransitionStyle::None,
      opacity: 0.
    }
  }

  pub fn set(&mut self, transition: TransitionStyle) -> GameResult<()> {
    self.style = transition;
    self.opacity = 0.;
    Ok(())
  }

  pub fn update(&mut self, mode: &mut GameMode) -> GameResult<()> {
    let mut done = false;
    match &self.style {
      TransitionStyle::None => (),
      TransitionStyle::WhiteInFast(new_mode) => {
        self.opacity += 0.1;
        self.opacity *= 0.95;
        if self.opacity > 1. {
          *mode = new_mode.clone();
          done = true;
        }
      },
      TransitionStyle::WhiteOutFast => {
        self.opacity -= 0.05;
        self.opacity *= 0.9;
        if self.opacity < 0. {
          done = true;
        }
      },
      TransitionStyle::BlackInFast(new_mode) => {
        self.opacity += 0.1;
        self.opacity *= 0.95;
        if self.opacity > 1. {
          *mode = new_mode.clone();
          done = true;
        }
      },
      TransitionStyle::BlackOutFast => {
        self.opacity -= 0.05;
        self.opacity *= 0.9;
        if self.opacity < 0. {
          done = true;
        }
      }
    }
    if done {
      self.style = match &self.style {
        TransitionStyle::WhiteInFast(_) => TransitionStyle::WhiteOutFast,
        TransitionStyle::BlackInFast(_) => TransitionStyle::BlackOutFast,
        _ => TransitionStyle::None
      }
    }
    Ok(())
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    match &self.style {
      TransitionStyle::None => (),
      TransitionStyle::WhiteInFast(_) | TransitionStyle::WhiteOutFast => {
        let rectangle = Rect::new(-WINDOW_SIZE.0, -WINDOW_SIZE.1, WINDOW_SIZE.0 * 2., WINDOW_SIZE.1 * 2.);
        let color = Color::new(1., 1., 1., self.opacity);
        let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rectangle, color).unwrap();
        draw(ctx, &mesh, DrawParam::new().dest(Point2::new(0., 0.)))?;
      },
      TransitionStyle::BlackInFast(_) | TransitionStyle::BlackOutFast => {
        let rectangle = Rect::new(0., 0., WINDOW_SIZE.0, WINDOW_SIZE.1);
        let color = Color::new(0., 0., 0., self.opacity);
        let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rectangle, color).unwrap();
        draw(ctx, &mesh, DrawParam::new())?;
      }
    }
    Ok(())
  }
}