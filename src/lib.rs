#![feature(iter_map_windows, let_chains)]
#![allow(
  irrefutable_let_patterns,
  reason = "Because that is favorite my secret trick"
)]

pub mod core;
pub mod debug;
pub mod level;
pub mod sync;
pub mod ui;

use bevy::prelude::*;

/// The game states
#[derive(States, Debug, Default, Hash, PartialEq, Eq, Clone)]
pub enum Game {
  #[default]
  Gameplay,
}

/// The game screen states
#[derive(States, Debug, Default, Hash, PartialEq, Eq, Clone)]
pub enum Pause {
  #[default]
  None,
  Pause,
}

pub struct GamePlugin;

#[allow(ambiguous_glob_reexports, unused_imports)]
pub mod prelude {
  pub use super::*;

  pub use {
    super::core::*,
    avian2d::prelude::*,
    bevy::prelude::*,
    debug::{AppExt, D, in_debug},
    ecs_tilemap::prelude::*,
    hanabi::prelude::*,
    lunex::*,
    num_traits as num,
    ordered_float::OrderedFloat,
    pancam::*,
    tweening::*,
  };

  impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
      app
        .add_plugins(CorePlugin)
        .init_state::<Game>()
        .enable_state_scoped_entities::<Game>()
        .init_state::<Pause>()
        .enable_state_scoped_entities::<Pause>()
        .add_plugins((
          PanCamPlugin,
          HanabiPlugin,
          TweeningPlugin,
          UiLunexPlugins,
        ))
        .add_plugins((sync::plugin, level::plugin, ui::plugin));
    }
  }
}
