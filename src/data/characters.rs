use ggez::Context;
use crate::battle::action::{ActionParameters, DamageType};
use crate::party::character::Character;
use crate::menu::item::OnClickEvent;
use crate::data::menus;

pub fn none_character(ctx: &mut Context, id: u8) -> Character {
  let attack            = (" ".to_owned(), OnClickEvent::None);
  let primary_ability   = (" ".to_owned(), OnClickEvent::None);
  let secondary_ability = (" ".to_owned(), OnClickEvent::None);
  Character::new(ctx, id, "/empty.png".to_owned(), "".to_owned(), 0, 0, 0, 0, 0, 0, 0, 0, attack, primary_ability, secondary_ability)
}

pub fn darrel_deen(ctx: &mut Context, id: u8) -> Character {
  let attack = ("Attack".to_owned(), OnClickEvent::ToTargetSelection(
    menus::to_target_selection, ActionParameters::new(DamageType::Physical, 4, 0., false, 1., false, 1., false)
  ));
  let primary_ability = ("Steal".to_owned(), OnClickEvent::ToTargetSelection(
    menus::to_target_selection, ActionParameters::new(DamageType::None, 0, 0., false, 0., false, 0., false)
  ));
  let secondary_ability = ("Flee".to_owned(), OnClickEvent::None);
  Character::new(ctx, id, "/characters/Darrel_Deen.png".to_owned(), "Darrel".to_owned(), 3, 9935, 910, 6, 5, 3, 4, 5, attack, primary_ability, secondary_ability)
}

pub fn nurse_seraphine(ctx: &mut Context, id: u8) -> Character {
  let attack = ("Attack".to_owned(), OnClickEvent::ToTargetSelection(
    menus::to_target_selection, ActionParameters::new(DamageType::Physical, 4, 0., false, 0., false, 0., false)
  ));
  let primary_ability = ("Medicine".to_owned(), OnClickEvent::ToTargetSelection(
    menus::to_target_selection, ActionParameters::new(DamageType::Physical, 4, 0., false, 0., false, 0., false)
  ));
  let secondary_ability = ("asdf".to_owned(), OnClickEvent::None);
  Character::new(ctx, id, "/characters/Nurse_Seraphine.png".to_owned(), "Seraphine".to_owned(), 3, 20, 15, 5, 3, 5, 3, 4, attack, primary_ability, secondary_ability)
}