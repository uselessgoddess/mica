use crate::prelude::*;

pub fn in_fov(transform: Transform2D, target: Vec2, angle: f32) -> bool {
  let dir = (target - transform.translation).normalize();

  // must be `/ 2.0` because it's whole angle of fov
  (angle / 2.0).to_radians().cos() < transform.up().dot(dir)
}
