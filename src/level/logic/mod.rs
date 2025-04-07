mod life;

use crate::prelude::*;

pub use life::{DamageEvent, Health, OnDeath};

pub fn plugin(app: &mut App) {
  app.add_plugins(life::plugin);
}
