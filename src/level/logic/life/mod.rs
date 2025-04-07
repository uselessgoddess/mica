use crate::prelude::*;

mod health;

pub use health::Health;

pub fn plugin(app: &mut App) {
  app
    .add_event::<OnDeath>()
    .add_event::<DamageEvent>()
    .add_plugins(health::plugin)
    .add_systems(Update, (damage, death));
}

#[derive(Event, Debug, Copy, Clone)]
pub struct OnDeath(pub Entity);

#[derive(Event, Debug, Copy, Clone)]
pub struct DamageEvent {
  pub entity: Entity,
  pub damage: f32,
}

fn damage(
  mut query: Query<&mut Bar<Health>>,
  mut damage: EventReader<DamageEvent>,
) {
  for &DamageEvent { entity, damage } in damage.read() {
    if let Ok(mut health) = query.get_mut(entity) {
      health.dec(damage);
    }
  }
}

fn death(
  query: Query<(Entity, &Bar<Health>)>,
  mut damage: EventWriter<OnDeath>,
) {
  for (entity, health) in query.iter() {
    if health.is_empty() {
      damage.send(OnDeath(entity));
    }
  }
}
