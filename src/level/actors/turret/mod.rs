mod laser;

use crate::prelude::*;

pub use laser::Laser;

pub fn plugin(app: &mut App) {
  app.add_plugins(laser::plugin);
}
