pub mod bar;
mod dev;
mod layers;
pub mod noise;
mod system;
pub mod tilemap;
mod transform;
mod utils;

pub use {
  bar::{BarHeight, Bar, Percentage},
  layers::layer,
  transform::Transform2D,
  utils::type_name,
};

use crate::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(system::plugin)
      .add_plugins(tilemap::plugin)
      .add_plugins(transform::plugin);

    if debug::dev() {
      app.add_plugins(dev::plugin);
    }
  }
}
