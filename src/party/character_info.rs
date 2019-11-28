use ggez::{Context, GameResult};
use crate::menu::item::{MenuItem, OnClickEvent};

pub struct CharacterInfo {
  pub name: MenuItem,
  pub max_hp: MenuItem,
  pub max_mp: MenuItem,
  pub hp: MenuItem,
  pub mp: MenuItem,
  pub atb: u8,
  pub status_effects: Vec<(String, MenuItem)>
}

impl CharacterInfo {
  pub fn new(ctx: &mut Context, id: u8, name: &String, hp: u16, mp: u16) -> CharacterInfo {
    let info_name = MenuItem::new(ctx, "/empty.png".to_owned(), name.to_owned(), (330., 370. + id as f32 * 62.), 24., OnClickEvent::None);
    let info_hp = MenuItem::new(
      ctx, "/empty.png".to_owned(), format!("{}/", hp), (610. + (5 - format!("{}/", hp).len()) as f32 * 24., 370. + id as f32 * 62.), 24., OnClickEvent::None
    );
    let info_mp = MenuItem::new(
      ctx, "/empty.png".to_owned(), format!("{}/", mp), (874. + (4 - format!("{}/", mp).len()) as f32 * 24., 370. + id as f32 * 62.), 24., OnClickEvent::None
    );
    let max_hp  = MenuItem::new(
      ctx, "/empty.png".to_owned(), format!("{}" , hp), (730. + (4 - format!("{}" , hp).len()) as f32 * 24., 370. + id as f32 * 62.), 24., OnClickEvent::None
    );
    let max_mp  = MenuItem::new(
      ctx, "/empty.png".to_owned(), format!("{}" , mp), (970. + (3 - format!("{}" , mp).len()) as f32 * 24., 370. + id as f32 * 62.), 24., OnClickEvent::None
    );
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

  pub fn set_effect(&mut self, ctx: &mut Context, id: u8, effect_name: String) -> GameResult<()> {
    let path = format!("/status_effects/{}.png", effect_name);
    self.status_effects.push((effect_name, MenuItem::new(
      ctx,
      path,
      "".to_owned(),
      (330. + self.status_effects.len() as f32 * 25., 397. + id as f32 * 62.),
      24.,
      OnClickEvent::None
    )));
    Ok(())
  }

  pub fn remove_effect(&mut self, effect_name: String) -> GameResult<()> {
    let mut index = 0;
    for (i, effect) in self.status_effects.iter_mut().enumerate() {
      if effect.0 == effect_name {
        index = i;
        break;
      }
    }
    self.status_effects.remove(index);
    for (i, effect) in self.status_effects.iter_mut().enumerate() {
      if i >= index {
        effect.1.screen_pos.0 -= 25.;
      }
    }
    Ok(())
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    self.name.draw(ctx)?;
    self.max_hp.draw(ctx)?;
    self.max_mp.draw(ctx)?;
    self.hp.draw(ctx)?;
    self.mp.draw(ctx)?;
    for effect in &mut self.status_effects {
      effect.1.draw(ctx)?;
    }
    Ok(())
  }
}