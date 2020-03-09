use ggez::{Context, GameResult};
use ggez::timer::ticks;
use rand::{Rng, thread_rng};
use crate::battle::Battle;
use crate::battle::action::{ActionParameters, DamageType};
use crate::battle::state::BattleState;
use crate::party::{Party, InventoryElement};
use crate::party::character::{Character, Animation, Sprite};
use crate::menu::item::OnClickEvent;
use crate::menu::notification::Notification;
use crate::data::menus;

pub fn none_character(ctx: &mut Context, id: u8) -> Character {
  let attack            = (" ".to_owned(), OnClickEvent::None);
  let primary_ability   = (" ".to_owned(), OnClickEvent::None);
  let secondary_ability = (" ".to_owned(), OnClickEvent::None);
  Character::new(ctx, id, "/empty.png".to_owned(), "/empty.png".to_owned(), "".to_owned(), 0, 0, 0, 0, 0, 0, 0, 0, false, attack, primary_ability, secondary_ability)
}

pub fn darrel_deen(ctx: &mut Context, id: u8) -> Character {
  let attack = ("Attack".to_owned(), OnClickEvent::ToTargetSelection(
    menus::to_target_selection, ActionParameters::new(DamageType::Physical, 4, 0., false, 1., false, 1., false), (0, 0)
  ));
  fn steal_action(
    ctx: &mut Context,
    inventory: &mut Vec<InventoryElement>,
    _action_parameters: &ActionParameters,
    target_state: &mut BattleState,
    notification: &mut Option<Notification>
  ) -> GameResult<()> {
    if target_state.common_steal.is_none() && target_state.rare_steal.is_none() {
      *notification = Some(Notification::new(ctx, "Nothing to steal".to_owned()));
    } else {
      let rng = thread_rng().gen::<f32>();
      if rng < 0.5 {
        if let Some(common_steal) = &target_state.common_steal {
          if let Some(inventory_stack) = inventory.into_iter().find(|inventory_element| match inventory_element {
              InventoryElement::Item(item, _) => item.get_name() == common_steal.get_name()
            }
          ) {
            match inventory_stack {
              InventoryElement::Item(_, amount) => if *amount < 99 {*amount += 1}
            }
            *notification = Some(Notification::new(ctx, format!("Stole {}", common_steal.get_name())));
          }
        }
      } else if rng < 0.6 {
        if let Some(rare_steal) = &target_state.rare_steal {
          if let Some(inventory_stack) = inventory.into_iter().find(|inventory_element| match inventory_element {
              InventoryElement::Item(item, _) => item.get_name() == rare_steal.get_name()
            }
          ) {
            match inventory_stack {
              InventoryElement::Item(_, amount) => *amount += 1
            }
            *notification = Some(Notification::new(ctx, format!("Stole {}", rare_steal.get_name())));
          }
        }
      } else {
        *notification = Some(Notification::new(ctx, "Could not steal".to_owned()));
      }
    }
    Ok(())
  }
  fn flee_action(ctx: &mut Context, party: &mut Party, battle: &mut Battle) -> GameResult<()> {
    let mut escapeable = false;
    for enemy_column in &battle.enemies {
      for enemy in enemy_column {
        if enemy.escapeable {
          escapeable = true;
        }
      }
    }
    let rng = thread_rng().gen::<f32>();
    if escapeable && rng < 0.8 {
      if party.first.state.hp > 0 {
        party.first.sprite = Sprite::WalkLeft;
        party.first.animation = (Animation::Flee, 80, ticks(ctx))
      }
      if party.second.state.hp > 0 {
        party.second.sprite = Sprite::WalkLeft;
        party.second.animation = (Animation::Flee, 80, ticks(ctx))
      }
      if party.third.state.hp > 0 {
        party.third.sprite = Sprite::WalkLeft;
        party.third.animation = (Animation::Flee, 80, ticks(ctx))
      }
      if party.fourth.state.hp > 0 {
        party.fourth.sprite = Sprite::WalkLeft;
        party.fourth.animation = (Animation::Flee, 80, ticks(ctx))
      }
      battle.notification = Some(Notification::new(ctx, "Party escaped the battle".to_owned()));
    } else {
      battle.notification = Some(Notification::new(ctx, "Could not manage to flee".to_owned()));
    }
    Ok(())
  }
  let primary_ability = ("Steal".to_owned(), OnClickEvent::ToTargetSelection(
    menus::to_target_selection, ActionParameters::new(DamageType::None(steal_action), 0, 0., false, 0., false, 0., false), (0, 1)
  ));
  let secondary_ability = ("Flee".to_owned(), OnClickEvent::BattleAction(flee_action));
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
    false,
    attack,
    primary_ability,
    secondary_ability
  )
}

pub fn nurse_seraphine(ctx: &mut Context, id: u8) -> Character {
  let attack = ("Attack".to_owned(), OnClickEvent::ToTargetSelection(
    menus::to_target_selection, ActionParameters::new(DamageType::Physical, 4, 0., false, 0., false, 0., false), (0, 0)
  ));
  let primary_ability = ("Medicine".to_owned(), OnClickEvent::ToTargetSelection(
    menus::to_target_selection, ActionParameters::new(DamageType::Healing, 4, 0., false, 0., false, 0., false), (0, 1)
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
    true,
    attack,
    primary_ability,
    secondary_ability
  )
}