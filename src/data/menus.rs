use ggez::{Context, GameResult};
use crate::GameMode;
use crate::menu::MenuScreen;
use crate::menu::item::MenuItem;
use crate::menu::container::MenuContainer;
use crate::party::Party;
use crate::party::character::Character;
use crate::battle::enemy::Enemy;
use crate::battle::action::{ActionParameters, DamageType};
use crate::menu::item::OnClickEvent;

pub fn none_menu(ctx: &mut Context) -> MenuScreen {
  MenuScreen::new(ctx, false, Vec::new(), vec![Vec::new()], Vec::new(), (0, 0), false, OnClickEvent::None)
}

pub fn menu_main(ctx: &mut Context) -> MenuScreen {
  fn to_none_menu(ctx: &mut Context, _mode: &mut GameMode, _party: &mut Party, _enemies: &Vec<Vec<Enemy>>) -> MenuScreen {
    none_menu(ctx)
  }
  let submenu_selection = MenuContainer::new(ctx, 10. , 10., 250., 300.);
  let character_info    = MenuContainer::new(ctx, 275., 10., 795., 700.);
  let item    = text!(ctx, "Item"   , 55., 60. , OnClickEvent::None);
  let ability = text!(ctx, "Ability", 55., 100., OnClickEvent::None);
  let equip   = text!(ctx, "Equip"  , 55., 140., OnClickEvent::None);
  let config  = text!(ctx, "Config" , 55., 180., OnClickEvent::None);
  MenuScreen::new(
    ctx,
    true,
    vec![submenu_selection, character_info],
    vec![
      vec![item, ability, equip, config]
    ],
    Vec::new(),
    (0, 0),
    true,
    OnClickEvent::ToMenuScreen(to_none_menu)
  )
}

pub fn battle_main(ctx: &mut Context) -> MenuScreen {
  fn to_target_selection(ctx: &mut Context, party: &mut Party, enemies: &Vec<Vec<Enemy>>, action_parameters: &ActionParameters) -> MenuScreen {
    battle_target_selection(ctx, party, enemies, (1, 0), action_parameters)
  }
  let commands = MenuContainer::new(ctx, 10., 480., 260., 220.);
  let action_parameters = ActionParameters::new(DamageType::Physical, 4, 0., false, 0.9, false, 0.9, false);
  let attack         = text!(ctx, "Attack", 55., 520., OnClickEvent::ToTargetSelection(to_target_selection, action_parameters));
  let first_ability  = text!(ctx, "Steal" , 55., 560., OnClickEvent::None);
  let second_ability = text!(ctx, "Flee"  , 55., 600., OnClickEvent::None);
  let item           = text!(ctx, "Item"  , 55., 640., OnClickEvent::None);
  MenuScreen::new(
    ctx,
    true,
    vec![commands],
    vec![vec![attack, first_ability, second_ability, item]],
    Vec::new(),
    (0, 0),
    true,
    OnClickEvent::None
  )
}

pub fn battle_target_selection(
  ctx: &mut Context,
  party: &mut Party,
  enemies: &Vec<Vec<Enemy>>,
  cursor_pos: (usize, usize),
  action_parameters: &ActionParameters
) -> MenuScreen {
  fn to_battle_main(ctx: &mut Context, _mode: &mut GameMode, _party: &mut Party, _enemies: &Vec<Vec<Enemy>>) -> MenuScreen {
    battle_main(ctx)
  }
  let commands       = MenuContainer::new(ctx, 10., 480., 260., 220.);
  let target_positions = battle_target_positions!(ctx, party, enemies, action_parameters);
  let attack         = text!(ctx, "Attack", 55., 520., OnClickEvent::None);
  let first_ability  = text!(ctx, "Steal" , 55., 560., OnClickEvent::None);
  let second_ability = text!(ctx, "Flee"  , 55., 600., OnClickEvent::None);
  let item           = text!(ctx, "Item"  , 55., 640., OnClickEvent::None);
  MenuScreen::new(
    ctx,
    true,
    vec![commands],
    target_positions,
    vec![attack, first_ability, second_ability, item],
    cursor_pos,
    true,
    OnClickEvent::ToMenuScreen(to_battle_main)
  )
}

pub fn battle_won(ctx: &mut Context, party: &mut Party, experience: &mut u32) -> MenuScreen {
  fn start_exp_count(menu: &mut MenuScreen, _party: &mut Party) -> GameResult<()> {
    fn count_experience(menu: &mut MenuScreen, party: &mut Party) -> GameResult<()> {
      let exp_left = menu.unselectable_items[1].text.parse::<u32>().unwrap();
      if exp_left > 0 {
        menu.unselectable_items[1].text = format!("{}", exp_left - party.get_size());
        if party.first .name.len() > 0 {menu.unselectable_items[3].text = format!("{}", menu.unselectable_items[3].text.parse::<u32>().unwrap() + 1);}
        if party.second.name.len() > 0 {menu.unselectable_items[5].text = format!("{}", menu.unselectable_items[5].text.parse::<u32>().unwrap() + 1);}
        if party.third .name.len() > 0 {menu.unselectable_items[7].text = format!("{}", menu.unselectable_items[7].text.parse::<u32>().unwrap() + 1);}
        if party.fourth.name.len() > 0 {menu.unselectable_items[9].text = format!("{}", menu.unselectable_items[9].text.parse::<u32>().unwrap() + 1);}
          menu.selectable_items[0][0].on_click = OnClickEvent::MutateMenu(finish_exp_count);
      } else {
        finish_exp_count(menu, party)?;
      }
      Ok(())
    }
    menu.mutation = Some(count_experience);
    Ok(())
  }
  fn finish_exp_count(menu: &mut MenuScreen, _party: &mut Party) -> GameResult<()> {
    fn end_exp_cound(menu: &mut MenuScreen, _party: &mut Party) -> GameResult<()> {
      let exp_left = menu.unselectable_items[1].text.parse::<u32>().unwrap();
      if exp_left > 0 {
        menu.unselectable_items[1].text = format!("{}", 0);
        menu.unselectable_items[3].text = format!("{}", menu.unselectable_items[3].text.parse::<u32>().unwrap() + exp_left);
        menu.unselectable_items[5].text = format!("{}", menu.unselectable_items[5].text.parse::<u32>().unwrap() + exp_left);
        menu.unselectable_items[7].text = format!("{}", menu.unselectable_items[7].text.parse::<u32>().unwrap() + exp_left);
        menu.unselectable_items[9].text = format!("{}", menu.unselectable_items[9].text.parse::<u32>().unwrap() + exp_left);
      }
      menu.selectable_items[0][0].on_click = OnClickEvent::Transition(GameMode::Map);
      Ok(())
    }
    menu.mutation = Some(end_exp_cound);
    Ok(())
  }
  let experience_container   = MenuContainer::new(ctx, 50. , 10. , 500. , 100.);
  let character_container    = MenuContainer::new(ctx, 10. , 120., 1060., 240.);
  let items_string_container = MenuContainer::new(ctx, 50. , 370., 200. , 100.);
  let items_container        = MenuContainer::new(ctx, 10. , 480., 1060., 120.);
  let continue_container     = MenuContainer::new(ctx, 760., 610., 280. , 100.);
  let gained_exp_string      = text!(ctx, "Experience", 90. , 50. , OnClickEvent::None);
  while *experience % party.get_size() > 0 {
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
  if party.first .name.len() > 0 {party.first .state.experience += *experience / party.get_size();}
  if party.second.name.len() > 0 {party.second.state.experience += *experience / party.get_size();}
  if party.third .name.len() > 0 {party.third .state.experience += *experience / party.get_size();}
  if party.fourth.name.len() > 0 {party.fourth.state.experience += *experience / party.get_size();}
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
    true,
    OnClickEvent::None
  )
}