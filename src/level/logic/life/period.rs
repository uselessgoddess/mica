use {
  crate::{level::Death, prelude::*},
  std::time::Duration,
};

pub fn plugin(app: &mut App) {
  app.register_type::<Period>().add_systems(Update, period);
}

#[derive(Component, Reflect, Deref, DerefMut)]
pub struct Period {
  #[deref]
  timer: Timer,
  despawn: bool,
}

impl Period {
  pub fn new(duration: Duration) -> Self {
    Self { timer: Timer::new(duration, TimerMode::Once), despawn: false }
  }

  pub fn from_secs(secs: f32) -> Self {
    Self::new(Duration::from_secs_f32(secs))
  }

  pub fn despawn(mut self) -> Self {
    self.despawn = true;
    self
  }
}

fn period(
  mut query: Query<(Entity, &mut Period)>,
  mut commands: Commands,
  time: Res<Time>,
) {
  for (entity, mut period) in query.iter_mut() {
    if period.tick(time.delta()).finished() {
      if period.despawn {
        commands.entity(entity).despawn_recursive();
      } else {
        commands.entity(entity).trigger(Death);
      }
    }
  }
}
