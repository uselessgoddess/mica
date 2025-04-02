use {
  crate::prelude::{core::tilemap, *},
  pathfinding::prelude::*,
  std::collections::VecDeque,
};

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Wall;

#[derive(Component, Reflect, Deref, DerefMut)]
pub struct Target(TilePos);

#[derive(Component, Reflect, Deref, DerefMut)]
pub struct Path(VecDeque<TilePos>);

pub fn plugin(app: &mut App) {
  app.register_type::<Target>().register_type::<Path>().add_systems(
    Update,
    (spawn_enemies, fix_enemies, update_paths, promote_paths),
  );

  if debug::dev() {
    app.add_systems(Update, gizmos_path);
  }
}

fn spawn_enemies(
  storage: Query<&sync::Storage>,
  enemies: Query<Entity, With<Enemy>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  let storage = storage.single();

  if enemies.iter().count() < 16 {
    let mesh = meshes.add(Circle::new(5.0));
    let material = materials.add(Color::srgb(1.0, 0.0, 1.0));

    commands.spawn(storage.center_in_world(tilemap::random())).insert((
      Enemy,
      Mesh2d(mesh),
      MeshMaterial2d(material),
    ));
  }
}

fn fix_enemies(
  enemies: Query<Entity, (With<Enemy>, Without<Target>, Without<Path>)>,
  mut commands: Commands,
) {
  for entity in enemies.iter() {
    commands.entity(entity).insert(Target(tilemap::random()));
  }
}

fn update_paths(
  storage: Query<&sync::Storage>,
  enemies: Query<(Entity, &Target, &sync::Pos)>,
  walls: Query<&Wall>,
  mut commands: Commands,
) {
  let storage = storage.single();

  let offset = |pos, x, y| -> Option<(u32, u32)> {
    let (entity, TilePos { x, y }) = storage.offset(pos, x, y)?;

    walls.get(entity).err().map(|_| (x, y))
  };

  let successors = |x, y| {
    let pos = TilePos::new(x, y);
    [
      offset(pos, 1, 0),
      offset(pos, -1, 0),
      offset(pos, 0, 1),
      offset(pos, 0, -1),
    ]
  };

  for (entity, target, &sync::Pos { x, y }) in enemies.iter() {
    if let Some(path) = bfs(
      &(x, y),
      |&(x, y)| {
        successors(x, y).into_iter().filter_map(|x| x).collect::<Vec<_>>()
      },
      |&(x, y)| x == target.x && y == target.y,
    ) {
      commands.entity(entity).remove::<Target>().insert(Path(
        path.into_iter().map(|(x, y)| TilePos::new(x, y)).collect(),
      ));
    } else {
      commands.entity(entity).remove::<Target>();
    }
  }
}

fn promote_paths(
  storage: Query<&sync::Storage>,
  mut enemies: Query<(Entity, &mut Transform, &mut Path)>,
  mut commands: Commands,
  time: ResMut<Time>,
) {
  let storage = storage.single();

  for (entity, mut transform, mut path) in enemies.iter_mut() {
    let edge = storage.center_in_world(*path.0.front().unwrap());

    let decay = f32::ln(10.0) * 50.0;
    transform.translation.smooth_nudge(
      &edge.translation,
      decay,
      time.delta_secs(),
    );

    if (transform.translation - edge.translation).length() < 0.1 {
      path.pop_front();
    }

    if path.is_empty() {
      commands.entity(entity).remove::<Path>();
    }
  }
}

fn gizmos_path(
  storage: Query<&sync::Storage>,
  query: Query<&Path>,
  mut gizmos: Gizmos,
) {
  let storage = storage.single();

  for path in query.iter() {
    path
      .iter()
      .map(|&pos| storage.center_in_world(pos).translation.xy())
      .map_windows(|&[x, y]| (x, y))
      .for_each(|(x, y)| {
        gizmos.line_2d(x, y, Color::srgb(1.0, 0.0, 0.0));
      });
  }
}
