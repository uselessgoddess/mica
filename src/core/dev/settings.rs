use {crate::prelude::*, inspector_egui::egui};

pub fn ui(ui: &mut egui::Ui, world: &mut World) {
  ui.heading("Developer Settings");

  if let Some(debug) = world.get_resource_mut::<D>().as_deref_mut() {
    egui::ComboBox::from_label("Debug level")
      .selected_text(format!("{debug:?}"))
      .show_ui(ui, |ui| {
        ui.selectable_value(debug, D::None, "None");
        ui.selectable_value(debug, D::L1, "L1");
        ui.selectable_value(debug, D::L2, "L2");
        ui.selectable_value(debug, D::L3, "L3");
      });
  }

  let mut gizmos = world
    .get_resource_mut::<GizmoConfigStore>()
    .expect("`GizmoConfigStore` expected to be created from `bevy`");
  let (config, _) = gizmos.config_mut::<PhysicsGizmos>();

  ui.checkbox(&mut config.enabled, "Physics gizmo");
}
