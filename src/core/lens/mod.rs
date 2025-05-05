mod text;

use crate::prelude::*;

pub use text::*;

pub fn plugin(app: &mut App) {
  app.add_plugins(text::plugin);
}
