use ggez::Context;
use rand::{Rng, thread_rng};
use crate::battle::enemy::Enemy;
use crate::data::enemies;

pub fn test_room(ctx: &mut Context) -> Vec<Vec<Enemy>> {
  let rng = thread_rng().gen::<f32>();
  if rng < 0.9 {
    vec![
      vec![enemies::test_triangle(ctx, 5, (0., 0.), (0, 0)), enemies::test_triangle(ctx, 6, (0., 2.), (0, 1))]
      // vec![enemies::test_circle(ctx, 6, (2., 0.), (1, 0)), enemies::test_square(ctx, 7, (2., 1.), (1, 1))]
    ]
  } else if rng < 0.666666666666667 {
    vec![
      vec![enemies::test_circle(ctx, 5, (0., 0.), (0, 0)), enemies::test_square(ctx, 6, (0., 1.), (0, 1))],
      vec![enemies::test_triangle(ctx, 7, (1., 0.), (1, 0))]
    ]
  } else {
    vec![
      vec![enemies::test_circle(ctx, 5 , (0., 0.), (0, 0)), enemies::test_square(ctx, 6 , (0., 1.), (0, 1)), enemies::test_circle(ctx, 7 , (0., 2.), (0, 2))],
      vec![enemies::test_square(ctx, 8 , (1., 0.), (1, 0)), enemies::test_circle(ctx, 9 , (1., 1.), (1, 1)), enemies::test_square(ctx, 10, (1., 2.), (1, 2))],
      vec![enemies::test_circle(ctx, 11, (2., 0.), (2, 0)), enemies::test_square(ctx, 12, (2., 1.), (2, 1)), enemies::test_circle(ctx, 13, (2., 2.), (2, 2))]
    ]
  }
}