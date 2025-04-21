use crate::prelude::*;

mod core;
pub mod enemy;
pub mod facility;
pub mod turret;

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
    .add_plugins((
      core::plugin,
      enemy::plugin,
      turret::plugin,
      facility::plugin,
    ));
}
