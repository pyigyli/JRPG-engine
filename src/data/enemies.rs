use ggez::{Context, GameResult};
use ggez::timer::ticks;
use rand::{Rng, thread_rng};
use crate::battle::action::{ActionParameters, DamageType};
use crate::battle::enemy::{Enemy, Animation};
use crate::party::Party;
use crate::menu::notification::Notification;

fn turn_action(ctx: &mut Context, enemy: &mut Enemy, party: &mut Party, notification: &mut Option<Notification>) -> GameResult<()> {
  let mut rng = thread_rng();
  if rng.gen::<f32>() < 0.75 {
    *notification = Some(Notification::new(ctx, format!("{} attacks", enemy.name)));
    let mut party_targets = Vec::new();
    if party.first .name.len() > 0 {party_targets.push(0)}
    if party.second.name.len() > 0 {party_targets.push(1)}
    if party.third .name.len() > 0 {party_targets.push(2)}
    if party.fourth.name.len() > 0 {party_targets.push(3)}
    let target_number;
    let target_rng = rng.gen::<f32>();
    if target_rng <= 1. / party_targets.len() as f32 {
      target_number = party_targets[0];
    } else if target_rng <= 2. / party_targets.len() as f32 {
      target_number = party_targets[1];
    } else if target_rng <= 3. / party_targets.len() as f32 {
      target_number = party_targets[2];
    } else {
      target_number = party_targets[3];
    }
    let action_parameters = ActionParameters::new(DamageType::Physical, enemy.state.attack, 0., false, 0., false, 0., false);
    enemy.animation = (Animation::StartTurn(target_number, action_parameters), 60, ticks(ctx));
  } else {
    *notification = Some(Notification::new(ctx, format!("{} sits still", enemy.name)));
    enemy.animation = (Animation::EndTurn, 30, ticks(ctx));
  }
  Ok(())
}

pub fn test_triangle(ctx: &mut Context, id: u8, screen_pos: (f32, f32), selection_pos: (usize, usize)) -> Enemy {
  Enemy::new(ctx, id, "/enemies/test-triangle.png".to_owned(), screen_pos, selection_pos, "Triangle".to_owned(),  3, 16, 5, 6, 2, 2, 2, 2, 15, turn_action)
}

pub fn test_circle(ctx: &mut Context, id: u8, screen_pos: (f32, f32), selection_pos: (usize, usize)) -> Enemy {
  Enemy::new(ctx, id, "/enemies/test-circle.png".to_owned()  , screen_pos, selection_pos, "Circle".to_owned(),    2, 12, 6, 4, 1, 1, 2, 3, 7, turn_action)
}

pub fn test_square(ctx: &mut Context, id: u8, screen_pos: (f32, f32), selection_pos: (usize, usize)) -> Enemy {
  Enemy::new(ctx, id, "/enemies/test-square.png".to_owned()  , screen_pos, selection_pos, "Square".to_owned(),    2, 12, 6, 4, 2, 2, 2, 3, 8, turn_action)
}