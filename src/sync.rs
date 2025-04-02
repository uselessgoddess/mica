use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app
    .register_type::<Pos>()
    .add_systems(PreUpdate, (storage, transform, tilepos));
}

#[derive(Component, Deref, DerefMut)]
pub struct Storage {
  pub translation: Vec3,
  #[deref]
  pub storage: TileStorage,
  pub grid_size: TilemapGridSize,
  pub map_type: TilemapType,
}

impl Storage {
  pub fn center_in_world(&self, pos: TilePos) -> Transform {
    let translation = pos.center_in_world(&self.grid_size, &self.map_type)
      + self.translation.xy();
    Transform::from_translation(translation.extend(0.0))
  }

  pub fn from_world_pos(&self, world: Transform) -> Option<TilePos> {
    let world = world.translation.xy() - self.translation.xy();
    TilePos::from_world_pos(&world, &self.size, &self.grid_size, &self.map_type)
  }

  pub fn offset(
    &self,
    pos: TilePos,
    x: i32,
    y: i32,
  ) -> Option<(Entity, TilePos)> {
    let pos =
      TilePos::from_i32_pair(pos.x as i32 + x, pos.y as i32 + y, &self.size)?;
    self.get(&pos).map(|entity| (entity, pos))
  }
}

fn storage(
  storage: Query<(
    Entity,
    &Transform,
    &TileStorage,
    &TilemapGridSize,
    &TilemapType,
  )>,
  mut commands: Commands,
) {
  for (entity, &transform, storage, &grid_size, &map_type) in storage.iter() {
    commands.entity(entity).insert(Storage {
      translation: transform.translation,
      storage: storage.clone(),
      grid_size,
      map_type,
    });
  }
}

fn transform(
  storage: Query<&Storage>,
  enemies: Query<(Entity, &TilePos)>,
  mut commands: Commands,
) {
  let Ok(storage) = storage.get_single() else { return };

  for (entity, &pos) in enemies.iter() {
    commands.entity(entity).insert(storage.center_in_world(pos));
  }
}

#[derive(Component, Reflect, Default, Clone, Copy, Debug)]
#[reflect(Component)]
pub struct Pos {
  pub x: u32,
  pub y: u32,
}

impl From<TilePos> for Pos {
  fn from(TilePos { x, y }: TilePos) -> Self {
    Self { x, y }
  }
}

impl From<Pos> for TilePos {
  fn from(Pos { x, y }: Pos) -> Self {
    Self { x, y }
  }
}

fn tilepos(
  storage: Query<&Storage>,
  enemies: Query<(Entity, &Transform), Without<TilePos>>,
  mut commands: Commands,
) {
  let Ok(storage) = storage.get_single() else { return };

  for (entity, &transform) in enemies.iter() {
    if let Some(pos) = storage.from_world_pos(transform).map(Pos::from) {
      commands.entity(entity).insert(pos);
    }
  }
}
