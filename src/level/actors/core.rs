use {super::SpawnSet, crate::prelude::*};

pub fn plugin(app: &mut App) {
  app
    .register_type::<Core>()
    .add_systems(Update, (rotate_z, spawn.in_set(SpawnSet)));
}

#[derive(Component, Reflect)]
#[require(TilePos)]
pub struct Core;

fn spawn(
  query: Query<Entity, Added<Core>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  for entity in query.iter() {
    let mesh = meshes.add(Rectangle::from_length(tilemap::TILE * 0.7));
    commands
      .entity(entity)
      .with_children(|parent| {
        parent.spawn((
          layer::<1>(),
          RotateZ(5.0),
          Mesh2d(mesh.clone()),
          MeshMaterial2d(materials.add(Color::srgb(0.25, 0.0, 0.75))),
        ));
      })
      .with_children(|parent| {
        parent.spawn((
          layer::<2>(),
          RotateZ(-2.0),
          Mesh2d(mesh.clone()),
          MeshMaterial2d(materials.add(Color::srgb(0.75, 0.0, 0.25))),
        ));
      });
  }
}

#[derive(Component)]
struct RotateZ(f32);

fn rotate_z(mut query: Query<(&mut Transform2D, &RotateZ)>, time: Res<Time>) {
  for (mut transform, &RotateZ(rotate)) in query.iter_mut() {
    transform.rotate_z(rotate * time.delta_secs());
  }
}
