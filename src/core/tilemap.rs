use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app.add_plugins(TilemapPlugin);
}

pub const SIZE: TilemapSize = TilemapSize { x: 31, y: 31 };

pub fn random() -> TilePos {
  TilePos { x: rand::random_range(0..SIZE.x), y: rand::random_range(0..SIZE.y) }
}
