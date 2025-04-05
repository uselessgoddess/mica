use crate::prelude::*;

pub fn movement(
  time: Res<Time>,
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut query: Query<
    (&mut Transform2D, &mut OrthographicProjection),
    With<Camera>,
  >,
) {
  for (mut transform, mut ortho) in query.iter_mut() {
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyA) {
      direction -= Vec2::new(1.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
      direction += Vec2::new(1.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::KeyW) {
      direction += Vec2::new(0.0, 1.0);
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
      direction -= Vec2::new(0.0, 1.0);
    }

    if keyboard_input.pressed(KeyCode::KeyZ) {
      ortho.scale += 0.1 * time.delta_secs() * 5.0;
    }

    if keyboard_input.pressed(KeyCode::KeyX) {
      ortho.scale -= 0.1 * time.delta_secs() * 5.0;
    }

    ortho.scale = ortho.scale.clamp(0.1, 1.0);

    transform.translation += time.delta_secs() * direction * 500.;
  }
}
