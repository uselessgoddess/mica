mod life;
mod projectile;

use crate::prelude::*;

pub use {
  life::{Damage, Death, Health, Period},
  projectile::Projectile,
};

pub fn plugin(app: &mut App) {
  app.add_plugins(life::plugin).add_plugins(projectile::plugin);
}
