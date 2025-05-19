use crate::{
  level::{Damage, Death, Health, Occupied, pathfinding::Target},
  prelude::*,
};

pub fn plugin(app: &mut App) {
  app
    .add_systems(Update, (spawn_enemies, promote_paths).chain())
    .add_systems(Update, poison_system)
    .add_systems(Update, gizmos.run_if(in_debug(D::L2)))
    .add_observer(on_death);
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Wall;

fn poison_system(
  query: Query<Entity, With<Enemy>>,
  mut commands: Commands,
  time: Res<Time>,
) {
  let damage = Damage(5.0 * time.delta_secs());
  for entity in query.iter() {
    commands.entity(entity).trigger(damage);
  }
}

// TODO: use custom system to filter by components like
fn on_death(
  trigger: Trigger<Death>,
  query: Query<(), With<Enemy>>,
  mut commands: Commands,
) {
  let (entity, _) = trigger.read_event();

  if query.get(entity).is_ok() {
    commands.entity(entity).despawn_recursive();
  }
}

fn spawn_enemies(
  storage: Query<&sync::Storage>,
  enemies: Query<Entity, With<Enemy>>,
  occupied: Query<&Occupied>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  let storage = storage.single();

  if enemies.iter().count() < 16 {
    let sample = tilemap::sample_border();

    if let Some(entity) = storage.get(&sample)
      && occupied.get(entity).is_ok()
    {
      return;
    }

    let mesh = meshes.add(Circle::new(5.0));
    let material = materials.add(Color::srgb(1.0, 0.0, 1.0));

    let transform =
      Transform2D::from_translation(storage.center_in_world(sample));
    commands
      .spawn(transform)
      .insert((
        Enemy,
        Bar::<Health>::new(100.0),
        Target::new(tilemap::center()),
        Collider::circle(5.0),
      ))
      .insert((Mesh2d(mesh), MeshMaterial2d(material)));
  }
}

fn promote_paths(
  storage: Query<&sync::Storage>,
  mut enemies: Query<(&mut Transform2D, &mut Target)>,
  time: ResMut<Time>,
) {
  let storage = storage.single();

  for (mut transform, mut target) in enemies.iter_mut() {
    let Some(edge) = target.path.front() else {
      return;
    };
    let edge = storage.center_in_world(*edge);

    let direction = edge - transform.translation.xy();
    transform.translation +=
      direction.normalize_or_zero() * 64.0 * time.delta_secs();

    if transform.translation.distance(edge) < 1.0 {
      target.path.pop_front();
    }
  }
}

fn gizmos(
  storage: Query<&sync::Storage>,
  query: Query<&Target>,
  mut gizmos: Gizmos,
) {
  let storage = storage.single();

  for target in query.iter() {
    (target.path.iter())
      .map(|&pos| storage.center_in_world(pos))
      .map_windows(|&[x, y]| (x, y))
      .for_each(|(x, y)| {
        gizmos.line_2d(x, y, Color::srgb(1.0, 0.0, 0.0));
      });
  }
}
