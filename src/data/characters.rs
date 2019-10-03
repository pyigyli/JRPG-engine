use ggez::{Context, GameResult};
use crate::battle::action::{ActionParameters, DamageType};
use crate::battle::state::BattleState;
use crate::party::character::Character;
use crate::menu::item::OnClickEvent;
use crate::data::menus;

pub fn none_character(ctx: &mut Context, id: u8) -> Character {
  let attack            = (" ".to_owned(), OnClickEvent::None);
  let primary_ability   = (" ".to_owned(), OnClickEvent::None);
  let secondary_ability = (" ".to_owned(), OnClickEvent::None);
  Character::new(ctx, id, "/empty.png".to_owned(), "/empty.png".to_owned(), "".to_owned(), 0, 0, 0, 0, 0, 0, 0, 0, attack, primary_ability, secondary_ability)
}

pub fn darrel_deen(ctx: &mut Context, id: u8) -> Character {
  let attack = ("Attack".to_owned(), OnClickEvent::ToTargetSelection(
    menus::to_target_selection, ActionParameters::new(DamageType::Physical, 4, 0., false, 1., false, 1., false)
  ));
  fn steal_action(action_parameters: &ActionParameters, target_state: &mut BattleState) -> GameResult<()> {
    Ok(())
  }
  let primary_ability = ("Steal".to_owned(), OnClickEvent::ToTargetSelection(
    menus::to_target_selection, ActionParameters::new(DamageType::None(steal_action), 0, 0., false, 0., false, 0., false)
  ));
  let secondary_ability = ("Flee".to_owned(), OnClickEvent::None);
  Character::new(
    ctx,
    id,
    "/characters/Darrel_Deen.png".to_owned(),
    "/characters/Darrel_Deen_avatar.png".to_owned(),
    "Darrel".to_owned(),
    3, // LVL
    9935, // HP
    910, // MP
    6, // ATK
    5, // DEF
    3, // MAG
    4, // RES
    5, // AGI
    attack,
    primary_ability,
    secondary_ability
  )
}

pub fn nurse_seraphine(ctx: &mut Context, id: u8) -> Character {
  let attack = ("Attack".to_owned(), OnClickEvent::ToTargetSelection(
    menus::to_target_selection, ActionParameters::new(DamageType::Physical, 4, 0., false, 0., false, 0., false)
  ));
  let primary_ability = ("Medicine".to_owned(), OnClickEvent::ToTargetSelection(
    menus::to_target_selection, ActionParameters::new(DamageType::Physical, 4, 0., false, 0., false, 0., false)
  ));
  let secondary_ability = ("asdf".to_owned(), OnClickEvent::None);
  Character::new(
    ctx,
    id,
    "/characters/Nurse_Seraphine.png".to_owned(),
    "/characters/Nurse_Seraphine_avatar.png".to_owned(),
    "Seraphine".to_owned(),
    3, // LVL
    20, // HP
    15, // MP
    5, // ATK
    3, // DEF
    5, // MAG
    3, // RES
    4, // AGI
    attack,
    primary_ability,
    secondary_ability
  )
}