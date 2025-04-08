use {
  super::Enemy,
  crate::{level::DamageEvent, prelude::*},
};

pub fn plugin(app: &mut App) {
  app.register_type::<Turret>().add_systems(Update, (spawn, attack));
}

#[derive(Component, Reflect)]
pub struct Turret;

fn spawn(
  query: Query<Entity, Added<Turret>>,
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
  query: Query<(Entity, &Transform2D), With<Turret>>,
  enemies: Query<(Entity, &Transform2D), With<Enemy>>,
  mut damage: EventWriter<DamageEvent>,
  mut gizmos: Gizmos,
  time: Res<Time>,
) {
  for (_, a) in query.iter() {
    let enemy = enemies
      .iter()
      .map(|(entity, b)| (entity, b, (a.translation - b.translation).length()))
      .min_by(|(_, _, a), (_, _, b)| f32::total_cmp(a, b));
    if let Some((entity, b, _)) = enemy {
      damage.send(DamageEvent { entity, damage: 50.0 * time.delta_secs() });
      gizmos.line_2d(a.translation, b.translation, Color::srgb(0.0, 1.0, 0.0));
    }
  }
}
