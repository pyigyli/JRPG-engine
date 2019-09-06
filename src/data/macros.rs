macro_rules! text {($ctx:expr, $text:expr, $position_x:expr, $position_y:expr, $onclick:expr) => {
  MenuItem::new($ctx, String::new(), $text.to_owned(),    ($position_x, $position_y), $onclick)
};}

macro_rules! tiles {($([$(($spritesheet_x:expr, $spritesheet_y:expr, $entity:expr)$(,)?)*])*) => {
  vec![$(vec![$(Tile::new(($spritesheet_x, $spritesheet_y), $entity)),*]),*]
}}

macro_rules! battle_target_positions {($ctx:expr, $party:expr, $enemies:expr, $action_parameters:expr) => {{
  fn push_character(
    ctx: &mut Context,
    party_column: &mut Vec<MenuItem>,
    character: &mut Character,
    position: (usize, usize),
    action_parameters: &ActionParameters
  ) -> () {
    if character.name.len() > 0 {
      party_column.push(MenuItem::new(
          ctx,
          String::new(),
          " ".to_owned(),
          (220. + character.x_offset, 105. + character.id as f32 * 65.),
          OnClickEvent::ActOnTarget(position, action_parameters.clone())
      ));
    }
  }
  let mut target_positions = vec![];
  let mut party_column = vec![];
  push_character($ctx, &mut party_column, &mut $party.first , (0, 0), $action_parameters);
  push_character($ctx, &mut party_column, &mut $party.second, (0, 1), $action_parameters);
  push_character($ctx, &mut party_column, &mut $party.third , (0, 2), $action_parameters);
  push_character($ctx, &mut party_column, &mut $party.fourth, (0, 3), $action_parameters);
  target_positions.push(party_column);
  for column in $enemies {
    let mut column_vec = vec![];
    for enemy in column {
      column_vec.push(MenuItem::new(
        $ctx,
        String::new(),
        " ".to_owned(),
        (720. + enemy.x_offset + enemy.screen_pos.0 * 70., 220. + enemy.screen_pos.1 * 65.),
        OnClickEvent::ActOnTarget((enemy.selection_pos.0 + 1, enemy.selection_pos.1), $action_parameters.clone())
      ));
    }
    target_positions.push(column_vec);
  }
  target_positions
}};}