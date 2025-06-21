use {
  super::Select,
  crate::prelude::*,
  inspector_egui::{bevy_inspector, egui},
};

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

  gizmos::<PhysicsGizmos>(ui, world, "Physics gizmos");
  gizmos::<DefaultGizmoConfigGroup>(ui, world, "System gizmos");

  if let Some(select) = world.get_resource::<Select>()
    && let Select::Some(entity) | Select::Hover(Some(entity)) = *select
  {
    bevy_inspector::ui_for_entity(world, entity, ui);
  }
}

fn gizmos<G: GizmoConfigGroup>(
  ui: &mut egui::Ui,
  world: &mut World,
  label: &str,
) {
  let mut gizmos =
    world.get_resource_mut::<GizmoConfigStore>().unwrap_or_else(|| {
      panic!("`{}` expected to be created from `bevy`", type_name::<G>())
    });
  let (config, _) = gizmos.config_mut::<G>();

  ui.checkbox(&mut config.enabled, label);
}
