use crate::prelude::*;

mod core;
pub mod enemy;

pub use {
  core::Core,
  enemy::{Enemy, Wall},
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpawnSet;

pub fn plugin(app: &mut App) {
  app
    .configure_sets(Update, SpawnSet.run_if(in_state(GameState::Playing)))
    .add_plugins(core::plugin)
    .add_plugins(enemy::plugin);
}
