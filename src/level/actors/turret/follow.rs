use {
  crate::{level::turret::Target, prelude::*},
  avian2d::math::FRAC_PI_2,
};

pub fn plugin(app: &mut App) {
  app.add_systems(Update, follow);
}

#[derive(Component)]
#[require(super::Turret)]
pub struct FollowTarget {
  pub speed: f32,
}

fn follow(
  mut query: Query<(&mut Transform2D, &Target, &FollowTarget)>,
  time: Res<Time>,
) {
  for (transform, &Target { target, .. }, follow) in query.iter_mut() {
    let Transform2D { translation: turret, rotation, .. } =
      transform.into_inner();

    let direction = *turret - target;

    if direction.length_squared() < f32::EPSILON {
      continue;
    }

    let target_angle = direction.to_angle() + FRAC_PI_2;
    let current_angle = rotation.as_radians();

    let mut angle_diff = target_angle - current_angle;

    if angle_diff.abs() < f32::EPSILON {
      continue;
    }

    let rotation_step = follow.speed * time.delta_secs();

    let rotation_amount = if angle_diff.abs() <= rotation_step {
      angle_diff
    } else {
      rotation_step * angle_diff.signum()
    };

    *rotation = Rot2::radians(current_angle + rotation_amount);
  }
}
