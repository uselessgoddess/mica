use bevy::{
  core::FrameCount,
  log::{DEFAULT_FILTER, LogPlugin},
  prelude::*,
  window::WindowResolution,
};

pub fn plugin(app: &mut App) {
  app
    .add_plugins(
      DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
          primary_window: Window {
            resolution: WindowResolution::new(1920.0, 1080.0),
            title: "Mica".to_string(),
            visible: true,
            ..default()
          }
          .into(),
          ..default()
        })
        .set(LogPlugin {
          filter: format!("{},bevy_hanabi=error", DEFAULT_FILTER),
          ..default()
        }),
    )
    .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.4)))
    .add_systems(Update, make_visible);
}

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
  // The delay may be different for your core or system.
  if frames.0 == 3 {
    // At this point the gpu is ready to show the core so we can make the window visible.
    // Alternatively, you could toggle the visibility in Startup.
    // It will work, but it will have one white frame before it starts rendering
    window.single_mut().visible = true;
  }
}
