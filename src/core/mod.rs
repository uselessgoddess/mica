pub mod bar;
mod dev;
pub mod ecs;
mod layers;
pub mod noise;
mod system;
pub mod tilemap;
mod transform;
mod utils;

pub use {
  bar::{Bar, BarHeight, Percentage},
  ecs::{Effect, Sensor},
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
      .add_plugins(transform::plugin) // todo!: move to `ecs` mod
      .add_plugins(ecs::plugin)
      .add_plugins(PhysicsPlugins::default());

    if debug::dev() {
      app.add_plugins(dev::plugin);
    }
  }
}
