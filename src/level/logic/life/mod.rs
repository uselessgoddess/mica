use crate::prelude::*;

mod health;

pub use health::Health;

pub fn plugin(app: &mut App) {
  app
    .add_effect::<Death>()
    .add_effect::<Damage>()
    .add_plugins(health::plugin)
    .add_systems(Update, (damage, death));
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Death;

#[derive(Debug, Default, Copy, Clone)]
pub struct Damage(pub f32);

// todo!: implement derive-macro
impl Effect for Death {}
impl Effect for Damage {}

fn damage(
  mut query: Query<&mut Bar<Health>>,
  mut damage: EventReader<Affect<Damage>>,
) {
  for &Affect { entity, effect: Damage(damage), .. } in damage.read() {
    if let Ok(mut health) = query.get_mut(entity) {
      health.dec(damage);
    }
  }
}

fn death(
  query: Query<(Entity, &Bar<Health>)>,
  mut damage: EventWriter<Affect<Death>>,
) {
  for (entity, health) in query.iter() {
    if health.is_empty() {
      damage.send(Affect::new(entity));
    }
  }
}
