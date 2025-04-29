use {
  super::MissileMetadata,
  crate::{
    level::{Follow, Period},
    prelude::*,
  },
};

pub fn plugin(app: &mut App) {
  app.register_type::<Thrust>().add_systems(Update, (spawn, clear));
}

#[derive(Debug, Component, Reflect, Copy, Clone)]
pub struct Thrust {
  pub thrust: f32,
  pub fuel: f32,
}

impl Default for Thrust {
  fn default() -> Self {
    Self { thrust: 10000.0, fuel: 1.0 }
  }
}

#[derive(Component)]
pub struct ThrustEffect(Entity);

fn spawn(
  // requires `MissileMetadata` to satisfy system ordering
  query: Query<
    (Entity, &Thrust, &MissileMetadata, &Transform2D),
    Added<MissileMetadata>,
  >,
  mut commands: Commands,
) {
  for (parent, thrust, MissileMetadata { contrail, .. }, &transform) in
    query.iter()
  {
    let mut target = Entity::PLACEHOLDER;
    commands.entity(parent).with_children(|parent| {
      target = parent.spawn(Transform2D::from_xy(0.0, -10.0)).id();
    });
    commands.spawn((
      Name::new("Contrail"),
      (transform, Follow(target), ThrustEffect(parent)),
      Period::from_secs(thrust.fuel + 10.0).despawn(),
      ParticleEffect::new(contrail.clone()),
    ));
  }
}

fn clear(
  mut query: Query<(&ThrustEffect, &mut EffectSpawner)>,
  thrusts: Query<&Thrust>,
) {
  for (&ThrustEffect(parent), mut spawner) in query.iter_mut() {
    if let Ok(Thrust { fuel: ..=0.0, .. }) | Err(_) = thrusts.get(parent) {
      spawner.active = false;
    }
  }
}
