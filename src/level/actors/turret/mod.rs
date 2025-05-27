mod follow;
pub mod laser;
pub mod rocket;
mod target;

use {
  crate::{level::Enemy, prelude::*},
  std::mem,
};

pub use {follow::FollowTarget, laser::Laser, rocket::Rocket, target::Target};

#[derive(Reflect, Debug, Default, Copy, Clone)]
pub enum TargetKind {
  #[default]
  Angle,
  Distance,
}

#[derive(Component, Reflect, Default)]
#[require(MonitorTargets, Cooldown)]
pub struct Turret {
  target: TargetKind,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TurretSet {
  /// Monitor enemies to prepare targets
  ///
  /// ```
  /// # use bevy::prelude::*;
  /// # use mica::level::turret::{MonitorTargets, TurretSet};
  /// #
  /// # let mut app = App::new();
  /// # app.add_plugins(DefaultPlugins);
  ///
  /// app.add_systems(PostUpdate, custom.after(TurretSet::Monitor));
  ///
  /// fn custom(query: Query<&mut MonitorTargets>) {
  ///   // ...
  /// }
  /// ```
  Monitor,
  Target,
}

pub fn plugin(app: &mut App) {
  register(app)
    .add_plugins((laser::plugin, rocket::plugin, follow::plugin))
    .add_systems(Update, (target, targets).chain().in_set(TurretSet::Target))
    .add_systems(PostUpdate, self_monitor.in_set(TurretSet::Monitor))
    .add_systems(Last, slave)
    .add_systems(Update, fov_gizmos.run_if(in_debug(D::L2)));
}

fn register(app: &mut App) -> &mut App {
  app
    .register_type::<Fov>()
    .register_type::<Target>()
    .register_type::<Turret>()
    .register_type::<Cooldown>()
    .register_type::<MonitorTargets>();
  app
}

#[derive(Component, Reflect, Default, Clone, Deref, DerefMut)]
pub struct MonitorTargets(pub Vec<Entity>);

impl MonitorTargets {
  pub fn filter_by(&mut self, mut filter: impl FnMut(Entity) -> bool) {
    self.0 =
      mem::take(&mut self.0).into_iter().filter(|&e| filter(e)).collect();
  }
}

/// Monitor targets for turrets if they are not slaves
fn self_monitor(
  mut turrets: Query<
    (&mut MonitorTargets, &Turret, &Transform2D),
    Without<Slave>,
  >,
  enemies: Query<(Entity, &Transform2D), With<Enemy>>,
) {
  turrets.par_iter_mut().for_each(|(mut monitor, turret, &from)| {
    let targets: Vec<_> = enemies
      .iter()
      .sort_by_key::<(Entity, &Transform2D), _>(|&(entity, to)| {
        let target = Target::new(entity, from, to.translation);
        let key = match turret.target {
          TargetKind::Angle => target.angle.abs(),
          TargetKind::Distance => target.len,
        };
        OrderedFloat(key)
      })
      .map(|(entity, _)| entity)
      .collect();

    monitor.0 = targets;
  });
}

fn target(
  mut turrets: Query<(&mut Target, &Transform2D)>,
  targets: Query<&Transform2D>,
) {
  for (mut target, &from) in turrets.iter_mut() {
    if let Some(entity) = target.entity
      && let Ok(to) = targets.get(entity)
    {
      *target = Target::new(entity, from, to.translation);
    }
  }
}

fn targets(
  turrets: Query<(Entity, &Transform2D, &MonitorTargets, Option<&Target>)>,
  world: Query<&Transform2D>,
  mut commands: Commands,
) {
  for (entity, &from, monitor, target) in turrets.iter() {
    // skip if target entity exists
    if let Some(Target { entity: Some(entity), .. }) = target.copied()
      && world.get(entity).is_ok()
    {
      continue;
    }

    commands.entity(entity).remove::<Target>();

    if let Some(target) = monitor.first().copied()
      && let Ok(to) = world.get(target)
    {
      commands.entity(entity).insert(Target::new(target, from, to.translation));
    }
  }
}

/// Mark turret that slaved by some facility for single update
#[derive(Component, Debug, Default)]
pub struct Slave;

fn slave(query: Query<Entity, With<Slave>>, mut commands: Commands) {
  for entity in query.iter() {
    commands.entity(entity).remove::<Slave>();
  }
}

#[derive(Component, Reflect, Debug, Default)]
#[non_exhaustive]
pub enum Cooldown {
  #[default]
  Allow,
  Forbid,
}

impl Cooldown {
  pub fn allow(&self) -> bool {
    matches!(self, Cooldown::Allow)
  }
}

#[derive(Component, Reflect, Debug, Copy, Clone)]
pub struct Fov {
  /// Angle in degrees
  pub angle: f32,
}

impl Fov {
  pub fn new(angle: f32) -> Fov {
    Self { angle }
  }

  pub fn bounds(&self, ray: Vec2) -> (Vec2, Vec2) {
    // FIXME: maybe store `Rot2` in `Fov`
    (
      Rot2::degrees(-self.angle / 2.0) * ray,
      Rot2::degrees(self.angle / 2.0) * ray,
    )
  }

  pub fn in_fov(&self, transform: Transform2D, target: Vec2) -> bool {
    let dir = (target - transform.translation).normalize();

    // must be `/ 2.0` because it's a whole angle of fov
    (self.angle / 2.0).to_radians().cos() < transform.up().dot(dir)
  }
}

impl Default for Fov {
  fn default() -> Self {
    Self::new(10.0)
  }
}

pub fn fov_gizmos(query: Query<(&Transform2D, &Fov)>, mut commands: Commands) {
  for (target @ &Transform2D { translation, .. }, fov) in query.iter() {
    let mut gizmos = Shapes(&mut commands);
    let (left, right) = fov.bounds(target.up());

    // FIXME: maybe add `tile(10.0)` like function?
    gizmos
      .triangle(
        translation,
        translation + left * (tilemap::TILE * 12.5),
        translation + right * (tilemap::TILE * 12.5),
      )
      .fill(Fill {
        color: Color::srgba(0.0, 0.0, 0.0, 0.25),
        options: FillOptions::DEFAULT,
      })
      .width(0.25)
      .build();
  }
}
