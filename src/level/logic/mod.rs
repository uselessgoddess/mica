pub mod effects;
mod life;

use crate::prelude::*;

pub use life::{DamageEvent, DeathEvent, Health};

pub fn plugin(app: &mut App) {
  app.add_plugins(life::plugin).add_plugins(effects::plugin);
}
