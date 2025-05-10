use crate::prelude::*;

pub fn contrail() -> EffectAsset {
  let mut gradient = Gradient::new();
  gradient.add_key(0.0, Vec4::new(1.0, 1.0, 1.0, 0.25));
  gradient.add_key(1.0, Vec4::splat(0.0));

  let mut module = Module::default();

  let init_pos = SetPositionSphereModifier {
    center: module.lit(Vec3::ZERO),
    radius: module.lit(2.0),
    dimension: ShapeDimension::Surface,
  };
  let init_vel = SetVelocitySphereModifier {
    center: module.lit(Vec3::ZERO),
    speed: module.lit(2.),
  };
  let lifetime = module.lit(10.0);
  let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

  EffectAsset::new(81920, SpawnerSettings::rate(4096.0.into()), module)
    .with_name("contrail")
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    .render(ColorOverLifetimeModifier::new(gradient))
}

pub fn explosion() -> EffectAsset {
  let writer = ExprWriter::new();

  let init_pos = SetPositionCircleModifier {
    center: writer.lit(Vec3::ZERO).expr(),
    axis: writer.lit(Vec3::Y).expr(),
    radius: writer.lit(0.3).expr(),
    dimension: ShapeDimension::Volume,
  };

  let rgb = writer.rand(VectorType::VEC3F) * writer.lit(0.9) + writer.lit(0.1);
  let init_color = SetAttributeModifier::new(
    Attribute::COLOR,
    rgb.vec4_xyz_w(writer.lit(1.)).pack4x8unorm().expr(),
  );

  // The velocity is random in any direction
  let center = writer.attr(Attribute::POSITION);
  let speed = writer.lit(200.).uniform(writer.lit(300.));
  let dir = writer
    .rand(VectorType::VEC3F)
    .mul(writer.lit(2.0))
    .sub(writer.lit(1.0))
    .normalized();
  let init_vel = SetAttributeModifier::new(
    Attribute::VELOCITY,
    (center + dir * speed).expr(),
  );

  let lifetime = writer.lit(0.05).uniform(writer.lit(0.3)).expr();
  let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

  let drag = writer.lit(4.).expr();
  let update_drag = LinearDragModifier::new(drag);

  let mut color_gradient = Gradient::new();
  color_gradient.add_key(0.0, Vec4::new(4.0, 4.0, 4.0, 1.0));
  color_gradient.add_key(0.1, Vec4::new(4.0, 4.0, 0.0, 1.0));
  color_gradient.add_key(0.8, Vec4::new(4.0, 0.0, 0.0, 1.0));
  color_gradient.add_key(1.0, Vec4::new(4.0, 0.0, 0.0, 0.0));

  EffectAsset::new(
    16384,
    SpawnerSettings::once(16384.0.into()),
    writer.finish(),
  )
  .with_name("explosion")
  .init(init_pos)
  .init(init_vel)
  .init(init_color)
  .init(init_lifetime)
  .update(update_drag)
  .render(ColorOverLifetimeModifier::new(color_gradient))
  .render(SizeOverLifetimeModifier {
    gradient: Gradient::constant(Vec3::splat(2.0)),
    ..default()
  })
  .render(OrientModifier::new(OrientMode::AlongVelocity))
}
