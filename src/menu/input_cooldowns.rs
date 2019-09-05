pub struct InputCooldowns {
  pub a: bool,
  pub s: bool,
  pub d: bool,
  pub f: bool,
  pub up: bool,
  pub down: bool,
  pub left: bool,
  pub right: bool
}

impl InputCooldowns {
  pub fn new() -> InputCooldowns {
    InputCooldowns {
      a: true,
      s: true,
      d: true,
      f: true,
      up: true,
      down: true,
      left: true,
      right: true
    }
  }
}