use crate::prelude::*;

mod actors;
mod logic;
mod prepare;

pub use {actors::*, logic::*, prepare::Occupied};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LevelSet {
  Prepare, // pre-update
}

pub fn plugin(app: &mut App) {
  app
    .configure_sets(
      PreUpdate,
      LevelSet::Prepare.run_if(in_state(GameState::Playing)),
    )
    .add_plugins(actors::plugin)
    .add_plugins(logic::plugin)
    .add_plugins(prepare::plugin);
}
