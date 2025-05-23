use {crate::prelude::*, pathfinding::prelude::bfs};

use {
  crate::level::{LevelSet, Occupied},
  std::collections::VecDeque,
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PathfindingSet;

pub fn plugin(app: &mut App) {
  app.configure_sets(
    PreUpdate,
    PathfindingSet
      .after(sync::SyncSet) // after tilemap setup
      .after(LevelSet::Prepare) // after obstacles prepare
      .run_if(in_state(Game::Gameplay)),
  );

  register(app)
    .init_resource::<Sink>()
    .add_systems(Update, plan)
    .add_systems(PreUpdate, promote.in_set(PathfindingSet));
}

#[derive(Debug, Component, Reflect, Default)]
pub struct Target {
  pub pos: TilePos,
  pub path: VecDeque<TilePos>,
  pub epoch: usize,
}

impl Target {
  pub fn new(pos: TilePos) -> Self {
    Self { pos, ..default() }
  }
}

fn register(app: &mut App) -> &mut App {
  app.register_type::<Target>();
  app
}

#[derive(Resource, Debug, Default, Deref, DerefMut)]
struct Sink(VecDeque<(Entity, TilePos)>);

const PATH_LIMIT: usize = 8;

fn plan(mut query: Query<(Entity, &mut Target)>, mut sink: ResMut<Sink>) {
  let mut query = query.iter_mut().collect::<Vec<_>>();
  query.shuffle(&mut rand::rng());

  for (entity, mut target) in query {
    if sink.len() < PATH_LIMIT {
      sink.push_back((entity, target.pos));
      target.epoch += 1;
    } else {
      break;
    }
  }
}

fn promote(
  storage: Single<&sync::Storage>,
  mut enemies: Query<(&mut Target, &sync::Pos)>,
  occupied: Query<&Occupied>,
  mut sink: ResMut<Sink>,
) {
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

  while let Some((entity, pos)) = sink.pop_front() {
    if let Ok((mut target, &sync::Pos { x, y })) = enemies.get_mut(entity)
      && let Some(path) = bfs(
        &(x, y),
        |&(x, y)| successors(x, y).into_iter().flatten().collect::<Vec<_>>(),
        |&(x, y)| x == pos.x && y == pos.y,
      )
    {
      let mut path: VecDeque<_> =
        path.into_iter().map(|(x, y)| TilePos::new(x, y)).collect();
      if path.len() > 1 {
        path.pop_front();
      }
      target.path = path;
    }
  }
}
