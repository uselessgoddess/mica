use crate::prelude::*;

mod core;

pub use core::Core;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpawnSet;

pub fn plugin(app: &mut App) {
  app
    // .configure_sets(Update, SpawnSet.run_if(in_state(GameState::Playing)))
    .add_plugins(core::plugin);
}
