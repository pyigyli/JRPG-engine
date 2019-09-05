use ggez::{Context, GameResult};
use crate::GameMode;
use crate::menu::MenuScreen;
use crate::menu::item::MenuItem;
use crate::menu::container::MenuContainer;
use crate::party::Party;
use crate::party::character::Character;
use crate::battle::enemy::Enemy;
use crate::menu::item::OnClickEvent;

pub fn none_menu(ctx: &mut Context) -> MenuScreen {
  MenuScreen::new(ctx, false, vec![], vec![vec![]], vec![], (0, 0), false, OnClickEvent::None)
}

pub fn menu_main(ctx: &mut Context) -> MenuScreen {
  fn to_none_menu(ctx: &mut Context, _mode: &mut GameMode, _party: &mut Party, _enemies: &Vec<Vec<Enemy>>) -> MenuScreen {
    none_menu(ctx)
  }
  let submenu_selection = MenuContainer::new(ctx, 10. , 10., 250., 300.);
  let characters        = MenuContainer::new(ctx, 275., 10., 795., 700.);
  let item    = text!(ctx, "Item"   , 55., 60. , OnClickEvent::None);
  let ability = text!(ctx, "Ability", 55., 100., OnClickEvent::None);
  let equip   = text!(ctx, "Equip"  , 55., 140., OnClickEvent::None);
  let config  = text!(ctx, "Config" , 55., 180., OnClickEvent::None);
  MenuScreen::new(
    ctx,
    true,
    vec![submenu_selection, characters],
    vec![
      vec![item, ability, equip, config]
    ],
    vec![],
    (0, 0),
    true,
    OnClickEvent::ToMenuScreen(to_none_menu)
  )
}

pub fn battle_main(ctx: &mut Context) -> MenuScreen {
  fn to_target_selection(ctx: &mut Context, _mode: &mut GameMode, party: &mut Party, enemies: &Vec<Vec<Enemy>>) -> MenuScreen {
    battle_target_selection(ctx, party, enemies, (1, 0))
  }
  let commands = MenuContainer::new(ctx, 10., 480., 260., 220.);
  let attack         = text!(ctx, "Attack", 55., 520., OnClickEvent::ToMenuScreen(to_target_selection));
  let first_ability  = text!(ctx, "Steal" , 55., 560., OnClickEvent::None);
  let second_ability = text!(ctx, "Flee"  , 55., 600., OnClickEvent::None);
  let item           = text!(ctx, "Item"  , 55., 640., OnClickEvent::None);
  MenuScreen::new(
    ctx,
    true,
    vec![commands],
    vec![vec![attack, first_ability, second_ability, item]],
    vec![],
    (0, 0),
    true,
    OnClickEvent::None
  )
}

pub fn battle_target_selection(ctx: &mut Context, party: &mut Party, enemies: &Vec<Vec<Enemy>>, cursor_pos: (usize, usize)) -> MenuScreen {
  fn to_battle_main(ctx: &mut Context, _mode: &mut GameMode, _party: &mut Party, _enemies: &Vec<Vec<Enemy>>) -> MenuScreen {
    battle_main(ctx)
  }
  let characters = MenuContainer::new(ctx, 300., 480., 750., 220.);
  let commands = MenuContainer::new(ctx, 10., 480., 260., 220.);
  let target_positions = battle_target_positions!(ctx, party, enemies);
  let attack         = text!(ctx, "Attack", 55., 520., OnClickEvent::None);
  let first_ability  = text!(ctx, "Steal" , 55., 560., OnClickEvent::None);
  let second_ability = text!(ctx, "Flee"  , 55., 600., OnClickEvent::None);
  let item           = text!(ctx, "Item"  , 55., 640., OnClickEvent::None);
  MenuScreen::new(
    ctx,
    true,
    vec![commands, characters],
    target_positions,
    vec![attack, first_ability, second_ability, item],
    cursor_pos,
    true,
    OnClickEvent::ToMenuScreen(to_battle_main)
  )
}

pub fn battle_won(ctx: &mut Context, party: &mut Party, experience: &mut u32) -> MenuScreen {
  fn start_exp_count(menu: &mut MenuScreen) -> GameResult<()> {
    fn count_experience(menu: &mut MenuScreen) -> GameResult<()> {
      let exp_left = menu.unselectable_items[0].text.parse::<u32>().unwrap();
      if exp_left > 0 {
        menu.unselectable_items[0].text = format!("{}", exp_left - 1);
        menu.unselectable_items[1].text = format!("{}", menu.unselectable_items[1].text.parse::<u32>().unwrap() + 1);
        menu.selectable_items[0][0].on_click = OnClickEvent::MutateMenu(finish_exp_count);
      } else {
        finish_exp_count(menu)?;
      }
      Ok(())
    }
    menu.mutation = Some(count_experience);
    Ok(())
  }
  fn finish_exp_count(menu: &mut MenuScreen) -> GameResult<()> {
    fn end_exp_cound(menu: &mut MenuScreen) -> GameResult<()> {
      let exp_left = menu.unselectable_items[0].text.parse::<u32>().unwrap();
      if exp_left > 0 {
        menu.unselectable_items[0].text = format!("{}", 0);
        menu.unselectable_items[1].text = format!("{}", menu.unselectable_items[1].text.parse::<u32>().unwrap() + exp_left);
      }
      menu.selectable_items[0][0].on_click = OnClickEvent::Transition(GameMode::Map);
      Ok(())
    }
    menu.mutation = Some(end_exp_cound);
    Ok(())
  }
  let container = MenuContainer::new(ctx, 10., 10., 1060., 700.);
  let continue_button     = text!(ctx, " ", 0., 0., OnClickEvent::MutateMenu(start_exp_count));
  let gained_exp          = text!(ctx, format!("{}", experience), 100., 100., OnClickEvent::None);
  let first_character_exp = text!(ctx, format!("{}", party.first.experience), 100., 125., OnClickEvent::None);
  party.first.experience += *experience;
  MenuScreen::new(
    ctx,
    true,
    vec![container],
    vec![vec![continue_button]],
    vec![gained_exp, first_character_exp],
    (0, 0),
    true,
    OnClickEvent::None
  )
}