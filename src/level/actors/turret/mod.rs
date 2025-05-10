mod laser;
mod rocket;

use {
  crate::{level::Enemy, prelude::*},
  std::{cmp::Ordering, collections::BTreeSet},
};

pub use {laser::Laser, rocket::Rocket};

#[derive(Component, Reflect, Default)]
#[require(MonitorTargets)]
pub struct Turret;

pub fn plugin(app: &mut App) {
  register(app)
    .add_plugins(laser::plugin)
    .add_plugins(rocket::plugin)
    .add_systems(PostUpdate, self_monitor)
    .add_systems(Last, slave);
}

fn register(app: &mut App) -> &mut App {
  app
    .register_type::<Turret>()
    .register_type::<Target>()
    .register_type::<MonitorTargets>();
  app
}

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

#[derive(Component, Reflect, Default, Clone, Deref, DerefMut)]
pub struct MonitorTargets(pub BTreeSet<Target>);

impl MonitorTargets {
  pub fn single(target: Target) -> Self {
    Self(BTreeSet::from([target]))
  }
}

/// Monitor targets for turrets if they are not slaves
fn self_monitor(
  mut turrets: Query<
    (Entity, &mut MonitorTargets, &Transform2D),
    (With<Turret>, Without<Slave>),
  >,
  enemies: Query<(Entity, &Transform2D), With<Enemy>>,
) {
  turrets.par_iter_mut().for_each(
    |(_, mut monitor, &Transform2D { translation: a, .. })| {
      let targets: BTreeSet<_> = enemies
        .iter()
        .map(|(entity, &Transform2D { translation: b, .. })| {
          Target::new(entity, a, b)
        })
        .collect();

      monitor.0 = targets;
    },
  );
}

#[derive(Component, Debug, Default)]
pub struct Slave;

fn slave(query: Query<Entity, With<Slave>>, mut commands: Commands) {
  for entity in query.iter() {
    commands.entity(entity).remove::<Slave>();
  }
}
