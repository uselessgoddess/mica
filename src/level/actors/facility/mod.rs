mod designator;

use crate::prelude::*;

pub use designator::Designator;

pub fn plugin(app: &mut App) {
  app.add_plugins(designator::plugin);
}
