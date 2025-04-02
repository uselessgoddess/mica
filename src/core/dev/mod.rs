use {
  crate::prelude::*,
  bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};

pub fn plugin(app: &mut App) {
  {
    app.add_plugins((
      FrameTimeDiagnosticsPlugin,
      LogDiagnosticsPlugin::filtered(vec![]),
      inspector_egui::quick::WorldInspectorPlugin::new(),
    ));
  }
}
