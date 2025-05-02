use {
  super::{Explosion, effects, thrust::Thrust},
  crate::{
    level::{Death, Lifetime, Projectile},
    prelude::{core::Sensor, *},
  },
};

pub fn plugin(app: &mut App) {
  register(app)
    .add_systems(Update, (spawn, thrust, guide, fuse))
    .add_systems(Update, gizmos.run_if(in_debug(D::L2)))
    .add_observer(on_affect)
    .add_observer(on_death);
}

fn register(app: &mut App) -> &mut App {
  app
    .register_type::<Fuse>()
    .register_type::<Flaps>()
    .register_type::<MissileMetadata>()
}

#[derive(Component, Reflect)]
pub struct Flaps(f32);

#[derive(Component, Reflect)]
pub struct Fuse {
  /// Radio trigger radius
  pub sens: f32,
  /// Time elapsed to fuse after ran out of fuel
  pub time: f32,
}

#[derive(Component, Reflect)]
#[require(Projectile, Thrust, Flaps)]
pub struct Missile {
  pub target: Vec2,
  /// Explosion radius
  pub radius: f32,
}

impl Default for Flaps {
  fn default() -> Self {
    Self(10000.0)
  }
}

#[derive(Component, Reflect)]
pub struct MissileMetadata {
  pub explosion: Handle<EffectAsset>,
  pub contrail: Handle<EffectAsset>,
}

pub(crate) fn spawn(
  query: Query<Entity, Added<Missile>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut effects: ResMut<Assets<EffectAsset>>,
) {
  let (width, height) = (tilemap::TILE * 0.12, tilemap::TILE * 0.25);

  for entity in query.iter() {
    let mesh = meshes.add(Rectangle::new(width, height));
    let material = materials.add(Color::srgb(0.35, 0.35, 0.35));
    commands
      .entity(entity)
      .insert((
        RigidBody::Dynamic,
        physics::projectile(),
        Sensor::none(false),
        ExternalForce::default().with_persistence(false),
        ExternalTorque::default().with_persistence(false),
        Collider::rectangle(width, height),
      ))
      .insert((LinearDamping(0.3), AngularDamping(2.5)))
      .insert((Mesh2d(mesh), MeshMaterial2d(material)))
      .insert(MissileMetadata {
        explosion: effects.add(effects::explosion()),
        contrail: effects.add(effects::contrail()),
      });
  }
}

fn fuse(
  query: Query<(Entity, &Fuse, &Missile, &Transform2D)>,
  mut commands: Commands,
) {
  for (entity, fuse, missile, Transform2D { translation, .. }) in query.iter() {
    if translation.distance(missile.target) < fuse.sens {
      commands.entity(entity).trigger(Death);
    }
  }
}

fn thrust(
  mut query: Query<(
    Entity,
    &Transform2D,
    &Fuse,
    &mut Thrust,
    &mut ExternalForce,
  )>,
  mut commands: Commands,
  time: Res<Time>,
) {
  let delta = time.delta_secs();
  for (entity, transform, fuse, mut thrust, mut force) in query.iter_mut() {
    // Fuel reach zero level
    if thrust.fuel >= 0.0 && thrust.fuel - delta < 0.0 {
      commands.entity(entity).insert(Lifetime::from_secs(fuse.time));
    }
    if thrust.fuel < 0.0 {
      continue;
    } else {
      thrust.fuel -= time.delta_secs();
    }
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
  query: Query<(&Transform2D, &Missile, &MissileMetadata)>,
  mut commands: Commands,
) {
  let (entity, _) = trigger.read_event();

  if let Ok((&transform, missile, MissileMetadata { explosion, .. })) =
    query.get(entity)
  {
    commands
      .spawn((transform, Explosion { radius: missile.radius, damage: 100.0 }));
    commands.spawn((
      transform,
      Lifetime::from_secs(1.0).despawn(),
      ParticleEffect::new(explosion.clone()),
    ));
    commands.entity(entity).despawn_recursive();
  }
}
