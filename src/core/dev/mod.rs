use {
  crate::prelude::*,
  bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
  inspector_egui::quick,
};

pub fn plugin(app: &mut App) {
  {
    app.add_plugins((
      FrameTimeDiagnosticsPlugin,
      LogDiagnosticsPlugin::filtered(vec![]),
      quick::WorldInspectorPlugin::new(),
    ));

    if app.is_debug(D::L1) {
      app.add_plugins(PhysicsDebugPlugin::default()).insert_gizmo_config(
        PhysicsGizmos { aabb_color: Some(Color::WHITE), ..default() },
        GizmoConfig::default(),
      );
    }
  }
}
