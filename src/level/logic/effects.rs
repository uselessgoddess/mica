use crate::{
  level::{DamageEvent, DeathEvent},
  prelude::*,
};

pub fn plugin(app: &mut App) {
  app.add_plugins(ecs::effect::<Damage>).add_plugins(ecs::effect::<Death>);
}

#[derive(Default, Clone)]
pub struct Damage(pub f32);

impl Effect for Damage {
  type Event = DamageEvent;

  fn affect(&self, (_, entity): (Entity, Entity)) -> Self::Event {
    DamageEvent { entity, damage: self.0 }
  }
}

#[derive(Default, Clone)]
pub struct Death;

impl Effect for Death {
  type Event = DeathEvent;

  fn affect(&self, (_, entity): (Entity, Entity)) -> Self::Event {
    DeathEvent(entity)
  }
}
