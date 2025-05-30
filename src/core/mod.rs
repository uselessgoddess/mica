pub mod bar;
mod dev;
pub mod ecs;
mod layers;
mod lens;
pub mod noise;
pub mod palette;
pub mod physics;
mod shapes;
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
  shapes::Shapes,
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
        shapes::plugin,
      ))
      .add_plugins(PhysicsPlugins::default());

    app.insert_resource(SubstepCount(1));
    app.insert_resource(Gravity::ZERO);

    if debug::dev() {
      app.add_plugins(dev::plugin);
    }
  }
}

#[derive(Component)]
#[require(Camera2d)]
pub struct PrimaryCamera;

pub mod timer {
  use bevy::time::{Timer, TimerMode};

  pub fn repeat(secs: f32) -> Timer {
    Timer::from_seconds(secs, TimerMode::Repeating)
  }
}

pub trait TweenableExt {
  fn is_total_completed(&self) -> bool;
}

impl<T> TweenableExt for &dyn Tweenable<T> {
  fn is_total_completed(&self) -> bool {
    if let TotalDuration::Finite(duration) = self.total_duration()
      && self.elapsed() >= duration
    {
      true
    } else {
      false
    }
  }
}
