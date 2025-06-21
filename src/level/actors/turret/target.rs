use {crate::prelude::*, std::cmp::Ordering};

#[derive(Event, Reflect, Default, Copy, Clone)]
pub struct Target {
  pub entity: Option<Entity>,
  pub target: Vec2,
  pub angle: f32,
  pub len: f32,
}

impl Target {
  pub fn new(entity: Entity, from: Transform2D, to: Vec2) -> Self {
    let len = from.translation.distance(to);
    let angle = (to - from.translation)
      .try_normalize()
      .map(|dir| from.up().angle_to(dir))
      .unwrap_or_default();
    Self { entity: Some(entity), target: to, len, angle }
  }
}

impl PartialEq for Target {
  fn eq(&self, other: &Self) -> bool {
    OrderedFloat(self.angle).eq(&OrderedFloat(other.angle))
  }
}

impl Eq for Target {}

impl PartialOrd for Target {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Target {
  fn cmp(&self, other: &Self) -> Ordering {
    OrderedFloat(self.angle.abs()).cmp(&OrderedFloat(other.angle.abs()))
  }
}
