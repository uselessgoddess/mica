use crate::prelude::*;

mod health;
mod period;

pub use {health::Health, period::Lifetime};

pub fn plugin(app: &mut App) {
  app
    .add_effect::<Death>()
    .add_effect::<Damage>()
    .add_plugins(health::plugin)
    .add_plugins(period::plugin)
    .add_systems(Update, death)
    .add_observer(damage);
}

#[derive(Event, Debug, Default, Copy, Clone)]
pub struct Death;

#[derive(Event, Debug, Default, Copy, Clone)]
pub struct Damage(pub f32);

fn damage(trigger: Trigger<Damage>, mut query: Query<&mut Bar<Health>>) {
  let (entity, Damage(damage)) = trigger.read_event();

  if let Ok(mut health) = query.get_mut(entity) {
    health.dec(damage);
  }
}

fn death(query: Query<(Entity, &Bar<Health>)>, mut commands: Commands) {
  for (entity, health) in query.iter() {
    if health.is_empty() {
      commands.entity(entity).trigger(Death);
    }
  }
}
