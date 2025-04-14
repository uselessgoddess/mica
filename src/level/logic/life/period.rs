use {
  crate::{level::Death, prelude::*},
  std::time::Duration,
};

pub fn plugin(app: &mut App) {
  app.register_type::<Period>().add_systems(Update, period);
}

#[derive(Component, Reflect, Deref, DerefMut)]
pub struct Period(Timer);

impl Period {
  pub fn new(duration: Duration) -> Self {
    Self(Timer::new(duration, TimerMode::Once))
  }

  pub fn from_secs(secs: f32) -> Self {
    Self::new(Duration::from_secs_f32(secs))
  }
}

fn period(
  mut query: Query<(Entity, &mut Period)>,
  mut commands: Commands,
  time: Res<Time>,
) {
  for (entity, mut period) in query.iter_mut() {
    if period.tick(time.delta()).finished() {
      commands.entity(entity).trigger(Death);
    }
  }
}
