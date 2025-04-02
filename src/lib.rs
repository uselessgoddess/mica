#![feature(iter_map_windows)]

pub mod camera;
pub mod core;
mod debug;
pub mod level;
pub mod sync;

use {bevy::prelude::*, core::CorePlugin};

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
  #[default]
  Loading,
  Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(CorePlugin)
        // .init_state::<GameState>()
    ;
  }
}

#[allow(ambiguous_glob_reexports, unused_imports)]
pub mod prelude {
  pub use super::*;

  pub use {bevy::prelude::*, ecs_tilemap::prelude::*};
}
