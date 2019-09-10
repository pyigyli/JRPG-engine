use ggez::graphics::{spritebatch, Image, DrawParam, draw};
use ggez::nalgebra::Point2;
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::{Context, GameResult};
use crate::data::menus;
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
use crate::transition::{Transition, TransitionStyle};

pub struct MenuScreen {
  pub open: bool,
  containers: Vec<MenuContainer>,
  pub selectable_items: Vec<Vec<MenuItem>>,
  pub unselectable_items: Vec<MenuItem>,
  cursor: spritebatch::SpriteBatch,
  pub cursor_pos: (usize, usize),
  columns_first: bool,
  input_cooldowns: InputCooldowns,
  return_action: OnClickEvent,
  pub mutation: Option<for<'r> fn(&'r mut MenuScreen) -> GameResult<()>>
}

impl MenuScreen {
  pub fn new(
    ctx: &mut Context,
    open: bool,
    containers: Vec<MenuContainer>,
    selectable_items: Vec<Vec<MenuItem>>,
    unselectable_items: Vec<MenuItem>,
    cursor_pos: (usize, usize),
    columns_first: bool,
    return_action: OnClickEvent
  ) -> MenuScreen {
    MenuScreen {
      open,
      containers,
      selectable_items,
      unselectable_items,
      cursor: spritebatch::SpriteBatch::new(Image::new(ctx, "/cursor.png").unwrap()),
      cursor_pos,
      columns_first,
      input_cooldowns: InputCooldowns::new(),
      return_action,
      mutation: None
    }
  }

  pub fn update(&mut self, ctx: &mut Context, mode: &mut GameMode, party: &mut Party, battle: &mut Battle, transition: &mut Transition) -> GameResult<()> {
    if *mode == GameMode::Map && keyboard::is_key_pressed(ctx, KeyCode::F) {
      *self = menus::menu_main(ctx);
      *mode = GameMode::Menu;
    } else if self.open {
      if keyboard::is_key_pressed(ctx, KeyCode::A) && !self.input_cooldowns.a {
        self.input_cooldowns.a = true;
        match &mut self.selectable_items[self.cursor_pos.0][self.cursor_pos.1].on_click {
          OnClickEvent::ToMenuScreen(new_menu)                                 => *self = new_menu(ctx, mode, party, &battle.enemies),
          OnClickEvent::ToTargetSelection(target_selection, action_parameters) => *self = target_selection(ctx, party, &mut battle.enemies, action_parameters),
          OnClickEvent::ActOnTarget(target, action_parameters) => {
            self.open = false;
            party.battle_turn_action(ctx, battle, *target, action_parameters)?;
          },
          OnClickEvent::MutateMenu(mutation) => self.mutation = Some(*mutation),
          OnClickEvent::Transition(new_mode) => transition.set(TransitionStyle::BlackInFast(new_mode.clone()))?,
          OnClickEvent::None => ()
        }
      } else if !keyboard::is_key_pressed(ctx, KeyCode::A) {
        self.input_cooldowns.a = false;
      }
      if keyboard::is_key_pressed(ctx, KeyCode::S) && !self.input_cooldowns.s {
        self.input_cooldowns.s = true;
        match self.return_action {
          OnClickEvent::ToMenuScreen(new_menu) => {
            *self = new_menu(ctx, mode, party, &battle.enemies);
          },
          _ => ()
        };
      } else if !keyboard::is_key_pressed(ctx, KeyCode::S) {
        self.input_cooldowns.s = false;
      }
      if keyboard::is_key_pressed(ctx, KeyCode::Up) && !self.input_cooldowns.up {
        self.input_cooldowns.up = true;
        if       self.columns_first && self.cursor_pos.1 > 0 {self.cursor_pos.1 -= 1;}
        else if !self.columns_first && self.cursor_pos.1 > 0 {self.cursor_pos = (0, self.cursor_pos.1 - 1);}
        return Ok(());
      } else if !keyboard::is_key_pressed(ctx, KeyCode::Up) {
        self.input_cooldowns.up = false;
      }
      if keyboard::is_key_pressed(ctx, KeyCode::Down) && !self.input_cooldowns.down {
        self.input_cooldowns.down = true;
        if       self.columns_first && self.cursor_pos.1 < self.selectable_items[self.cursor_pos.0].len() - 1 {self.cursor_pos.1 += 1;}
        else if !self.columns_first && self.cursor_pos.1 < self.selectable_items.len() - 1 {self.cursor_pos = (0, self.cursor_pos.1 + 1)}
        return Ok(());
      } else if !keyboard::is_key_pressed(ctx, KeyCode::Down) {
        self.input_cooldowns.down = false;
      }
      if keyboard::is_key_pressed(ctx, KeyCode::Left) && !self.input_cooldowns.left {
        self.input_cooldowns.left = true;
        if       self.columns_first && self.cursor_pos.0 > 0 {self.cursor_pos = (self.cursor_pos.0 - 1, 0);}
        else if !self.columns_first && self.cursor_pos.0 > 0 {self.cursor_pos.0 -= 1;}
        return Ok(());
      } else if !keyboard::is_key_pressed(ctx, KeyCode::Left) {
        self.input_cooldowns.left = false;
      }
      if keyboard::is_key_pressed(ctx, KeyCode::Right) && !self.input_cooldowns.right {
        self.input_cooldowns.right = true;
        if       self.columns_first && self.cursor_pos.0 < self.selectable_items.len() - 1 {self.cursor_pos = (self.cursor_pos.0 + 1, 0)}
        else if !self.columns_first && self.cursor_pos.0 < self.selectable_items[self.cursor_pos.1].len() - 1 {self.cursor_pos.0 += 1;}
        return Ok(());
      } else if !keyboard::is_key_pressed(ctx, KeyCode::Right) {
        self.input_cooldowns.right = false;
      }
      if let Some(mutate) = self.mutation {
        mutate(self)?;
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
        let cursor_pos = self.selectable_items[self.cursor_pos.0][self.cursor_pos.1].screen_pos;
        self.cursor.add(DrawParam::new());
        let param = DrawParam::new()
          .dest(Point2::new(cursor_pos.0 - 50., cursor_pos.1));
        draw(ctx, &self.cursor, param)?;
      }
    }
    Ok(())
  }
}