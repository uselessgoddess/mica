use {super::SpawnSet, crate::prelude::*, bevy::core_pipeline::bloom::Bloom};

pub fn plugin(app: &mut App) {
  app.add_systems(Update, spawn.in_set(SpawnSet));
}

fn spawn(query: Query<Entity, Added<PrimaryCamera>>, mut commands: Commands) {
  for entity in query.iter() {
    commands
      .entity(entity)
      .insert((
        UiSourceCamera::<0>,
        Transform2D::IDENTITY.with_layer(ui::DEPTH),
      ))
      .insert((Camera { hdr: true, ..default() }, Bloom::OLD_SCHOOL, PanCam {
        grab_buttons: vec![MouseButton::Left, MouseButton::Middle],
        speed: 500.0,
        ..default()
      }));
  }
}
