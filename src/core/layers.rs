use crate::prelude::*;

pub fn layer<const N: usize>() -> Transform {
  Transform::from_xyz(0.0, 0.0, N as f32)
}
