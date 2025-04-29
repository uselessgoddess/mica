use crate::prelude::*;

pub const ALL: LayerMask = LayerMask::ALL;
pub const NONE: LayerMask = LayerMask::NONE;
pub const DEFAULT: LayerMask = LayerMask::DEFAULT;
//
pub const ENV: LayerMask = LayerMask(1 << 1);
pub const PROJ: LayerMask = LayerMask(1 << 2);
pub const ENEMY: LayerMask = LayerMask(1 << 3);

pub fn projectile() -> CollisionLayers {
  CollisionLayers::new(PROJ, ENEMY)
}
