pub mod laser;
pub mod rocket;
mod target;
pub mod utils;

use {
  crate::{level::Enemy, prelude::*},
  std::{collections::BTreeSet, mem, ops::Deref},
};

pub use {laser::Laser, rocket::Rocket, target::Target};

#[derive(Component, Reflect, Default)]
#[require(MonitorTargets, Cooldown)]
pub struct Turret;

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
}

pub fn plugin(app: &mut App) {
  register(app)
    .add_plugins(laser::plugin)
    .add_plugins(rocket::plugin)
    .add_systems(
      PostUpdate,
      (self_monitor, filter_monitor).chain().in_set(TurretSet::Monitor),
    )
    .add_systems(Last, slave)
    .add_systems(Update, fov_gizmos.run_if(in_debug(D::L2)));
}

fn register(app: &mut App) -> &mut App {
  app
    .register_type::<Fov>()
    .register_type::<Turret>()
    .register_type::<Cooldown>()
    .register_type::<MonitorTargets>();
  app
}

#[derive(Component, Reflect, Default, Clone, Deref, DerefMut)]
pub struct MonitorTargets(pub BTreeSet<Target>);

impl MonitorTargets {
  pub fn single(target: Target) -> Self {
    Self(BTreeSet::from([target]))
  }

  pub fn filter_by(&mut self, filter: impl FnMut(&Target) -> bool) {
    self.0 = mem::take(&mut self.0).into_iter().filter(filter).collect();
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

fn filter_monitor(
  mut turrets: Query<(&Transform2D, &Fov, &mut MonitorTargets)>,
) {
  for (&transform, fov, mut monitor) in turrets.iter_mut() {
    monitor.filter_by(|&Target { target, .. }| {
      utils::in_fov(transform, target, fov.angle)
    });
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
