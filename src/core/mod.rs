mod dev;
mod layers;
pub mod noise;
mod system;
pub mod tilemap;
mod transform;

pub use {layers::layer, transform::Transform2D};

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
