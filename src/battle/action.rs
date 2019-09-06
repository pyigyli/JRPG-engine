pub enum DamageType {
  None, Physical, Magical, Pure, Healing
}

impl Clone for DamageType {
  fn clone(&self) -> Self {
    match self {
      DamageType::None     => DamageType::None,
      DamageType::Physical => DamageType::Physical,
      DamageType::Magical  => DamageType::Magical,
      DamageType::Pure     => DamageType::Pure,
      DamageType::Healing  => DamageType::Healing
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