mod life;
pub mod pathfinding;
mod projectile;

use crate::prelude::*;

pub use {
  life::{Damage, Death, Health, Lifetime},
  projectile::Projectile,
};

pub fn plugin(app: &mut App) {
  app.register_type::<ChildOf>().add_plugins((
    life::plugin,
    projectile::plugin,
    pathfinding::plugin,
  ));
}

/// Just a joke, rename to `Parent` after bevy 0.16
#[derive(Component, Reflect)]
pub struct ChildOf(pub Entity);
