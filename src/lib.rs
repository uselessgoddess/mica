#![feature(iter_map_windows, let_chains)]

pub mod core;
pub mod debug;
pub mod level;
pub mod sync;

use {bevy::prelude::*, core::CorePlugin, pancam::PanCamPlugin};

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
  #[default]
  Loading,
  Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(CorePlugin)
      .init_state::<GameState>()
      .add_plugins(PanCamPlugin)
      .add_plugins(sync::plugin)
      .add_plugins(level::plugin);
  }
}

#[allow(ambiguous_glob_reexports, unused_imports)]
pub mod prelude {
  pub use super::*;

  pub use {
    super::core::*,
    avian2d::prelude::{Sensor as ColliderSensor, *},
    bevy::prelude::*,
    debug::{AppExt, D, in_debug},
    ecs_tilemap::prelude::*,
    num_traits as num,
    pancam::*,
  };
}
