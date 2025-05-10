use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app.add_plugins(TilemapPlugin);
}

pub const SIZE: TilemapSize = TilemapSize { x: 31, y: 31 };
pub const TILE: f32 = 32.0;

pub const fn center() -> TilePos {
  TilePos { x: SIZE.x / 2, y: SIZE.y / 2 }
}

pub fn random() -> TilePos {
  TilePos { x: rand::random_range(0..SIZE.x), y: rand::random_range(0..SIZE.y) }
}

#[rustfmt::skip]
pub fn sample_border() -> TilePos {
  let TilePos { x, y } = random();
  match rand::random_range(0.0..1.0) {
        ..0.25 => TilePos { x, y: 0 },
    0.25..0.50 => TilePos { x, y: SIZE.y - 1 },
    0.50..0.75 => TilePos { x: 0, y },
    0.75..        => TilePos { x: SIZE.x - 1, y },
    _ => unreachable!(),
  }
}
