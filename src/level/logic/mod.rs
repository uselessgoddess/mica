mod life;

use crate::prelude::*;

pub use life::{Damage, Death, Health};

pub fn plugin(app: &mut App) {
  app.add_plugins(life::plugin);
}
