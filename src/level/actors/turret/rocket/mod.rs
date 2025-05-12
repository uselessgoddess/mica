mod effects;
mod explosion;
mod missile;
mod thrust;

use crate::prelude::*;

use crate::level::{
  ChildOf,
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

pub struct Build;

impl ecs::Build for Build {
  type Input = TilePos;

  fn apply(&self, input: Self::Input, world: &mut World, entity: Entity) {
    world.entity_mut(entity).insert(input).insert(Rocket::default());
  }
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
  mut effects: ResMut<Assets<EffectAsset>>,
) {
  for entity in query.iter() {
    let mesh = meshes.add(Circle::new(tilemap::TILE * 0.25));
    let material = materials.add(Color::srgb(0.25, 0.15, 0.25));
    commands
      .entity(entity)
      .insert((Mesh2d(mesh), MeshMaterial2d(material)))
      .insert(MissileMetadata {
        explosion: effects.add(effects::explosion()),
        contrail: effects.add(effects::contrail()),
      });
  }
}

fn attack(
  mut turrets: Query<(
    Entity,
    &Transform2D,
    &MonitorTargets,
    &MissileMetadata,
    &mut Rocket,
  )>,
  mut commands: Commands,
  time: Res<Time>,
) {
  for (entity, &transform, monitor, metadata, mut rocket) in turrets.iter_mut()
  {
    if let Some(Target { target, .. }) = monitor.first().copied()
      && rocket.cooldown.tick(time.delta()).just_finished()
    {
      commands.spawn((
        Name::new("Missile"),
        metadata.clone(),
        transform.add_layer(1.0),
        (
          Missile { target, radius: tilemap::TILE },
          Thrust { fuel: 2.0, ..default() },
          Fuse { sens: tilemap::TILE * 0.5, time: 1.0 },
          // to receive designation from parent
          ChildOf(entity),
        ),
      ));
    }
  }
}
