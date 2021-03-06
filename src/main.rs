use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{EventHandler, run};
use ggez::conf::{WindowSetup, WindowMode, FullscreenType, NumSamples};
use ggez::timer::sleep;
use std::time::Duration;
use std::path;
mod globals;
use globals::WINDOW_SIZE;
mod menu;
use menu::MenuScreen;
mod tilemap;
use tilemap::Tilemap;
mod data;
mod battle;
use battle::Battle;
mod party;
use party::Party;
mod transition;
use transition::{Transition, TransitionStyle};

#[derive(Clone)]
pub enum GameMode {
  Map, Menu, Battle
}

impl PartialEq for GameMode {
  fn eq(&self, other: &Self) -> bool {
    match self {
      GameMode::Map    => {match other {GameMode::Map    => true, _ => false}},
      GameMode::Menu   => {match other {GameMode::Menu   => true, _ => false}},
      GameMode::Battle => {match other {GameMode::Battle => true, _ => false}}
    }
  }
}

struct GameState {
  mode: GameMode,
  menu: MenuScreen,
  map: Tilemap,
  party: Party,
  battle: Battle,
  battle_menu: MenuScreen,
  transition: Transition
}

impl GameState {
  pub fn new(ctx: &mut Context) -> GameState {
    let mut party = Party::new(ctx);
    let mut menu = data::menus::none_menu(ctx);
    let battle = Battle::new(ctx, vec![Vec::new()], &mut party, &mut menu);
    GameState {
      mode: GameMode::Map,
      menu,
      map: data::tilemaps::test_room(ctx),
      party,
      battle,
      battle_menu: data::menus::none_menu(ctx),
      transition: Transition::new(),
    }
  }
}

impl EventHandler for GameState {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    if self.transition.style == TransitionStyle::None {
      self.battle.update(ctx, &mut self.mode, &mut self.party, &mut self.menu, &mut self.battle_menu, &mut self.transition)?;
      self.map.update(ctx, &mut self.mode, &mut self.party, &mut self.battle, &mut self.menu, &mut self.transition)?;
      self.menu.update(ctx, &mut self.mode, &mut self.party, &mut self.battle, &mut self.transition)?;
    } else {
      self.transition.update(ctx, &mut self.mode, &mut self.menu, &mut self.party, &self.battle.enemies)?;
    }
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx, graphics::BLACK);
    match self.mode {
      GameMode::Battle => self.battle.draw(ctx, &mut self.party, &mut self.battle_menu)?,
      GameMode::Map    => self.map.draw(ctx)?,
      GameMode::Menu   => self.menu.draw(ctx)?
    }
    if self.transition.style != TransitionStyle::None {
      self.transition.draw(ctx)?;
    }
    sleep(Duration::new(0, 1));
    graphics::present(ctx)
  }
}

fn main() {
  let (mut ctx, mut event_loop) = ContextBuilder::new("game_name", "author_name")
    .add_resource_path(path::PathBuf::from("./resources"))
    .window_setup(WindowSetup {
      title: "THE ULTIMATE GAMING EXPERIENCE".to_owned(),
      samples: NumSamples::Zero,
      vsync: true,
      icon: "".to_owned(),
      srgb: true
    })
    .window_mode(WindowMode {
      width: WINDOW_SIZE.0,
      height: WINDOW_SIZE.1,
      maximized: false,
      fullscreen_type: FullscreenType::Windowed,
      borderless: false,
      min_width: 0.,
      max_width: 0.,
      min_height: 0.,
      max_height: 0.,
      resizable: false
    })
    .build()
    .unwrap();
  let mut game = GameState::new(&mut ctx);
  match run(&mut ctx, &mut event_loop, &mut game) {
    Ok(_) => println!("Finished"),
    Err(e) => println!("Error:\n{}", e)
  }
}