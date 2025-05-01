mod effects;
mod explosion;
mod missile;
mod thrust;

use crate::prelude::*;

use crate::level::{
  Enemy,
  turret::{MonitorTargets, Target},
};

use {
  explosion::Explosion,
  missile::{Fuse, Missile, MissileMetadata},
  thrust::Thrust,
};

pub fn plugin(app: &mut App) {
  app
    .register_type::<Rocket>()
    .add_plugins((missile::plugin, explosion::plugin, thrust::plugin))
    .add_systems(Update, (spawn, attack));
}

#[derive(Component, Reflect)]
#[require(super::Turret)]
pub struct Rocket {
  pub cooldown: Timer,
}

impl Default for Rocket {
  fn default() -> Self {
    Self { cooldown: Timer::from_seconds(2.0, TimerMode::Repeating) }
  }
}

fn spawn(
  query: Query<Entity, Added<Rocket>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  for entity in query.iter() {
    let mesh = meshes.add(Circle::new(tilemap::TILE * 0.25));
    let material = materials.add(Color::srgb(0.25, 0.15, 0.25));
    commands.entity(entity).insert((Mesh2d(mesh), MeshMaterial2d(material)));
  }
}

fn attack(
  mut turrets: Query<(&Transform2D, &MonitorTargets, &mut Rocket)>,
  enemies: Query<&Transform2D, With<Enemy>>,
  mut commands: Commands,
  time: Res<Time>,
) {
  for (&transform, monitor, mut rocket) in turrets.iter_mut() {
    if let Some(Target { entity: Some(target), .. }) = monitor.first().copied()
      && let Ok(enemy) = enemies.get(target).copied()
      && rocket.cooldown.tick(time.delta()).just_finished()
    {
      commands.spawn((
        Name::new("Missile"),
        transform.add_layer(1.0),
        (
          Missile { target: enemy.translation, radius: tilemap::TILE },
          Thrust { fuel: 2.0, ..default() },
          Fuse { sens: tilemap::TILE * 0.5, time: 0.5 },
        ),
      ));
    }
  }
}
