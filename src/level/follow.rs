use {
  crate::prelude::*,
  bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    window::PrimaryWindow,
  },
};

pub fn plugin(app: &mut App) {
  app
    .register_type::<Follow>()
    .register_type::<FollowMouse>()
    .add_systems(Update, (follow, follow_mouse));
}

#[derive(Component, Reflect, Copy, Clone)]
#[require(Transform)]
#[component(on_add = on_add)]
pub struct Follow(pub Entity);

fn on_add(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
  let Some(Follow(target)) = world.get(entity).copied() else { unreachable!() };

  if entity != target
    && let Some(global) = world.get::<GlobalTransform>(target).copied()
    && let Some(mut transform) = world.get_mut::<Transform2D>(entity)
  {
    *transform = global.compute_transform().into();
  }
}

fn follow(
  query: Query<(Entity, &Follow)>,
  targets: Query<&GlobalTransform>,
  mut commands: Commands,
) {
  for (entity, &Follow(target)) in query.iter() {
    if entity != target
      && let Ok(&global) = targets.get(target)
    {
      commands.entity(entity).try_insert(global.compute_transform());
    }
  }
}

#[derive(Component, Reflect, Default, Copy, Clone)]
#[require(Transform)]
pub struct FollowMouse;

fn follow_mouse(
  mut query: Query<&mut Transform2D, With<FollowMouse>>,
  window: Single<&Window, With<PrimaryWindow>>,
  q_camera: Single<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
) {
  let (camera, transform) = q_camera.into_inner();

  if let Some(cursor) = window.cursor_position()
    && let Ok(cursor) = camera.viewport_to_world_2d(transform, cursor)
  {
    for mut follow in query.iter_mut() {
      follow.translation = cursor;
    }
  }
}
