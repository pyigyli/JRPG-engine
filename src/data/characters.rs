use ggez::Context;
use crate::party::character::Character;

pub fn none_character(ctx: &mut Context, id: u8) -> Character {
  Character::new(ctx, id, "/empty.png".to_owned(), "".to_owned(), 0, 0, 0, 0, 0, 0, 0, 0)
}

pub fn darrel_deen(ctx: &mut Context, id: u8) -> Character {
  Character::new(ctx, id, "/characters/Darrel_Deen.png".to_owned(), "Darrel".to_owned(), 3, 35, 10, 6, 5, 3, 4, 5)
}

pub fn nurse_seraphine(ctx: &mut Context, id: u8) -> Character {
  Character::new(ctx, id, "/characters/Nurse_Seraphine.png".to_owned(), "Seraphine".to_owned(), 3, 20, 15, 5, 3, 5, 3, 4)
}