mod follow;
mod life;
mod projectile;

use crate::prelude::*;

pub use {
  follow::Follow,
  life::{Damage, Death, Health, Lifetime},
  projectile::Projectile,
};

pub fn plugin(app: &mut App) {
  app.register_type::<ChildOf>().add_plugins((
    life::plugin,
    projectile::plugin,
    follow::plugin,
  ));
}

/// Just a joke, rename to `Parent` after bevy 0.16
#[derive(Component, Reflect)]
pub struct ChildOf(pub Entity);
