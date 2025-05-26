use {
  crate::prelude::*,
  std::{cmp::Ordering, f32::consts::FRAC_2_PI},
};

#[derive(Event, Reflect, Default, Copy, Clone)]
pub struct Target {
  pub entity: Option<Entity>,
  pub target: Vec2,
  pub angle: f32,
  pub len: f32,
}

impl Target {
  pub fn new(entity: Entity, from: Transform2D, to: Vec2) -> Self {
    Self {
      entity: Some(entity),
      target: to,
      angle: from.up().angle_to(to),
      len: from.translation.distance(to),
    }
  }

  pub fn in_place(
    self,
    f: impl FnOnce(EntityWorldMut, Self) + Send + 'static,
  ) -> impl EntityCommand {
    move |entity: Entity, world: &mut World| {
      // skip if target entity exists
      if let Some(Target { entity: Some(entity), .. }) =
        world.entity(entity).get::<Target>().copied()
        && world.get_entity(entity).is_ok()
      {
        return;
      }

      let _ = f(world.entity_mut(entity), self);
    }
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
