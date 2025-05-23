use {crate::prelude::*, std::cmp::Ordering};

#[derive(Event, Reflect, Default, Copy, Clone)]
pub struct Target {
  pub entity: Option<Entity>,
  pub target: Vec2,
  pub len: f32,
}

impl PartialEq for Target {
  fn eq(&self, other: &Self) -> bool {
    OrderedFloat(self.len).eq(&OrderedFloat(other.len))
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
    OrderedFloat(self.len).cmp(&OrderedFloat(other.len))
  }
}

impl Target {
  pub fn new(entity: Entity, from: Vec2, to: Vec2) -> Self {
    Self { entity: Some(entity), target: to, len: from.distance(to) }
  }
}
