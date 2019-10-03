use ggez::{Context, GameResult};
use crate::GameMode;
use crate::battle::enemy::Enemy;
use crate::battle::state::BattleState;
use crate::menu::{MenuScreen, MenuMovement};
use crate::menu::container::MenuContainer;
use crate::menu::item::{MenuItem, OnClickEvent};
use crate::party::{Party, InventoryElement};
use crate::data::menus;
use std::cmp::min;

pub enum ItemVariant {
  Potion
}

pub struct Item {
  variant: ItemVariant
}

impl Item {
  pub fn new(variant: ItemVariant) -> Item {
    Item {
      variant
    }
  }

  pub fn use_on_targets(&self, targets: &mut Vec<BattleState>) -> GameResult<()> {
    match self.variant {
      ItemVariant::Potion => targets[0].hp = min(targets[0].hp + 100, targets[0].max_hp)
    }
    Ok(())
  }

  pub fn get_name(&self) -> String {
    match self.variant {
      ItemVariant::Potion => "Potion".to_owned()
    }
  }

  pub fn get_click_event(&self, item_cursor_pos: (usize, usize)) -> OnClickEvent {
    fn select_target(ctx: &mut Context, _mode: &mut GameMode, party: &mut Party, _enemies: &Vec<Vec<Enemy>>, item_cursor_pos: (usize, usize)) -> MenuScreen {
      let container = MenuContainer::new(ctx, 10. , 10., 1060., 700.);
      let selection = MenuContainer::new(ctx, 50. , 50., 500. , 600.);
      let mut characters = Vec::new();
      if party.first .name.len() > 0 {characters.push(MenuItem::new(ctx, party.first .get_avatar(), "".to_owned(), (90., 90. ), OnClickEvent::None))}
      if party.second.name.len() > 0 {characters.push(MenuItem::new(ctx, party.second.get_avatar(), "".to_owned(), (90., 240.), OnClickEvent::None))}
      if party.third .name.len() > 0 {characters.push(MenuItem::new(ctx, party.third .get_avatar(), "".to_owned(), (90., 390.), OnClickEvent::None))}
      if party.fourth.name.len() > 0 {characters.push(MenuItem::new(ctx, party.fourth.get_avatar(), "".to_owned(), (90., 440.), OnClickEvent::None))}
      let mut unselectables = Vec::new();
      for (index, element) in party.inventory.iter_mut().enumerate() {
        let element_name = match element {
          InventoryElement::Item(item) => item.get_name()
        };
        if index % 2 == 0 {
          unselectables.push(MenuItem::new(ctx, "".to_owned(), element_name, ((index % 2) as f32 * 500. + 100., (index / 2) as f32 * 24. + 50.), OnClickEvent::None));
        } else {
          unselectables.push(MenuItem::new(ctx, "".to_owned(), element_name, ((index % 2) as f32 * 500. + 100., (index / 2) as f32 * 24. + 50.), OnClickEvent::None));
        }
      }
      MenuScreen::new(
        ctx,
        true,
        vec![container, selection],
        vec![characters],
        unselectables,
        (0, 0),
        MenuMovement::Grid,
        OnClickEvent::ToMenuScreen(menus::item_menu, item_cursor_pos)
      )
    }
    match self.variant {
      ItemVariant::Potion => OnClickEvent::ToMenuScreen(select_target, item_cursor_pos)
    }
  }
}