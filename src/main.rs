use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{EventHandler, run};
use ggez::conf::{WindowSetup, WindowMode, FullscreenType, NumSamples};
use ggez::timer::sleep;
use std::time::Duration;
use std::path;
mod globals;
mod menu;
mod tilemap;
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
  menu: menu::MenuScreen,
  map: tilemap::Tilemap,
  party: Party,
  battle: Battle,
  transition: Transition
}

impl GameState {
  pub fn new(ctx: &mut Context) -> GameState {
    let mut party = Party::new(ctx);
    let mut menu = data::menus::none_menu(ctx);
    let enemy = data::enemies::test_circle(ctx, 5, (0., 0.), (0, 0));
    let battle = Battle::new(ctx, vec![vec![enemy]], &mut party, &mut menu);
    GameState {
      mode: GameMode::Battle,
      menu,
      map: data::tilemaps::test_room(ctx),
      party,
      battle,
      transition: Transition::new(),
    }
  }
}

impl EventHandler for GameState {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    if self.transition.style == TransitionStyle::None {
      self.battle.update(ctx, &mut self.mode, &mut self.party, &mut self.menu)?;
      self.map.update(ctx, &mut self.mode, &mut self.party, &mut self.battle, &mut self.transition, &mut self.menu)?;
      self.menu.update(ctx, &mut self.mode, &mut self.party, &mut self.battle, &mut self.transition)?;
    } else {
      self.transition.update(&mut self.mode)?;
    }
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx, graphics::BLACK);
    match self.mode {
      GameMode::Battle => {
        self.battle.draw(ctx, &mut self.party)?;
        self.menu.draw(ctx)?;
      },
      GameMode::Map => {
        self.map.draw(ctx)?;
      },
      GameMode::Menu => {
        self.map.draw(ctx)?;
        self.menu.draw(ctx)?;
      }
    }
    if self.transition.style != TransitionStyle::None {
      self.transition.draw(ctx)?;
    }
    sleep(Duration::new(0, 16666666));
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
      width: globals::WINDOW_SIZE.0,
      height: globals::WINDOW_SIZE.1,
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