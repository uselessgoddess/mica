pub mod bar;
mod dev;
pub mod ecs;
mod layers;
mod lens;
pub mod noise;
pub mod physics;
mod system;
pub mod tilemap;
mod transform;
mod utils;

pub use {
  bar::{Bar, BarHeight, Percentage},
  ecs::{
    Affect, AppExt as _, Sensor, TriggerExt as _, spawn::CommandsExt as _,
  },
  layers::layer,
  lens::{TextLens, decryption_animation, typing_animation},
  transform::Transform2D,
  utils::type_name,
};

use crate::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins((
        system::plugin,
        tilemap::plugin,
        transform::plugin, // todo!: move to `ecs` mod
        ecs::plugin,
        lens::plugin,
      ))
      .add_plugins(PhysicsPlugins::default());

    app.insert_resource(Gravity::ZERO);

    if debug::dev() {
      app.add_plugins(dev::plugin);
    }
  }
}

#[derive(Component)]
#[require(Camera2d)]
pub struct PrimaryCamera;
