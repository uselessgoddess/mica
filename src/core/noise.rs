use {
  bevy::math::{Vec2, Vec3},
  noise::{NoiseFn, Perlin},
};

pub fn perlin_1d(x: f32) -> f32 {
  Perlin::default().get([x as f64]) as f32
}

pub fn perlin_2d(xy: Vec2) -> f32 {
  Perlin::default().get([xy.x as f64, xy.y as f64]) as f32
}

pub fn perlin_3d(xyz: Vec3) -> f32 {
  Perlin::default().get([xyz.x as f64, xyz.y as f64, xyz.z as f64]) as f32
}
