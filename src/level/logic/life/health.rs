use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app.add_plugins(bar::plugin::<Health>);
}

#[derive(Reflect)]
pub struct Health;

impl Percentage for Health {
  type Item = f32;

  fn value(value: f32, limit: f32) -> f32 {
    value / limit
  }
}
