use crate::prelude::*;

pub fn layer<const N: usize>() -> Transform2D {
  Transform2D::IDENTITY.with_layer(N as f32)
}
