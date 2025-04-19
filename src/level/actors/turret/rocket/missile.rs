use crate::{
  level::{Death, Enemy, Projectile},
  prelude::{core::Sensor, *},
};

use super::Explosion;

pub fn plugin(app: &mut App) {
  app
    .register_type::<Missile>()
    .add_systems(
      Update,
      (spawn, thrust, guide, fuse, gizmos.run_if(in_debug(D::L2))),
    )
    .add_observer(on_affect)
    .add_observer(on_death);
}

#[derive(Component)]
pub struct Flaps(f32);

#[derive(Component)]
pub struct Thrust {
  pub thrust: f32,
  pub fuel: f32,
}

#[derive(Component)]
pub struct Fuse {
  /// Radio trigger radius
  pub sens: f32,
}

#[derive(Component, Reflect)]
#[require(Projectile, Thrust, Flaps)]
pub struct Missile {
  pub target: Vec2,
}

impl Default for Flaps {
  fn default() -> Self {
    Self(10000.0)
  }
}

impl Default for Thrust {
  fn default() -> Self {
    Self { thrust: 10000.0, fuel: 1.0 }
  }
}

fn spawn(
  query: Query<Entity, Added<Missile>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  let (width, height) = (tilemap::TILE * 0.12, tilemap::TILE * 0.25);

  for entity in query.iter() {
    let mesh = meshes.add(Rectangle::new(width, height));
    let material = materials.add(Color::srgb(0.35, 0.35, 0.35));
    commands
      .entity(entity)
      .insert((
        RigidBody::Dynamic,
        Sensor::none(false),
        ExternalForce::default().with_persistence(false),
        ExternalTorque::default().with_persistence(false),
        Collider::rectangle(width, height),
      ))
      .insert((LinearDamping(0.3), AngularDamping(2.5)))
      .insert((Mesh2d(mesh), MeshMaterial2d(material)));
  }
}

fn fuse(
  mut query: Query<(Entity, &Fuse, &Missile, &Transform2D)>,
  mut commands: Commands,
) {
  for (entity, fuse, missile, Transform2D { translation, .. }) in
    query.iter_mut()
  {
    if translation.distance(missile.target) < fuse.sens {
      commands.entity(entity).trigger(Death);
    }
  }
}

fn thrust(
  mut query: Query<(&Transform2D, &mut Thrust, &mut ExternalForce)>,
  time: Res<Time>,
) {
  for (transform, mut thrust, mut force) in &mut query {
    if thrust.fuel <= 0.0 {
      continue;
    }
    thrust.fuel -= time.delta_secs();

    force.set_force(transform.rotation * Vec2::Y * thrust.thrust);
  }
}

fn guide(
  mut query: Query<(&Flaps, &Missile, &Transform2D, &mut ExternalTorque)>,
) {
  for (flaps, missile, transform, mut torque) in query.iter_mut() {
    let direction = transform.rotation * Vec2::Y;

    let to_target =
      (missile.target - transform.translation).normalize_or_zero();
    let angle = direction.angle_to(to_target);

    torque.set_torque(angle.signum() * flaps.0);
  }
}

fn gizmos(mut query: Query<(&Transform2D, &Missile)>, mut gizmos: Gizmos) {
  for (transform, missile) in query.iter_mut() {
    gizmos.line_gradient_2d(
      transform.translation,
      missile.target,
      Color::srgb(0.0, 1.0, 0.0),
      Color::srgb(0.0, 0.0, 1.0),
    );
  }
}

fn on_affect(
  trigger: Trigger<Affect>,
  query: Query<(), With<Missile>>,
  mut commands: Commands,
) {
  let (entity, _) = trigger.read_event();

  if query.get(entity).is_ok() {
    commands.entity(entity).trigger(Death);
  }
}

fn on_death(
  trigger: Trigger<Death>,
  query: Query<&Transform2D, With<Missile>>,
  mut commands: Commands,
) {
  let (entity, _) = trigger.read_event();

  if let Ok(&transform) = query.get(entity) {
    commands.spawn((transform, Explosion { radius: 64.0, damage: 100.0 }));
    commands.entity(entity).despawn_recursive();
  }
}
