use crate::{
  level::{Damage, Enemy},
  prelude::*,
};

pub fn plugin(app: &mut App) {
  app.register_type::<Laser>().add_systems(Update, (spawn, attack));
}

#[derive(Component, Reflect)]
pub struct Laser;

fn spawn(
  query: Query<Entity, Added<Laser>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  for entity in query.iter() {
    let mesh = meshes.add(Circle::new(tilemap::TILE * 0.25));
    let material = materials.add(Color::srgb(0.0, 0.75, 0.25));
    commands.entity(entity).insert((Mesh2d(mesh), MeshMaterial2d(material)));
  }
}

fn attack(
  query: Query<(Entity, &Transform2D), With<Laser>>,
  enemies: Query<(Entity, &Transform2D), With<Enemy>>,
  mut events: EventWriter<Affect<Damage>>,
  mut gizmos: Gizmos,
  time: Res<Time>,
) {
  let damage = Damage(10.0 * time.delta_secs());
  for (_, a) in query.iter() {
    let enemy = enemies
      .iter()
      .map(|(entity, b)| (entity, b, (a.translation - b.translation).length()))
      .min_by(|(_, _, a), (_, _, b)| f32::total_cmp(a, b));

    let Some((entity, b, _)) = enemy else { continue };

    gizmos.line_2d(a.translation, b.translation, Color::srgb(0.0, 1.0, 0.0));
    events.send(Affect::new(entity).effect(damage));
  }
}
