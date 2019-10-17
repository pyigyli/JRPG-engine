use ggez::{Context, GameResult};
use crate::menu::container::MenuContainer;
use crate::menu::item::{MenuItem, OnClickEvent};

pub struct Notification {
  container: MenuContainer,
  text: MenuItem,
  pub show_time: u16 // as frames
}

impl Notification {
  pub fn new(ctx: &mut Context, text: String) -> Notification {
    let text_pos = (530. - text.len() as f32 * 12., 40.);
    Notification {
      container: MenuContainer::new(ctx, 10., 10., 1060., 84.),
      text: MenuItem::new(ctx, "".to_owned(), text, text_pos, 24., OnClickEvent::None),
      show_time: 90
    }
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.container.draw(ctx)?;
    self.text.draw(ctx)?;
    Ok(())
  }
}