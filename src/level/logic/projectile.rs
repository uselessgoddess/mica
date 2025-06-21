use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app.register_type::<Projectile>().add_systems(Update, on_affected);
}

#[derive(Component, Reflect, Default)]
pub struct Projectile;

fn on_affected(
  query: Query<Entity, (With<Projectile>, Added<Affect>)>,
  mut commands: Commands,
) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}
