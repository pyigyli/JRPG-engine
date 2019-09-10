use ggez::{Context, GameResult};
use crate::menu::item::{MenuItem, OnClickEvent};

pub struct CharacterInfo {
  name: MenuItem,
  max_hp: MenuItem,
  max_mp: MenuItem,
  hp: MenuItem,
  mp: MenuItem,
  atb: u8,
  pub status_effects: Vec<MenuItem>
}

impl CharacterInfo {
  pub fn new(ctx: &mut Context, id: u8, name: &String, hp: u16, mp: u16) -> CharacterInfo {
    let info_name = MenuItem::new(ctx, "/empty.png".to_owned(), name.to_owned()   , (330., 450. + id as f32 * 60.), OnClickEvent::None);
    let info_hp   = MenuItem::new(ctx, "/empty.png".to_owned(), format!("{}/", hp), (600., 450. + id as f32 * 60.), OnClickEvent::None);
    let info_mp   = MenuItem::new(ctx, "/empty.png".to_owned(), format!("{}/", mp), (800., 450. + id as f32 * 60.), OnClickEvent::None);
    let max_hp    = MenuItem::new(ctx, "/empty.png".to_owned(), format!("{}", hp) , (700., 450. + id as f32 * 60.), OnClickEvent::None);
    let max_mp    = MenuItem::new(ctx, "/empty.png".to_owned(), format!("{}", mp) , (900., 450. + id as f32 * 60.), OnClickEvent::None);
    CharacterInfo {
      name: info_name,
      max_hp,
      max_mp,
      hp: info_hp,
      mp: info_mp,
      atb: 0,
      status_effects: Vec::new()
    }
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.name.draw(ctx)?;
    self.max_hp.draw(ctx)?;
    self.max_mp.draw(ctx)?;
    self.hp.draw(ctx)?;
    self.mp.draw(ctx)?;
    for effect in &mut self.status_effects {
      effect.draw(ctx)?;
    }
    Ok(())
  }
}