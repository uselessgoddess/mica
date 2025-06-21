use crate::{level::turret::Target, prelude::*};

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
  for (mut transform, &Target { angle, .. }, follow) in query.iter_mut() {
    if angle.abs() < f32::EPSILON {
      continue;
    }
    let rotate = follow.speed * time.delta_secs();
    let rotate =
      if angle.abs() <= rotate { angle } else { rotate * angle.signum() };

    transform.rotation *= Rot2::radians(rotate);
  }
}
