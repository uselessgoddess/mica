mod editor;
mod select;
mod settings;

use {
  crate::prelude::*,
  bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};

pub(crate) use select::Select;

pub fn plugin(app: &mut App) {
  {
    app.add_plugins((
      FrameTimeDiagnosticsPlugin,
      LogDiagnosticsPlugin::filtered(vec![]),
    ));

    if app.is_debug(D::L1) {
      app.add_plugins(PhysicsDebugPlugin::default()).insert_gizmo_config(
        PhysicsGizmos { aabb_color: Some(Color::WHITE), ..default() },
        GizmoConfig::default(),
      );
    }

    #[cfg(feature = "debug")]
    app.add_plugins((editor::plugin, select::plugin));
  }
}
