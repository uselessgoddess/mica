use crate::{
  level::{Enemy, LevelSet, Wall},
  prelude::*,
};

#[derive(Component, Reflect)]
pub struct Occupied;

pub fn plugin(app: &mut App) {
  app
    .register_type::<Occupied>()
    .add_systems(PreUpdate, occupy.in_set(LevelSet::Prepare))
    .add_systems(Update, gizmos.run_if(in_debug(D::L3)))
    .add_systems(PostUpdate, de_occupy);
}

fn occupy(
  storage: Single<&sync::Storage>,
  obstacles: Query<&sync::Pos, Or<(With<Enemy>, With<Wall>)>>,
  mut commands: Commands,
) {
  for &pos in obstacles.iter() {
    if let Some(entity) = storage.get(&pos.into()) {
      commands.entity(entity).insert(Occupied);
    }
  }
}

fn de_occupy(occupied: Query<Entity, With<Occupied>>, mut commands: Commands) {
  for entity in occupied.iter() {
    commands.entity(entity).remove::<Occupied>();
  }
}

fn gizmos(occupied: Query<&Transform2D, With<Occupied>>, mut gizmos: Gizmos) {
  for &transform in occupied.iter() {
    gizmos.cross_2d(
      transform.translation,
      tilemap::TILE / 2.0,
      Color::srgb(1.0, 0.0, 0.0),
    );
  }
}
