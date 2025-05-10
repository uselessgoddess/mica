use crate::prelude::*;

mod actors;
mod follow;
mod interact;
mod logic;
mod prepare;

pub use {
  actors::*,
  follow::{Follow, FollowMouse},
  interact::*,
  logic::*,
  prepare::Occupied,
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LevelSet {
  Prepare, // pre-update
}

pub fn plugin(app: &mut App) {
  app
    .configure_sets(
      PreUpdate,
      LevelSet::Prepare.after(sync::SyncSet).run_if(in_state(Game::Gameplay)),
    )
    .add_plugins((
      actors::plugin,
      logic::plugin,
      prepare::plugin,
      follow::plugin,
      interact::plugin,
    ))
    .add_systems(Update, settings);
}

fn settings(
  input: Res<ButtonInput<KeyCode>>,
  state: Res<State<Pause>>,
  mut next: ResMut<NextState<Pause>>,
) {
  if input.just_pressed(KeyCode::Escape) {
    if let Pause::Pause = state.get() {
      next.set(Pause::None)
    } else {
      next.set(Pause::Pause)
    }
  }
}
