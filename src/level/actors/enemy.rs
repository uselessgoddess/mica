use {
  crate::{
    level::{DamageEvent, Health, Occupied, OnDeath},
    prelude::*,
  },
  pathfinding::prelude::*,
  std::collections::VecDeque,
};

pub fn plugin(app: &mut App) {
  app
    .register_type::<Target>()
    .register_type::<Path>()
    .add_systems(Update, (spawn_enemies, update_paths, promote_paths).chain())
    .add_systems(Update, gizmos.run_if(in_debug(D::L2)))
    .add_systems(Update, (poison_system, on_death))
    .add_systems(Startup, setup);
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Wall;

#[derive(Debug, Component, Reflect, Deref, DerefMut)]
pub struct Target(TilePos);

#[derive(Debug, Component, Reflect, Deref, DerefMut)]
pub struct Path(VecDeque<TilePos>);

fn setup(mut commands: Commands) {
  commands
    .spawn(Text2d::new("0"))
    .insert(Transform::from_translation(Vec3::new(-750.0, 0.0, 0.0)))
    .insert(TextFont::default().with_font_size(192.0))
    .insert(TextColor::BLACK);
}

fn poison_system(
  mut events: EventWriter<DamageEvent>,
  query: Query<Entity, With<Enemy>>,
  time: Res<Time>,
) {
  for entity in query.iter() {
    events.send(DamageEvent { entity, damage: 5.0 * time.delta_secs() });
  }
}

fn on_death(
  mut text: Query<&mut Text2d>,
  query: Query<(), With<Enemy>>,
  mut death: EventReader<OnDeath>,
  mut commands: Commands,
) {
  for &OnDeath(entity) in death.read() {
    if let Ok(_) = query.get(entity) {
      let mut text = text.iter_mut().next().unwrap();
      text.0 = (text.0.parse::<i32>().unwrap() + 1).to_string();

      commands.entity(entity).despawn_recursive();
    }
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
    commands.spawn(transform).insert((
      Enemy,
      Bar::<Health>::new(100.0),
      Mesh2d(mesh),
      MeshMaterial2d(material),
      Target(tilemap::center()),
    ));
  }
}

fn update_paths(
  storage: Query<&sync::Storage>,
  enemies: Query<(Entity, &Target, &sync::Pos)>,
  occupied: Query<&Occupied>,
  mut commands: Commands,
) {
  let storage = storage.single();

  let offset = |pos, x, y| -> Option<(u32, u32)> {
    let (entity, TilePos { x, y }) = storage.offset(pos, x, y)?;

    occupied.get(entity).err().map(|_| (x, y))
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
    if x == 15 && y == 15 {
      commands.entity(entity).despawn_recursive();
      return;
    }

    if let Some(path) = bfs(
      &(x, y),
      |&(x, y)| successors(x, y).into_iter().flatten().collect::<Vec<_>>(),
      |&(x, y)| x == target.x && y == target.y,
    ) {
      commands.entity(entity).insert(Path(
        path.into_iter().skip(1).map(|(x, y)| TilePos::new(x, y)).collect(),
      ));
    } else {
      commands.entity(entity);
    }
  }
}

fn promote_paths(
  storage: Query<&sync::Storage>,
  mut enemies: Query<(Entity, &mut Transform2D, &mut Path)>,
  mut commands: Commands,
  time: ResMut<Time>,
) {
  let storage = storage.single();

  for (entity, mut transform, mut path) in enemies.iter_mut() {
    let Some(edge) = path.0.front() else {
      return;
    };
    let edge = storage.center_in_world(*edge);

    let direction = edge - transform.translation.xy();
    transform.translation +=
      direction.normalize_or_zero() * 64.0 * time.delta_secs();

    if (transform.translation.xy() - edge).length() < 0.1 {
      path.pop_front();
    }

    if path.is_empty() {
      commands.entity(entity).remove::<Path>();
    }
  }
}

fn gizmos(
  storage: Query<&sync::Storage>,
  query: Query<&Path>,
  mut gizmos: Gizmos,
) {
  let storage = storage.single();

  for path in query.iter() {
    path
      .iter()
      .map(|&pos| storage.center_in_world(pos))
      .map_windows(|&[x, y]| (x, y))
      .for_each(|(x, y)| {
        gizmos.line_2d(x, y, Color::srgb(1.0, 0.0, 0.0));
      });
  }
}
