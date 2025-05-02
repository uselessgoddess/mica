use {crate::prelude::*, std::any::TypeId};

use bevy::{
  asset::{ReflectAsset, UntypedAssetId},
  reflect::TypeRegistry,
  render::camera::Viewport,
  window::{PrimaryWindow, Window},
};

use {
  bevy_inspector::{
    hierarchy::SelectedEntities, ui_for_entities_shared_components,
    ui_for_entity_with_children,
  },
  inspector_egui::{
    DefaultInspectorConfigPlugin, bevy_egui, bevy_inspector, egui,
  },
};

use {
  bevy_egui::{EguiContext, EguiContextSettings, EguiPostUpdateSet},
  egui_dock::{DockArea, DockState, NodeIndex, Style},
};

pub fn plugin(app: &mut App) {
  app
    .add_plugins(DefaultInspectorConfigPlugin)
    .add_plugins(bevy_egui::EguiPlugin)
    .insert_resource(UiState::new())
    .add_systems(
      PostUpdate,
      show_ui_system
        .before(EguiPostUpdateSet::ProcessOutput)
        .before(bevy_egui::end_pass_system)
        .before(TransformSystem::TransformPropagate),
    )
    .add_systems(PostUpdate, set_camera_viewport.after(show_ui_system));
}

fn show_ui_system(world: &mut World) {
  let Ok(mut ctx) = world
    .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
    .get_single(world)
    .cloned()
  else {
    return;
  };
  world.resource_scope::<UiState, _>(|world, mut ui_state| {
    ui_state.ui(world, ctx.get_mut())
  });
}

fn set_camera_viewport(
  state: Res<UiState>,
  primary: Query<&mut Window, With<PrimaryWindow>>,
  settings: Query<&EguiContextSettings>,
  mut cameras: Query<&mut Camera, With<PrimaryCamera>>,
) {
  let Ok(mut cam) = cameras.get_single_mut() else {
    return;
  };

  let Ok(window) = primary.get_single() else {
    return;
  };

  let scale_factor = window.scale_factor() * settings.single().scale_factor;

  let viewport_pos = state.viewport_rect.left_top().to_vec2() * scale_factor;
  let viewport_size = state.viewport_rect.size() * scale_factor;

  let physical_position =
    UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32);
  let physical_size =
    UVec2::new(viewport_size.x as u32, viewport_size.y as u32);

  // The desired viewport rectangle at its offset in "physical pixel space"
  let rect = physical_position + physical_size;

  let window_size = window.physical_size();
  // wgpu will panic if trying to set a viewport rect which has coordinates extending
  // past the size of the render target, i.e. the physical window in our case.
  // Typically this shouldn't happen- but during init and resizing etc. edge cases might occur.
  // Simply do nothing in those cases.
  if rect.x <= window_size.x && rect.y <= window_size.y {
    cam.viewport =
      Some(Viewport { physical_position, physical_size, depth: 0.0..1.0 });
  }
}

#[derive(Eq, PartialEq)]
enum InspectorSelection {
  Entities,
  Resource(TypeId, String),
  Asset(TypeId, String, UntypedAssetId),
}

#[derive(Resource)]
struct UiState {
  state: DockState<DockWindow>,
  viewport_rect: egui::Rect,
  selected_entities: SelectedEntities,
  selection: InspectorSelection,
}

impl UiState {
  pub fn new() -> Self {
    let mut state = DockState::new(vec![DockWindow::GameView]);
    let tree = state.main_surface_mut();
    let [game, _inspector] = tree.split_right(NodeIndex::root(), 0.75, vec![
      DockWindow::Settings,
      DockWindow::Inspector,
    ]);
    let [_game, _hierarchy] =
      tree.split_left(game, 0.2, vec![DockWindow::Hierarchy]);
    // let [_game, _bottom] = tree
    //   .split_below(game, 0.8, vec![DockWindow::Resources, DockWindow::Assets]);

    Self {
      state,
      selected_entities: SelectedEntities::default(),
      selection: InspectorSelection::Entities,
      viewport_rect: egui::Rect::NOTHING,
    }
  }

  fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
    let mut tab_viewer = TabViewer {
      world,
      viewport_rect: &mut self.viewport_rect,
      selected_entities: &mut self.selected_entities,
      selection: &mut self.selection,
    };
    DockArea::new(&mut self.state)
      .style(Style::from_egui(ctx.style().as_ref()))
      .show(ctx, &mut tab_viewer);
  }
}

#[derive(Debug)]
enum DockWindow {
  GameView,
  Hierarchy,
  Resources,
  Assets,
  Settings,
  Inspector,
}

struct TabViewer<'a> {
  world: &'a mut World,
  selected_entities: &'a mut SelectedEntities,
  selection: &'a mut InspectorSelection,
  viewport_rect: &'a mut egui::Rect,
}

impl egui_dock::TabViewer for TabViewer<'_> {
  type Tab = DockWindow;

  fn title(&mut self, window: &mut Self::Tab) -> egui::WidgetText {
    format!("{window:?}").into()
  }

  fn ui(&mut self, ui: &mut egui::Ui, window: &mut Self::Tab) {
    let type_registry = self.world.resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();

    match window {
      DockWindow::GameView => *self.viewport_rect = ui.clip_rect(),
      DockWindow::Hierarchy => {
        bevy_inspector::ui_for_world(self.world, ui);
      }
      DockWindow::Resources => {
        select_resource(ui, &type_registry, self.selection)
      }
      DockWindow::Assets => {
        select_asset(ui, &type_registry, self.world, self.selection)
      }
      DockWindow::Settings => {
        // TODO: move out into another module
        ui.heading("Developer Settings");

        if let Some(debug) = self.world.get_resource_mut::<D>().as_deref_mut() {
          egui::ComboBox::from_label("Select One!")
            .selected_text(format!("{debug:?}"))
            .show_ui(ui, |ui| {
              ui.selectable_value(debug, D::None, "None");
              ui.selectable_value(debug, D::L1, "L1");
              ui.selectable_value(debug, D::L2, "L2");
              ui.selectable_value(debug, D::L3, "L3");
            });
        }
      }
      DockWindow::Inspector => match *self.selection {
        InspectorSelection::Entities => match self.selected_entities.as_slice()
        {
          &[entity] => ui_for_entity_with_children(self.world, entity, ui),
          entities => {
            ui_for_entities_shared_components(self.world, entities, ui)
          }
        },
        InspectorSelection::Resource(type_id, ref name) => {
          ui.label(name);
          bevy_inspector::by_type_id::ui_for_resource(
            self.world,
            type_id,
            ui,
            name,
            &type_registry,
          )
        }
        InspectorSelection::Asset(type_id, ref name, handle) => {
          ui.label(name);
          bevy_inspector::by_type_id::ui_for_asset(
            self.world,
            type_id,
            handle,
            ui,
            &type_registry,
          );
        }
      },
    }
  }

  fn clear_background(&self, window: &Self::Tab) -> bool {
    !matches!(window, DockWindow::GameView)
  }
}

fn select_resource(
  ui: &mut egui::Ui,
  type_registry: &TypeRegistry,
  selection: &mut InspectorSelection,
) {
  let mut resources: Vec<_> = type_registry
    .iter()
    .filter(|registration| registration.data::<ReflectResource>().is_some())
    .map(|registration| {
      (
        registration.type_info().type_path_table().short_path(),
        registration.type_id(),
      )
    })
    .collect();
  resources.sort_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));

  for (resource_name, type_id) in resources {
    let selected = match *selection {
      InspectorSelection::Resource(selected, _) => selected == type_id,
      _ => false,
    };

    if ui.selectable_label(selected, resource_name).clicked() {
      *selection =
        InspectorSelection::Resource(type_id, resource_name.to_string());
    }
  }
}

fn select_asset(
  ui: &mut egui::Ui,
  type_registry: &TypeRegistry,
  world: &World,
  selection: &mut InspectorSelection,
) {
  let mut assets: Vec<_> = type_registry
    .iter()
    .filter_map(|registration| {
      let reflect_asset = registration.data::<ReflectAsset>()?;
      Some((
        registration.type_info().type_path_table().short_path(),
        registration.type_id(),
        reflect_asset,
      ))
    })
    .collect();
  assets.sort_by(|(name_a, ..), (name_b, ..)| name_a.cmp(name_b));

  for (asset_name, asset_type_id, reflect_asset) in assets {
    let handles: Vec<_> = reflect_asset.ids(world).collect();

    ui.collapsing(format!("{asset_name} ({})", handles.len()), |ui| {
      for handle in handles {
        let selected = match *selection {
          InspectorSelection::Asset(_, _, selected_id) => selected_id == handle,
          _ => false,
        };

        if ui.selectable_label(selected, format!("{:?}", handle)).clicked() {
          *selection = InspectorSelection::Asset(
            asset_type_id,
            asset_name.to_string(),
            handle,
          );
        }
      }
    });
  }
}
