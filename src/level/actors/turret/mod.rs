mod laser;

use {
  crate::{level::Enemy, prelude::*},
  std::collections::BTreeSet,
};

pub use laser::Laser;

#[derive(Component, Reflect, Default)]
#[require(MonitorTargets)]
pub struct Turret;

pub fn plugin(app: &mut App) {
  app
    .register_type::<Turret>()
    .add_plugins(laser::plugin)
    .add_systems(Update, prepare_targets);
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct MonitorTargets(pub BTreeSet<(OrderedFloat<f32>, Entity)>);

// todo!: maybe better to use exclusive system in the future
fn prepare_targets(
  mut turrets: Query<(Entity, &Transform2D, &mut MonitorTargets), With<Turret>>,
  enemies: Query<(Entity, &Transform2D), With<Enemy>>,
) {
  for (_, a, mut monitor) in turrets.iter_mut() {
    monitor.clear();
    monitor.extend(enemies.iter().map(|(entity, b)| {
      (OrderedFloat((a.translation - b.translation).length()), entity)
    }));
  }
}
