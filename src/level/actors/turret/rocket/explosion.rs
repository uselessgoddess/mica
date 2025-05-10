use crate::{
  core::Sensor,
  level::{Damage, Death, Lifetime, Projectile},
  prelude::*,
};

pub fn plugin(app: &mut App) {
  app
    .register_type::<Explosion>()
    .add_systems(Update, (spawn,))
    .add_observer(on_affect)
    .add_observer(on_death);
}

#[derive(Component, Reflect)]
#[require(Projectile)]
pub struct Explosion {
  pub radius: f32,
  pub damage: f32,
}

fn spawn(
  query: Query<(Entity, &Explosion), Added<Explosion>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  for (entity, &Explosion { radius, damage }) in query.iter() {
    let mesh = meshes.add(Circle::new(radius));
    let material = materials.add(Color::srgba(1.0, 0.35, 0.35, 0.25));
    commands
      .entity(entity)
      .insert((
        Sensor::new(Damage(damage)),
        Collider::circle(radius),
        Lifetime::from_secs(0.25),
      ))
      .insert((Mesh2d(mesh), MeshMaterial2d(material)));
  }
}

fn on_affect(
  trigger: Trigger<Affect>,
  query: Query<(), With<Explosion>>,
  mut commands: Commands,
) {
  let (entity, _) = trigger.read_event();

  if query.get(entity).is_ok() {
    commands.entity(entity).remove::<Sensor>();
  }
}

fn on_death(
  trigger: Trigger<Death>,
  query: Query<&Transform2D, With<Explosion>>,
  mut commands: Commands,
) {
  let (entity, _) = trigger.read_event();

  if query.get(entity).is_ok() {
    commands.entity(entity).despawn_recursive();
  }
}
