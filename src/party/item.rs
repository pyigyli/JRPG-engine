use ggez::{Context, GameResult};
use ggez::graphics::Color;
use crate::GameMode;
use crate::battle::action::{ActionParameters, DamageType};
use crate::battle::enemy::Enemy;
use crate::battle::print_damage::PrintDamage;
use crate::battle::state::BattleState;
use crate::menu::{MenuScreen, MenuMovement, MenuMutation};
use crate::menu::container::MenuContainer;
use crate::menu::item::{MenuItem, OnClickEvent};
use crate::party::{Party, InventoryElement};
use crate::data::menus;
use std::cmp::min;

#[derive(Clone)]
pub enum ItemVariant {
  Potion,
  Ether
}

#[derive(Clone)]
pub struct InventoryItem {
  variant: ItemVariant
}

impl InventoryItem {
  pub fn new(variant: ItemVariant) -> InventoryItem {
    InventoryItem {
      variant
    }
  }

  pub fn apply_item_effect(&self, ctx: &mut Context, target: &mut BattleState, position: (f32, f32)) -> GameResult<()> { // negative position.0 mean no battle
    let (print_damage, print_color) = match self.variant {
      ItemVariant::Potion => {
        target.hp = min(target.hp + 100, target.max_hp);
        if let Some(info) = &mut target.character_info {info.hp.text = format!("{}/", target.hp);}
        (100, Color::new(0., 1., 0., 1.))
      },
      ItemVariant::Ether => {
        target.mp = min(target.mp + 25, target.max_mp);
        if let Some(info) = &mut target.character_info {info.mp.text = format!("{}/", target.mp);}
        (25, Color::new(0., 0., 1., 1.))
      }
    };
    if position.0 >= 0. {
      target.print_damage = Some(PrintDamage::new(ctx, print_damage, target.get_damage_position(position), print_color));
    }
    Ok(())
  }

  pub fn get_name(&self) -> String {
    match self.variant {
      ItemVariant::Potion => "Potion".to_owned(),
      ItemVariant::Ether  => "Ether".to_owned()
    }
  }

  pub fn get_to_menu_click_event(&self, item_cursor_pos: (usize, usize)) -> OnClickEvent {
    fn select_single_target(ctx: &mut Context, _mode: &mut GameMode, party: &mut Party, _enemies: &Vec<Vec<Enemy>>, item_cursor_pos: (usize, usize)) -> MenuScreen {
      fn use_item(
        ctx: &mut Context,
        menu: &mut MenuScreen,
        mode: &mut GameMode,
        party: &mut Party,
        enemies: &Vec<Vec<Enemy>>,
        item_cursor_pos: (usize, usize),
        targets: Vec<u8>
      ) -> GameResult<()> {
        let mut out_of_selected_item = false;
        for inventory_element in &mut party.inventory {
          match inventory_element {
            InventoryElement::Item(item, amount) => {
              if format!("{} x{}", item.get_name(), amount) == menu.unselectable_items[0].text && *amount > 0 {
                *amount -= 1;
                if targets.contains(&0) {item.apply_item_effect(ctx, &mut party.first .state, (-1., 0.))?;}
                if targets.contains(&1) {item.apply_item_effect(ctx, &mut party.second.state, (-1., 0.))?;}
                if targets.contains(&2) {item.apply_item_effect(ctx, &mut party.third .state, (-1., 0.))?;}
                if targets.contains(&3) {item.apply_item_effect(ctx, &mut party.fourth.state, (-1., 0.))?;}
                if *amount == 0 {
                  out_of_selected_item = true;
                } else {
                  for element in &mut menu.unselectable_items {
                    if element.text == format!("{} x{}", item.get_name(), *amount + 1) {
                      element.text = format!("{} x{}", item.get_name(), amount);
                    }
                  }
                }
              }
            }
          }
        }
        if out_of_selected_item {
          *menu = menus::item_menu(ctx, mode, party, enemies, item_cursor_pos);
        } else {
          menu.mutation = MenuMutation::None;
        }
        Ok(())
      }
      let container           = MenuContainer::new(ctx, 10. , 10., 1060., 700.);
      let selection           = MenuContainer::new(ctx, 50. , 50., 480. , 600.);
      let item_name_container = MenuContainer::new(ctx, 550., 50., 480. , 100.);
      let mut characters = Vec::new();
      if party.first .name.len() > 0 {
        characters.push(MenuItem::new(ctx, party.first .get_avatar(), "".to_owned(), (90., 90. ), 128., OnClickEvent::UseItemInMenu(use_item, vec!(0), item_cursor_pos)))
      }
      if party.second.name.len() > 0 {
        characters.push(MenuItem::new(ctx, party.second.get_avatar(), "".to_owned(), (90., 240.), 128., OnClickEvent::UseItemInMenu(use_item, vec!(1), item_cursor_pos)))
      }
      if party.third .name.len() > 0 {
        characters.push(MenuItem::new(ctx, party.third .get_avatar(), "".to_owned(), (90., 390.), 128., OnClickEvent::UseItemInMenu(use_item, vec!(2), item_cursor_pos)))
      }
      if party.fourth.name.len() > 0 {
        characters.push(MenuItem::new(ctx, party.fourth.get_avatar(), "".to_owned(), (90., 440.), 128., OnClickEvent::UseItemInMenu(use_item, vec!(3), item_cursor_pos)))
      }
      let mut unselectables = Vec::new();
      for (index, element) in party.inventory.iter_mut().enumerate() {
        let (element_name, element_amount) = match element {
          InventoryElement::Item(item, amount) => (item.get_name(), amount)
        };
        let item_height = (index / 2) as f32 * 24. + 50.;
        if item_cursor_pos.0 == index % 2 && item_cursor_pos.1 == index / 2 {
          unselectables.push(MenuItem::new(ctx, "".to_owned(), format!("{} x{}", element_name, element_amount), (590., 90.), 24., OnClickEvent::None));
          unselectables.reverse(); // Places element to index 0, so it's easily accessible.
        }
        if index % 2 == 1 && index / 2 > 2 {
          unselectables.push(MenuItem::new(ctx, "".to_owned(), element_name, (500. + 100., item_height), 24., OnClickEvent::None));
          unselectables.push(MenuItem::new(ctx, "".to_owned(), format!("{}", element_amount), (900. + 100., item_height), 24., OnClickEvent::None));
        }
      }
      MenuScreen::new(
        ctx,
        true,
        vec![container, selection, item_name_container],
        vec![characters],
        unselectables,
        (0, 0),
        MenuMovement::Grid,
        OnClickEvent::ToMenuScreen(menus::item_menu, item_cursor_pos)
      )
    }
    match self.variant {
      ItemVariant::Potion => OnClickEvent::ToMenuScreen(select_single_target, item_cursor_pos),
      ItemVariant::Ether  => OnClickEvent::ToMenuScreen(select_single_target, item_cursor_pos)
    }
  }

  pub fn get_target_selection_click_event(&self, item_index: usize) -> OnClickEvent {
    match self.variant {
      ItemVariant::Potion => OnClickEvent::ToTargetSelection(
        menus::to_target_selection,
        ActionParameters::new(DamageType::Item(self.clone()), 0, 0., false, 0., false, 0., false),
        (0, item_index)
      ),
      ItemVariant::Ether => OnClickEvent::ToTargetSelection(
        menus::to_target_selection,
        ActionParameters::new(DamageType::Item(self.clone()), 0, 0., false, 0., false, 0., false),
        (0, item_index)
      )
    }
  }
}