pub mod sensor;

use crate::prelude::*;

pub use sensor::{Effect, Sensor, effect};

pub fn plugin(app: &mut App) {
  app.add_plugins(sensor::plugin);
}
