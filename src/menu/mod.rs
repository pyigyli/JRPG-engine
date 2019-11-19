use ggez::graphics::{spritebatch, Image, DrawParam, draw};
use ggez::nalgebra::Point2;
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::{Context, GameResult};
pub mod item;
use item::{MenuItem, OnClickEvent};
pub mod container;
use container::MenuContainer;
pub mod input_cooldowns;
use input_cooldowns::InputCooldowns;
pub mod notification;
use crate::GameMode;
use crate::party::Party;
use crate::battle::Battle;
use crate::battle::enemy::Enemy;
use crate::transition::{Transition, TransitionStyle};
use crate::data::menus;

pub enum MenuMovement {
  Grid, ColumnOfRows, RowOfColumns
}

pub enum MenuMutation {
  None,
  DefaultMutation(for<'r, 's> fn(&'r mut MenuScreen, &'s mut Party) -> GameResult<()>),
  UseItemInMenu(
    for<'r, 's, 't0, 't1, 't2> fn(
      &'r mut Context,
      &'s mut MenuScreen,
      &'t0 mut GameMode,
      &'t1 mut Party,
      &'t2 Vec<Vec<Enemy>>,
      (usize, usize),
      Vec<u8>
    ) -> GameResult<()>,
    Vec<u8>,
    (usize, usize)
  )
}

pub struct MenuScreen {
  pub open: bool,
  containers: Vec<MenuContainer>,
  pub selectable_items: Vec<Vec<MenuItem>>,
  pub unselectable_items: Vec<MenuItem>,
  cursor: spritebatch::SpriteBatch,
  pub cursor_pos: (usize, usize),
  cursor_movement_style: MenuMovement,
  input_cooldowns: InputCooldowns,
  return_action: OnClickEvent,
  pub mutation: MenuMutation
}

impl MenuScreen {
  pub fn new(
    ctx: &mut Context,
    open: bool,
    containers: Vec<MenuContainer>,
    selectable_items: Vec<Vec<MenuItem>>,
    unselectable_items: Vec<MenuItem>,
    cursor_pos: (usize, usize),
    cursor_movement_style: MenuMovement,
    return_action: OnClickEvent
  ) -> MenuScreen {
    MenuScreen {
      open,
      containers,
      selectable_items,
      unselectable_items,
      cursor: spritebatch::SpriteBatch::new(Image::new(ctx, "/cursor.png").unwrap()),
      cursor_pos,
      cursor_movement_style,
      input_cooldowns: InputCooldowns::new(),
      return_action,
      mutation: MenuMutation::None
    }
  }

  pub fn update(
    &mut self,
    ctx: &mut Context,
    mode: &mut GameMode,
    party: &mut Party,
    battle: &mut Battle,
    transition: &mut Transition
  ) -> GameResult<()> {
    if *mode == GameMode::Map && keyboard::is_key_pressed(ctx, KeyCode::F) {
      fn to_main_menu(ctx: &mut Context, mode: &mut GameMode, party: &mut Party, enemies: &Vec<Vec<Enemy>>) -> MenuScreen {
        menus::main_menu(ctx, mode, party, enemies)
      }
      transition.set(TransitionStyle::MenuIn(to_main_menu))?;
    } else if self.open {
      if keyboard::is_key_pressed(ctx, KeyCode::A) && !self.input_cooldowns.a {
        self.input_cooldowns.a = true;
        match &mut self.selectable_items[self.cursor_pos.0][self.cursor_pos.1].on_click {
          OnClickEvent::ToMenuScreen(new_menu, cursor_start) => *self = new_menu(ctx, mode, party, &battle.enemies, *cursor_start),
          OnClickEvent::ToTargetSelection(target_selection, action_parameters, cursor_memory) => {
            *self = target_selection(ctx, party, &mut battle.enemies, action_parameters, *cursor_memory);
          },
          OnClickEvent::ActOnTarget(target, action_parameters) => {
            self.open = false;
            party.battle_turn_action(ctx, battle, *target, action_parameters)?;
          },
          OnClickEvent::MutateMenu(mutation)                              => self.mutation = MenuMutation::DefaultMutation(*mutation),
          OnClickEvent::Transition(new_mode)                              => transition.set(TransitionStyle::BlackInFast(new_mode.clone()))?,
          OnClickEvent::MenuTransition(new_menu)                          => transition.set(TransitionStyle::MenuIn(*new_menu))?,
          OnClickEvent::UseItemInMenu(mutation, targets, item_cursor_pos) => self.mutation = MenuMutation::UseItemInMenu(*mutation, targets.to_vec(), *item_cursor_pos),
          OnClickEvent::None => ()
        }
      } else if !keyboard::is_key_pressed(ctx, KeyCode::A) {
        self.input_cooldowns.a = false;
      }
      if keyboard::is_key_pressed(ctx, KeyCode::S) && !self.input_cooldowns.s {
        self.input_cooldowns.s = true;
        match &self.return_action {
          OnClickEvent::ToMenuScreen(new_menu, cursor_start) => *self = new_menu(ctx, mode, party, &battle.enemies, *cursor_start),
          OnClickEvent::Transition(new_mode)                 => transition.set(TransitionStyle::BlackInFast(new_mode.clone()))?,
          OnClickEvent::MenuTransition(new_menu)             => transition.set(TransitionStyle::MenuIn(*new_menu))?,
          _ => ()
        }
      } else if !keyboard::is_key_pressed(ctx, KeyCode::S) {
        self.input_cooldowns.s = false;
      }
      if keyboard::is_key_pressed(ctx, KeyCode::Up) && !self.input_cooldowns.up {
        self.input_cooldowns.up = true;
        match self.cursor_movement_style {
          MenuMovement::Grid         => {if self.cursor_pos.1 > 0 {self.cursor_pos.1 -= 1;}},
          MenuMovement::ColumnOfRows => {if self.cursor_pos.1 > 0 {self.cursor_pos = (0, self.cursor_pos.1 - 1);}},
          MenuMovement::RowOfColumns => {if self.cursor_pos.1 > 0 {self.cursor_pos.1 -= 1;}}
        }
        return Ok(());
      } else if !keyboard::is_key_pressed(ctx, KeyCode::Up) {
        self.input_cooldowns.up = false;
      }
      if keyboard::is_key_pressed(ctx, KeyCode::Down) && !self.input_cooldowns.down {
        self.input_cooldowns.down = true;
        match self.cursor_movement_style {
          MenuMovement::Grid         => {if self.cursor_pos.1 < self.selectable_items[self.cursor_pos.0].len() - 1 {self.cursor_pos.1 += 1;}},
          MenuMovement::ColumnOfRows => {if self.cursor_pos.1 < self.selectable_items.len() - 1 {self.cursor_pos = (0, self.cursor_pos.1 + 1)}},
          MenuMovement::RowOfColumns => {if self.cursor_pos.1 < self.selectable_items[self.cursor_pos.0].len() - 1 {self.cursor_pos.1 += 1;}}
        }
        return Ok(());
      } else if !keyboard::is_key_pressed(ctx, KeyCode::Down) {
        self.input_cooldowns.down = false;
      }
      if keyboard::is_key_pressed(ctx, KeyCode::Left) && !self.input_cooldowns.left {
        self.input_cooldowns.left = true;
        match self.cursor_movement_style {
          MenuMovement::Grid         => {if self.cursor_pos.0 > 0 && self.selectable_items[self.cursor_pos.0 - 1].len() >= self.cursor_pos.1 {self.cursor_pos.0 -= 1;}},
          MenuMovement::ColumnOfRows => {if self.cursor_pos.0 > 0 {self.cursor_pos.0 -= 1;}},
          MenuMovement::RowOfColumns => {if self.cursor_pos.0 > 0 {self.cursor_pos = (self.cursor_pos.0 - 1, 0);}}
        }
        return Ok(());
      } else if !keyboard::is_key_pressed(ctx, KeyCode::Left) {
        self.input_cooldowns.left = false;
      }
      if keyboard::is_key_pressed(ctx, KeyCode::Right) && !self.input_cooldowns.right {
        self.input_cooldowns.right = true;
        match self.cursor_movement_style {
          MenuMovement::Grid => {
            if self.cursor_pos.0 < self.selectable_items.len() - 1 && self.selectable_items[self.cursor_pos.0 + 1].len() >= self.cursor_pos.1 {self.cursor_pos.0 += 1;}
          },
          MenuMovement::ColumnOfRows => {if self.cursor_pos.0 < self.selectable_items[self.cursor_pos.1].len() - 1 {self.cursor_pos.0 += 1;}},
          MenuMovement::RowOfColumns => {if self.cursor_pos.0 < self.selectable_items.len() - 1 {self.cursor_pos = (self.cursor_pos.0 + 1, 0)}}
        }
        return Ok(());
      } else if !keyboard::is_key_pressed(ctx, KeyCode::Right) {
        self.input_cooldowns.right = false;
      }
      match &self.mutation {
        MenuMutation::None => (),
        MenuMutation::DefaultMutation(mutation) => mutation(self, party)?,
        MenuMutation::UseItemInMenu(mutation, targets, item_cursor_pos) => {
          let target_states = targets.to_vec();
          let cursor_pos = *item_cursor_pos;
          mutation(ctx, self, mode, party, &mut Vec::new(), cursor_pos, target_states)?;
        }
      }
    }
    Ok(())
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    if self.open {
      for menu_container in self.containers.iter_mut() {
        menu_container.draw(ctx)?;
      }
      for item in self.selectable_items.iter_mut().flatten() {
        item.draw(ctx)?;
      }
      for item in self.unselectable_items.iter_mut() {
        item.draw(ctx)?;
      }
      if self.selectable_items.first().unwrap().len() > 0 {
        let selected_item = &self.selectable_items[self.cursor_pos.0][self.cursor_pos.1];
        let cursor_pos = selected_item.screen_pos;
        self.cursor.add(DrawParam::new());
        let param = DrawParam::new().dest(Point2::new(cursor_pos.0 - 50., cursor_pos.1 + selected_item.sprite_height / 2. - 12.));
        draw(ctx, &self.cursor, param)?;
      }
    }
    Ok(())
  }
}