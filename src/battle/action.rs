use ggez::{Context, GameResult};
use crate::battle::state::BattleState;
use crate::menu::notification::Notification;
use crate::party::InventoryElement;
use crate::party::item::InventoryItem;

pub enum DamageType {
  None(for<'r, 's, 't1, 't2, 't3> fn(
    &'r mut Context,
    &'s mut Vec<InventoryElement>,
    &'t1 ActionParameters,
    &'t2 mut BattleState,
    &'t3 mut Option<Notification>
  ) -> GameResult<()>),
  Item(InventoryItem),
  Physical,
  Magical,
  Pure,
  Healing
}

impl Clone for DamageType {
  fn clone(&self) -> Self {
    match self {
      DamageType::None(action) => DamageType::None(*action),
      DamageType::Item(item)   => DamageType::Item(item.clone()),
      DamageType::Physical     => DamageType::Physical,
      DamageType::Magical      => DamageType::Magical,
      DamageType::Pure         => DamageType::Pure,
      DamageType::Healing      => DamageType::Healing
    }
  }
}

#[derive(Clone)]
pub struct ActionParameters {
  pub damage_type: DamageType,
  pub power: u16,
  pub dead_change: f32,
  pub revive: bool,
  pub poison_change: f32,
  pub cure_poison: bool,
  pub sleep_change: f32,
  pub cure_sleep: bool
}

impl ActionParameters {
  pub fn new(
    damage_type: DamageType,
    power: u16,
    dead_change: f32,
    revive: bool,
    poison_change: f32,
    cure_poison: bool,
    sleep_change: f32,
    cure_sleep: bool
  ) -> ActionParameters {
    ActionParameters {
      damage_type,
      power,
      dead_change,
      revive,
      poison_change,
      cure_poison,
      sleep_change,
      cure_sleep
    }
  }
}