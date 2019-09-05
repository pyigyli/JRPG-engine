use ggez::Context;
use crate::tilemap::Tilemap;
use crate::tilemap::tile::Tile;
use crate::tilemap::tile::EntityOnTile::*;
use crate::data::enemy_formations;

pub fn test_room(ctx: &mut Context) -> Tilemap {
  Tilemap::new(
    ctx,
    "/test-tileset.png".to_owned(),
    tiles![
      [(0., 0., Solid), (1., 0., Solid), (1., 0., Solid), (1., 0., Solid ), (1., 0., Solid), (1., 0., Solid), (1., 0., Solid), (2., 0., Solid)]
      [(0., 1., Solid), (1., 1., None ), (1., 1., None ), (1., 1., None  ), (1., 1., None ), (1., 1., None ), (1., 1., None ), (2., 1., Solid)]
      [(0., 1., Solid), (1., 1., None ), (1., 1., None ), (1., 1., Player), (1., 1., None ), (1., 1., None ), (1., 1., None ), (2., 1., Solid)]
      [(0., 1., Solid), (1., 1., None ), (1., 1., None ), (1., 1., None  ), (1., 1., None ), (1., 1., None ), (1., 1., None ), (2., 1., Solid)]
      [(0., 2., Solid), (1., 2., Solid), (1., 2., Solid), (1., 2., Solid ), (1., 2., Solid), (1., 2., Solid), (1., 2., Solid), (2., 2., Solid)]
    ],
    0.90,
    enemy_formations::test_room
  )
}