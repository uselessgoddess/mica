use crate::{
  level::{
    Enemy, Turret,
    turret::{MonitorTargets, Slave, Target, TurretSet},
  },
  prelude::*,
};

pub fn plugin(app: &mut App) {
  app
    .register_type::<Neighbors>()
    .register_type::<Designator>()
    .add_systems(Update, spawn)
    .add_systems(
      PostUpdate,
      (neighbors, target, designate).after(TurretSet::Monitor),
    );
}

#[derive(Component, Reflect, Default, Deref, DerefMut)]
pub struct Neighbors(Vec<Entity>);

#[derive(Component, Reflect, Default)]
#[require(Neighbors)]
pub struct Designator;

fn spawn(
  query: Query<Entity, Added<Designator>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  let mesh = meshes.add(Circle::new(tilemap::TILE * 0.5));
  let material = materials.add(Color::srgb(0.25, 0.0, 0.75));

  for entity in query.iter() {
    commands
      .entity(entity)
      .insert(Name::new("Designator"))
      .insert((Mesh2d(mesh.clone()), MeshMaterial2d(material.clone())));
  }
}

fn neighbors(
  storage: Single<&sync::Storage>,
  mut query: Query<(&sync::Pos, &mut Neighbors)>,
  turrets: Query<(Entity, &sync::Pos), With<Turret>>,
  mut commands: Commands,
) {
  use ecs_tilemap::helpers::square_grid::neighbors::Neighbors as TileNeighbors;

  for (&pos, mut neighbors) in query.iter_mut() {
    let area: Vec<_> = TileNeighbors::get_square_neighboring_positions(
      &pos.into(),
      &storage.size,
      /* diagonals */ false,
    )
    .iter()
    .copied()
    .collect();

    // TODO: implement custom areas in the future
    neighbors.0 = turrets
      .iter()
      .filter(|&(_, pos)| area.contains(&TilePos::from(*pos)))
      .map(|(entity, _)| entity)
      .collect();

    let batch: Vec<_> =
      neighbors.iter().map(|&entity| (entity, Slave)).collect();
    commands.insert_batch(batch);
  }
}

fn target(
  enemies: Query<(Entity, &Transform2D), With<Enemy>>,
  designs: Query<(Entity, &Transform2D, &Designator)>,
  mut commands: Commands,
) {
  for (entity, &Transform2D { translation: design, .. }, _) in designs.iter() {
    let mut targets =
      enemies.iter().map(|(entity, Transform2D { translation: enemy, .. })| {
        Target::new(entity, design, *enemy)
      });

    if let Some(target) = targets.next() {
      commands.entity(entity).insert(target);
    }
  }
}

fn designate(
  designs: Query<(&Target, &Designator, &Neighbors)>,
  mut commands: Commands,
) {
  for (&target, _, neighbors) in designs.iter() {
    let monitor = MonitorTargets::single(target);
    let batch: Vec<_> =
      neighbors.iter().map(|&entity| (entity, monitor.clone())).collect();
    commands.insert_batch(batch);
  }
}
