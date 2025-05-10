use {
  crate::prelude::*,
  bevy::ecs::{component::ComponentId, world::DeferredWorld},
};

pub fn plugin(app: &mut App) {
  app.register_type::<Follow>().add_systems(Update, follow);
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
