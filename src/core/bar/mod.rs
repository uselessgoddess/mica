use {
  crate::prelude::*,
  bevy::reflect::{GetTypeRegistration, Typed},
};

mod settings;

pub use settings::{BarHeight, ColorScheme, ForegroundColor, Settings};

pub fn plugin<P: Percentage>(app: &mut App) {
  app
    .register_type::<Bar<P>>()
    .init_resource::<ColorScheme<P>>()
    .add_systems(Update, spawn::<P>);
}

pub trait Percentage: TypePath + Send + Sync + 'static {
  type Item: GetTypeRegistration
    + FromReflect
    + Typed
    + num::Num
    + PartialEq
    + PartialOrd
    + Copy
    + Clone
    + Send
    + Sync
    + 'static;

  fn value(value: Self::Item, limit: Self::Item) -> f32;
}

/// Range of any bar values
#[derive(Reflect)]
pub struct Payload<P: Percentage> {
  value: P::Item,
  limit: P::Item,
}

impl<P: Percentage> Payload<P> {
  pub fn new(value: P::Item, limit: P::Item) -> Self {
    Self { value, limit }
  }

  pub fn value(&self) -> f32 {
    P::value(self.value, self.limit)
  }

  pub fn set(&mut self, value: P::Item) {
    self.value = num::clamp(value, num::zero(), self.limit);
  }

  pub fn inc(&mut self, value: P::Item) {
    self.set(self.value + value);
  }

  pub fn dec(&mut self, value: P::Item) {
    if value >= self.value {
      self.value = num::zero();
    } else {
      self.value = self.value - value;
    }
  }

  pub fn is_empty(&self) -> bool {
    self.value == num::zero()
  }
}

#[derive(Copy, Clone, Component)]
struct Repr(Entity, Entity);

#[derive(Component, Reflect, Deref, DerefMut)]
pub struct Bar<P: Percentage> {
  #[deref]
  payload: Payload<P>,
  settings: Settings,
}

impl<P: Percentage> Bar<P> {
  pub fn new(limit: P::Item) -> Self {
    Self { payload: Payload::new(limit, limit), settings: Default::default() }
  }

  pub fn with_settings(mut self, settings: Settings) -> Self {
    self.settings = settings;
    self
  }
}

fn spawn<P: Percentage>(
  mut query: Query<(Entity, &Bar<P>, Option<&mut Repr>), Changed<Bar<P>>>,
  mut q_scale: Query<&mut Transform2D>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  scheme: Res<ColorScheme<P>>,
) {
  for (entity, Bar { payload, settings }, repr) in query.iter_mut() {
    let mesh = meshes.add(Rectangle::new(1.0, 1.0));
    let modify = |transform: &mut Transform2D, width| {
      transform.translation.y = settings.offset;
      transform.scale = Vec2::new(width, settings.absolute_height());
    };

    let width = settings.width;
    if let Some(Repr(fore, back)) = repr.as_deref().copied() {
      if let Ok(mut transform) = q_scale.get_mut(fore) {
        modify(&mut transform, width * payload.value());
      }
      if let Ok(mut transform) = q_scale.get_mut(back) {
        modify(&mut transform, width);
      }
      continue;
    }
    let transform = |layer, width| {
      let mut transform = Transform2D::IDENTITY;
      modify(&mut transform, width);
      transform.layer = layer;
      transform
    };

    let ForegroundColor::Static(foreground) = scheme.foreground;

    let fore = MeshMaterial2d(materials.add(foreground));
    let back = MeshMaterial2d(materials.add(scheme.background));

    let back = commands
      .spawn(transform(1.0, width))
      .insert((back, Mesh2d(mesh.clone())))
      .id();

    let fore = commands
      .spawn(transform(2.0, width * payload.value()))
      .insert((fore, Mesh2d(mesh.clone())))
      .id();

    commands.entity(entity).insert(Repr(fore, back)).with_children(|parent| {
      parent
        .spawn(Name::new(type_name::<P>()))
        .insert((Transform2D::IDENTITY, Visibility::Visible))
        .add_child(fore)
        .add_child(back);
    });
  }
}
