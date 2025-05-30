use crate::{
  level::{Damage, Lifetime, Projectile, SpawnSet, turret::Target},
  prelude::{core::Sensor, *},
};

pub fn plugin(app: &mut App) {
  register(app)
    .add_systems(Update, spawn.in_set(SpawnSet))
    .add_observer(on_affect);
}

fn register(app: &mut App) -> &mut App {
  app
}

#[derive(Component, Reflect, Copy, Clone)]
#[require(Projectile)]
pub struct Bullet {
  pub damage: f32,
}

fn spawn(
  query: Query<(Entity, &Bullet), Added<Bullet>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  use bevy::color::palettes::tailwind::YELLOW_50;

  let (width, height) = (tilemap::TILE * 0.05, tilemap::TILE * 0.25);

  for (entity, &bullet) in query.iter() {
    let mesh = meshes.add(Rectangle::new(width, height));
    let material = materials.add(Color::from(YELLOW_50 * 2.0));

    commands.queue(move |world: &mut World| {
      let Ok(mut entity) = world.get_entity_mut(entity) else { return };
      entity
        .insert(physics::projectile())
        .insert(Sensor::new(Damage(bullet.damage)).with_sensor(false))
        .insert((
          RigidBody::Dynamic,
          ExternalForce::default().with_persistence(false),
          ExternalTorque::default().with_persistence(false),
          Collider::rectangle(width, height),
        ))
        .insert((Mesh2d(mesh.clone()), MeshMaterial2d(material.clone())));
    });
  }
}

fn on_affect(
  trigger: Trigger<Affect>,
  query: Query<(), With<Bullet>>,
  mut commands: Commands,
) {
  let (entity, _) = trigger.read_event();

  if query.get(entity).is_ok() {
    commands.entity(entity).despawn_recursive();
  }
}
