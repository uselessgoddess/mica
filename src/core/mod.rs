mod dev;
mod layers;
pub mod noise;
mod system;
pub mod tilemap;

pub use layers::layer;

use crate::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(system::plugin).add_plugins(tilemap::plugin);

    if debug::dev() {
      app.add_plugins(dev::plugin);
    }
  }
}
