pub enum EntityOnTile {
  None, Solid, Player
}

impl PartialEq for EntityOnTile {
  fn eq(&self, other: &Self) -> bool {
    match self {
      EntityOnTile::None   => {match other {EntityOnTile::None   => true, _ => false}},
      EntityOnTile::Player => {match other {EntityOnTile::Player => true, _ => false}},
      EntityOnTile::Solid  => {match other {EntityOnTile::Solid  => true, _ => false}}
    }
  }
}

pub struct Tile {
  pub spritesheet_pos: (f32, f32),
  pub entity: EntityOnTile
}

impl Tile {
  pub fn new(spritesheet_pos: (f32, f32), entity: EntityOnTile) -> Tile {
    Tile {
      spritesheet_pos,
      entity
    }
  }
}