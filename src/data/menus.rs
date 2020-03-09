use ggez::{Context, GameResult};
use crate::GameMode;
use crate::menu::{MenuScreen, MenuMovement, MenuMutation};
use crate::menu::item::{MenuItem, OnClickEvent};
use crate::menu::container::MenuContainer;
use crate::party::{Party, InventoryElement};
use crate::party::character::Character;
use crate::battle::enemy::Enemy;
use crate::battle::action::ActionParameters;

pub fn none_menu(ctx: &mut Context) -> MenuScreen {
  MenuScreen::new(ctx, false, Vec::new(), vec![Vec::new()], Vec::new(), (0, 0), MenuMovement::Grid, OnClickEvent::None)
}

pub fn main_menu(ctx: &mut Context, _mode: &mut GameMode, party: &mut Party, _enemies: &Vec<Vec<Enemy>>, cursor_start: (usize, usize)) -> MenuScreen {
  let submenu_selection = MenuContainer::new(ctx, 10. , 10., 250., 300.);
  let character_info    = MenuContainer::new(ctx, 275., 10., 795., 700.);
  fn to_item_menu(ctx: &mut Context, mode: &mut GameMode, party: &mut Party, enemies: &Vec<Vec<Enemy>>, cursor_start: (usize, usize)) -> MenuScreen {
    item_menu(ctx, mode, party, enemies, cursor_start)
  }
  fn to_row_menu(ctx: &mut Context, mode: &mut GameMode, party: &mut Party, enemies: &Vec<Vec<Enemy>>, cursor_start: (usize, usize)) -> MenuScreen {
    row_change_menu(ctx, mode, party, enemies, cursor_start)
  }
  let item    = text!(ctx, "Item"   , 55., 60. , OnClickEvent::MenuTransition(to_item_menu, (0, 0)));
  let ability = text!(ctx, "Ability", 55., 100., OnClickEvent::None);
  let equip   = text!(ctx, "Equip"  , 55., 140., OnClickEvent::None);
  let row     = text!(ctx, "Row"    , 55., 180., OnClickEvent::ToMenuScreen(to_row_menu, (0, 0)));
  let config  = text!(ctx, "Config" , 55., 220., OnClickEvent::None);
  let mut unselectable_items = Vec::new();
  fn push_party_memeber_to_unselectables(ctx: &mut Context, vector: &mut Vec<MenuItem>, character: &Character) {
    if character.name.len() > 0 {
      let offset_by_id = (character.state.id - 1) as f32 * 162.;
      vector.push(MenuItem::new(ctx, character.avatar_spritefile.to_owned(), "".to_owned(), (360. + character.x_offset, 52. + offset_by_id), 128., OnClickEvent::None));
      vector.push(MenuItem::new(ctx, "".to_owned(), format!("Lvl {}", character.state.level), (500., 60. + offset_by_id), 24., OnClickEvent::None));
      vector.push(MenuItem::new(ctx, "".to_owned(), character.name.to_owned(), (680., 60. + offset_by_id), 24., OnClickEvent::None));
      vector.push(MenuItem::new(
        ctx,
        "".to_owned(),
        format!("{}/", character.state.hp),
        (500. + (5 - format!("{}/", character.state.hp).len()) as f32 * 24., 100. + offset_by_id),
        24.,
        OnClickEvent::None
      ));
      vector.push(MenuItem::new(
        ctx,
        "".to_owned(),
        format!("{}/", character.state.mp),
        (822. + (4 - format!("{}/", character.state.mp).len()) as f32 * 24., 100. + offset_by_id),
        24.,
        OnClickEvent::None
      ));
      vector.push(MenuItem::new(
        ctx,
        "".to_owned(),
        format!("{}Hp", character.state.hp),
        (620. + (4 - format!("{}", character.state.hp).len()) as f32 * 24., 100. + offset_by_id),
        24.,
        OnClickEvent::None
      ));
      vector.push(MenuItem::new(
        ctx,
        "".to_owned(),
        format!("{}Mp",
        character.state.mp),
        (918. + (3 - format!("{}", character.state.mp).len()) as f32 * 24., 100. + offset_by_id),
        24.,
        OnClickEvent::None
      ));
    }
  }
  push_party_memeber_to_unselectables(ctx, &mut unselectable_items, &party.first);
  push_party_memeber_to_unselectables(ctx, &mut unselectable_items, &party.second);
  push_party_memeber_to_unselectables(ctx, &mut unselectable_items, &party.third);
  push_party_memeber_to_unselectables(ctx, &mut unselectable_items, &party.fourth);
  MenuScreen::new(
    ctx,
    true,
    vec![submenu_selection, character_info],
    vec![
      vec![item, ability, equip, row, config]
    ],
    unselectable_items,
    cursor_start,
    MenuMovement::RowOfColumns,
    OnClickEvent::Transition(GameMode::Map)
  )
}

pub fn item_menu(ctx: &mut Context, _mode: &mut GameMode, party: &mut Party, _enemies: &Vec<Vec<Enemy>>, mut cursor_start: (usize, usize)) -> MenuScreen {
  party.inventory.sort_by(|a, b| { // Sort items with amount 0 to the end of array.
    let a_amount = match a {
      InventoryElement::Item(_, amount) => match *amount {0 => 0, _ => 1}
    };
    let b_amount = match b {
      InventoryElement::Item(_, amount) => match *amount {0 => 0, _ => 1}
    };
    return b_amount.cmp(&a_amount);
  });
  let container = MenuContainer::new(ctx, 10. , 10., 1060., 700.);
  let mut first_item_column  = Vec::new();
  let mut second_item_column = Vec::new();
  let mut unselectable_items = Vec::new();
  for (index, element) in party.inventory.iter_mut().enumerate() {
    let (element_name, element_amount, click_event) = match element {
      InventoryElement::Item(item, amount) => (item.get_name(), amount, item.get_to_menu_click_event((index % 2, index / 2)))
    };
    let item_height = (index / 2) as f32 * 24. + 50.;
    if *element_amount > 0 {
      if index % 2 == 0 {
        first_item_column.push(MenuItem::new(ctx, "".to_owned(), element_name, (0. + 100., item_height), 24., click_event));
        unselectable_items.push(MenuItem::new(ctx, "".to_owned(), format!("{}", element_amount), (400. + 100., item_height), 24., OnClickEvent::None));
      } else {
        second_item_column.push(MenuItem::new(ctx, "".to_owned(), element_name, (500. + 100., item_height), 24., click_event));
        unselectable_items.push(MenuItem::new(ctx, "".to_owned(), format!("{}", element_amount), (900. + 100., item_height), 24., OnClickEvent::None));
      }
    }
  }
  let selectable_items = match first_item_column.len() {
    0 => vec![vec![MenuItem::new(ctx, "".to_owned(), " ".to_owned(), (0., 0.), 24., OnClickEvent::None)]],
    _ => vec![first_item_column, second_item_column]
  };
  if selectable_items[cursor_start.0].get(cursor_start.1).is_none() { // Change cursor position if last item of inventory was deplenished
    if cursor_start.0 % 2 == 0 {
      cursor_start.0 += 1;
      cursor_start.1 -= 1;
    } else {
      cursor_start.0 -= 1;
    }
  }
  MenuScreen::new(
    ctx,
    true,
    vec![container],
    selectable_items,
    unselectable_items,
    cursor_start,
    MenuMovement::Grid,
    OnClickEvent::MenuTransition(main_menu, (0, 0))
  )
}

pub fn row_change_menu(ctx: &mut Context, _mode: &mut GameMode, party: &mut Party, _enemies: &Vec<Vec<Enemy>>, cursor_start: (usize, usize)) -> MenuScreen {
  let submenu_selection = MenuContainer::new(ctx, 10. , 10., 250., 300.);
  let character_info    = MenuContainer::new(ctx, 275., 10., 795., 700.);
  fn to_main_menu(ctx: &mut Context, mode: &mut GameMode, party: &mut Party, enemies: &Vec<Vec<Enemy>>, cursor_start: (usize, usize)) -> MenuScreen {
    main_menu(ctx, mode, party, enemies, cursor_start)
  }
  let mut selectable_items = Vec::new();
  let mut unselectable_items = Vec::new();
  unselectable_items.push(text!(ctx, "Item"   , 55., 60. , OnClickEvent::None));
  unselectable_items.push(text!(ctx, "Ability", 55., 100., OnClickEvent::None));
  unselectable_items.push(text!(ctx, "Equip"  , 55., 140., OnClickEvent::None));
  unselectable_items.push(text!(ctx, "Row"    , 55., 180., OnClickEvent::None));
  unselectable_items.push(text!(ctx, "Config" , 55., 220., OnClickEvent::None));
  fn push_party_memeber_to_menu_items(ctx: &mut Context, selectables: &mut Vec<MenuItem>, unselectables: &mut Vec<MenuItem>, character: &Character) {
    fn toggle_row(menu: &mut MenuScreen, party: &mut Party) -> GameResult<()> {
      let mut characters_in_party = Vec::new();
      if party.first .name.len() > 0 {characters_in_party.push(0)}
      if party.second.name.len() > 0 {characters_in_party.push(1)}
      if party.third .name.len() > 0 {characters_in_party.push(2)}
      if party.fourth.name.len() > 0 {characters_in_party.push(3)}
      let selected_index = match menu.cursor_pos.1 {
        0 => characters_in_party[0],
        1 => characters_in_party[1],
        2 => characters_in_party[2],
        _ => characters_in_party[3]
      };
      let selected_character = match selected_index {
        0 => &mut party.first,
        1 => &mut party.second,
        2 => &mut party.third,
        _ => &mut party.fourth
      };
      if selected_character.state.back_row {
        selected_character.state.back_row = false;
        selected_character.x_offset += 50.;
        menu.selectable_items[0][selected_index].screen_pos.0 += 50.;
      } else {
        selected_character.state.back_row = true;
        selected_character.x_offset -= 50.;
        menu.selectable_items[0][selected_index].screen_pos.0 -= 50.;
      }
      menu.mutation = MenuMutation::None;
      Ok(())
    }
    if character.name.len() > 0 {
      let offset_by_id = (character.state.id - 1) as f32 * 162.;
      selectables.push(MenuItem::new(
        ctx,
        character.avatar_spritefile.to_owned(),
        "".to_owned(),
        (360. + character.x_offset, 52. + offset_by_id),
        128.,
        OnClickEvent::MutateMenu(toggle_row)
      ));
      unselectables.push(MenuItem::new(ctx, "".to_owned(), format!("Lvl {}", character.state.level), (500., 60. + offset_by_id), 24., OnClickEvent::None));
      unselectables.push(MenuItem::new(ctx, "".to_owned(), character.name.to_owned(), (680., 60. + offset_by_id), 24., OnClickEvent::None));
      unselectables.push(MenuItem::new(
        ctx,
        "".to_owned(),
        format!("{}/", character.state.hp),
        (500. + (5 - format!("{}/", character.state.hp).len()) as f32 * 24., 100. + offset_by_id),
        24.,
        OnClickEvent::None
      ));
      unselectables.push(MenuItem::new(
        ctx,
        "".to_owned(),
        format!("{}/", character.state.mp),
        (822. + (4 - format!("{}/", character.state.mp).len()) as f32 * 24., 100. + offset_by_id),
        24.,
        OnClickEvent::None
      ));
      unselectables.push(MenuItem::new(
        ctx,
        "".to_owned(),
        format!("{}Hp", character.state.hp),
        (620. + (4 - format!("{}", character.state.hp).len()) as f32 * 24., 100. + offset_by_id),
        24.,
        OnClickEvent::None
      ));
      unselectables.push(MenuItem::new(
        ctx,
        "".to_owned(),
        format!("{}Mp",
        character.state.mp),
        (918. + (3 - format!("{}", character.state.mp).len()) as f32 * 24., 100. + offset_by_id),
        24.,
        OnClickEvent::None
      ));
    }
  }
  push_party_memeber_to_menu_items(ctx, &mut selectable_items, &mut unselectable_items, &party.first);
  push_party_memeber_to_menu_items(ctx, &mut selectable_items, &mut unselectable_items, &party.second);
  push_party_memeber_to_menu_items(ctx, &mut selectable_items, &mut unselectable_items, &party.third);
  push_party_memeber_to_menu_items(ctx, &mut selectable_items, &mut unselectable_items, &party.fourth);
  MenuScreen::new(
    ctx,
    true,
    vec![submenu_selection, character_info],
    vec![selectable_items],
    unselectable_items,
    cursor_start,
    MenuMovement::RowOfColumns,
    OnClickEvent::ToMenuScreen(to_main_menu, (0, 3))
  )
}

pub fn to_target_selection(
  ctx: &mut Context,
  party: &mut Party,
  enemies: &Vec<Vec<Enemy>>,
  action_parameters: &ActionParameters,
  cursor_memory: (usize, usize)
) -> MenuScreen {
  battle_target_selection(ctx, party, enemies, (1, 0), action_parameters, cursor_memory)
}

pub fn battle_main(ctx: &mut Context, character: &mut Character, cursor_start: (usize, usize)) -> MenuScreen {
  let commands = MenuContainer::new(ctx, 10., 400., 280., 300.);
  let attack_ability    = character.get_attack_ability(ctx);
  let primary_ability   = character.get_primary_ability(ctx);
  let secondary_ability = character.get_secondary_ability(ctx);
  let item       = text!(ctx, "Item"  , 55., 560., OnClickEvent::ToMenuScreen(battle_item_menu, (0, 3)));
  let defend     = text!(ctx, "Defend", 55., 600., OnClickEvent::None);
  let row_change = text!(ctx, "Change", 55., 640., OnClickEvent::None);
  MenuScreen::new(
    ctx,
    true,
    vec![commands],
    vec![vec![attack_ability, primary_ability, secondary_ability, item, defend, row_change]],
    Vec::new(),
    cursor_start,
    MenuMovement::Grid,
    OnClickEvent::None
  )
}

fn to_battle_main(ctx: &mut Context, _mode: &mut GameMode, party: &mut Party, _enemies: &Vec<Vec<Enemy>>, cursor_start: (usize, usize)) -> MenuScreen {
  battle_main(ctx, party.get_active(), cursor_start)
}

pub fn battle_item_menu(ctx: &mut Context, _mode: &mut GameMode, party: &mut Party, _enemies: &Vec<Vec<Enemy>>, cursor_start: (usize, usize)) -> MenuScreen {
  let commands = MenuContainer::new(ctx, 10., 400., 280., 300.);
  let mut selectable_items   = Vec::new();
  let mut unselectable_items = Vec::new();
  let mut index = 0;
  for inventory_item in party.inventory.iter() {
    match inventory_item {
      InventoryElement::Item(item, amount) => {
        if *amount > 0 {
          selectable_items  .push(text!(ctx, item.get_name()       , 55. , 440. + index as f32 * 40., item.get_target_selection_click_event(index)));
          unselectable_items.push(text!(ctx, format!("x{}", amount), 220., 440. + index as f32 * 40., OnClickEvent::None));
          index += 1;
        }
      }
    };
  }
  if selectable_items.len() == 0 {
    selectable_items.push(MenuItem::new(ctx, "/empty.png".to_owned(), "".to_owned(), (0., 0.), 0., OnClickEvent::None));
  }
  MenuScreen::new(
    ctx,
    true,
    vec![commands],
    vec![selectable_items],
    unselectable_items,
    (0, 0),
    MenuMovement::Grid,
    OnClickEvent::ToMenuScreen(to_battle_main, cursor_start)
  )
}

pub fn battle_target_selection(
  ctx: &mut Context,
  party: &mut Party,
  enemies: &Vec<Vec<Enemy>>,
  cursor_pos: (usize, usize),
  action_parameters: &ActionParameters,
  cursor_memory: (usize, usize)
) -> MenuScreen {
  let commands = MenuContainer::new(ctx, 10., 400., 280., 300.);
  let target_positions = battle_target_positions!(ctx, party, enemies, action_parameters);
  let attack_ability    = party.get_active().get_attack_ability(ctx);
  let primary_ability   = party.get_active().get_primary_ability(ctx);
  let secondary_ability = party.get_active().get_secondary_ability(ctx);
  let item       = text!(ctx, "Item"  , 55., 560., OnClickEvent::None);
  let defend     = text!(ctx, "Defend", 55., 600., OnClickEvent::None);
  let row_change = text!(ctx, "Change", 55., 640., OnClickEvent::None);
  MenuScreen::new(
    ctx,
    true,
    vec![commands],
    target_positions,
    vec![attack_ability, primary_ability, secondary_ability, item, defend, row_change],
    cursor_pos,
    MenuMovement::Grid,
    OnClickEvent::ToMenuScreen(to_battle_main, cursor_memory)
  )
}

pub fn battle_won(ctx: &mut Context, party: &mut Party, experience: &mut u32) -> MenuScreen {
  fn start_exp_count(menu: &mut MenuScreen, _party: &mut Party) -> GameResult<()> {
    fn count_experience(menu: &mut MenuScreen, party: &mut Party) -> GameResult<()> {
      fn finish_exp_count(menu: &mut MenuScreen, _party: &mut Party) -> GameResult<()> {
        fn end_exp_cound(menu: &mut MenuScreen, _party: &mut Party) -> GameResult<()> {
          let exp_left = menu.unselectable_items[1].text.parse::<u32>().unwrap();
          if exp_left > 0 {
            menu.unselectable_items[1].text = format!("{}", 0);
            menu.unselectable_items[3].text = format!("{}", menu.unselectable_items[3].text.parse::<u32>().unwrap() + exp_left);
            menu.unselectable_items[5].text = format!("{}", menu.unselectable_items[5].text.parse::<u32>().unwrap() + exp_left);
            menu.unselectable_items[7].text = format!("{}", menu.unselectable_items[7].text.parse::<u32>().unwrap() + exp_left);
            menu.unselectable_items[9].text = format!("{}", menu.unselectable_items[9].text.parse::<u32>().unwrap() + exp_left);
          } else {
            menu.mutation = MenuMutation::None;
          }
          menu.selectable_items[0][0].on_click = OnClickEvent::Transition(GameMode::Map);
          Ok(())
        }
        menu.mutation = MenuMutation::DefaultMutation(end_exp_cound);
        Ok(())
      }
      let exp_left = menu.unselectable_items[1].text.parse::<u32>().unwrap();
      if exp_left > 0 {
        menu.unselectable_items[1].text = format!("{}", exp_left - party.get_alive_size());
        if party.first .name.len() > 0 && party.first.state.hp > 0 {
          menu.unselectable_items[3].text = format!("{}", menu.unselectable_items[3].text.parse::<u32>().unwrap() + 1);
        }
        if party.second.name.len() > 0 && party.second.state.hp > 0 {
          menu.unselectable_items[5].text = format!("{}", menu.unselectable_items[5].text.parse::<u32>().unwrap() + 1);
        }
        if party.third .name.len() > 0 && party.third.state.hp > 0 {
          menu.unselectable_items[7].text = format!("{}", menu.unselectable_items[7].text.parse::<u32>().unwrap() + 1);
        }
        if party.fourth.name.len() > 0 && party.fourth.state.hp > 0 {
          menu.unselectable_items[9].text = format!("{}", menu.unselectable_items[9].text.parse::<u32>().unwrap() + 1);
        }
        menu.selectable_items[0][0].on_click = OnClickEvent::MutateMenu(finish_exp_count);
      } else {
        finish_exp_count(menu, party)?;
      }
      Ok(())
    }
    menu.mutation = MenuMutation::DefaultMutation(count_experience);
    Ok(())
  }
  let experience_container   = MenuContainer::new(ctx, 50. , 10. , 500. , 100.);
  let character_container    = MenuContainer::new(ctx, 10. , 120., 1060., 240.);
  let items_string_container = MenuContainer::new(ctx, 50. , 370., 200. , 100.);
  let items_container        = MenuContainer::new(ctx, 10. , 480., 1060., 120.);
  let continue_container     = MenuContainer::new(ctx, 760., 610., 280. , 100.);
  let gained_exp_string      = text!(ctx, "Experience", 90. , 50. , OnClickEvent::None);
  while *experience % party.get_alive_size() > 0 {
    *experience += 1;
  }
  let gained_exp = text!(ctx, format!("{}", experience), 350., 50. , OnClickEvent::None);
  let first_character_name = text!(ctx, match party.first.name.len() > 0 {
    true  => format!("{}", party.first.name),
    false => " ".to_owned()
  }, 50. , 160., OnClickEvent::None);
  let first_character_exp = text!(ctx, match party.first.name.len() > 0 {
    true  => format!("{}", party.first.state.experience),
    false => " ".to_owned()
  }, 300., 160., OnClickEvent::None);
  let second_character_name = text!(ctx, match party.second.name.len() > 0 {
    true  => format!("{}", party.second.name),
    false => " ".to_owned()
  }, 590., 160., OnClickEvent::None);
  let second_character_exp = text!(ctx, match party.second.name.len() > 0 {
    true  => format!("{}", party.second.state.experience),
    false => " ".to_owned()
  }, 840., 160., OnClickEvent::None);
  let third_character_name = text!(ctx, match party.third.name.len() > 0 {
    true  => format!("{}", party.third.name),
    false => " ".to_owned()
  }, 50., 260., OnClickEvent::None);
  let third_character_exp = text!(ctx, match party.third.name.len() > 0 {
    true  => format!("{}", party.third.state.experience),
    false => " ".to_owned()
  }, 300., 260., OnClickEvent::None);
  let fourth_character_name = text!(ctx, match party.fourth.name.len() > 0 {
    true  => format!("{}", party.fourth.name),
    false => " ".to_owned()
  }, 590., 260., OnClickEvent::None);
  let fourth_character_exp = text!(ctx, match party.fourth.name.len() > 0 {
    true  => format!("{}", party.fourth.state.experience),
    false => " ".to_owned()
  }, 840., 260., OnClickEvent::None);
  let gained_items_string = text!(ctx, "Items", 90. , 410., OnClickEvent::None);
  let continue_button = text!(ctx, "Continue", 805., 650., OnClickEvent::MutateMenu(start_exp_count));
  if party.first .name.len() > 0 {party.first .state.experience += *experience / party.get_alive_size();}
  if party.second.name.len() > 0 {party.second.state.experience += *experience / party.get_alive_size();}
  if party.third .name.len() > 0 {party.third .state.experience += *experience / party.get_alive_size();}
  if party.fourth.name.len() > 0 {party.fourth.state.experience += *experience / party.get_alive_size();}
  MenuScreen::new(
    ctx,
    true,
    vec![experience_container, character_container, items_string_container, items_container, continue_container],
    vec![vec![continue_button]],
    vec![
      gained_exp_string, gained_exp,
       first_character_name,  first_character_exp,
      second_character_name, second_character_exp,
       third_character_name,  third_character_exp,
      fourth_character_name, fourth_character_exp,
      gained_items_string
    ],
    (0, 0),
    MenuMovement::Grid,
    OnClickEvent::None
  )
}