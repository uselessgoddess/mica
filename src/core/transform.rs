use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app
    .register_type::<Transform2D>()
    .add_systems(First, spawn)
    .add_systems(PostUpdate, sync);
}

pub fn spawn(
  query: Query<(Entity, &Transform), Added<Transform>>,
  mut commands: Commands,
) {
  for (entity, &transform) in query.iter() {
    commands.entity(entity).insert(Transform2D::from(transform));
  }
}

pub fn sync(query: Query<(Entity, &Transform2D)>, mut commands: Commands) {
  for (entity, &transform) in query.iter() {
    commands.entity(entity).insert(Transform::from(transform));
  }
}

#[derive(Debug, PartialEq, Clone, Copy, Component, Reflect)]
#[reflect(Component, Default, PartialEq, Debug)]
pub struct Transform2D {
  pub translation: Vec2,
  pub rotation: Rot2,
  pub scale: Vec2,
  pub layer: f32, // Z
}

impl Transform2D {
  pub const IDENTITY: Self = Transform2D {
    translation: Vec2::ZERO,
    rotation: Rot2::IDENTITY,
    scale: Vec2::ONE,
    layer: 0.0,
  };

  pub const fn from_translation(translation: Vec2) -> Self {
    Self { translation, ..Self::IDENTITY }
  }

  #[must_use]
  pub const fn with_scale(mut self, scale: Vec2) -> Self {
    self.scale = scale;
    self
  }

  #[must_use]
  pub const fn with_layer(mut self, layer: f32) -> Self {
    self.layer = layer;
    self
  }

  pub fn rotate(&mut self, rotation: Rot2) {
    self.rotation = rotation * self.rotation;
  }

  pub fn rotate_z(&mut self, angle: f32) {
    self.rotate(Rot2::radians(angle));
  }
}

impl Default for Transform2D {
  fn default() -> Self {
    Self::IDENTITY
  }
}

impl From<Transform> for Transform2D {
  fn from(Transform { translation, rotation, scale }: Transform) -> Self {
    Self {
      translation: translation.xy(),
      rotation: Rot2::radians(rotation.to_euler(EulerRot::XYZ).2),
      scale: scale.xy(),
      layer: translation.z,
    }
  }
}

impl From<Transform2D> for Transform {
  fn from(
    Transform2D { translation, rotation, scale, layer }: Transform2D,
  ) -> Self {
    Self {
      translation: translation.extend(layer),
      rotation: Quat::from_rotation_z(rotation.as_radians()),
      scale: scale.extend(1.0),
    }
  }
}
