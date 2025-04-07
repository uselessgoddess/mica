use crate::prelude::*;

mod core;
pub mod enemy;
mod turret;

pub use {
  core::Core,
  enemy::{Enemy, Wall},
  turret::Turret,
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpawnSet;

pub fn plugin(app: &mut App) {
  app
    .configure_sets(Update, SpawnSet.run_if(in_state(GameState::Playing)))
    .add_plugins(core::plugin)
    .add_plugins(enemy::plugin)
    .add_plugins(turret::plugin);
}
