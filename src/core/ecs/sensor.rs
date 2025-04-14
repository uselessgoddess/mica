use crate::prelude::*;

#[derive(Event, Copy, Clone)]
pub struct Affect;

pub fn plugin(app: &mut App) {
  app.add_effect::<Nothing>().add_event::<Affect>();
}

pub fn effect<E: Event + Clone>(app: &mut App) {
  app.add_systems(Update, sensor::<E>).add_observer(
    |trigger: Trigger<OnAdd, Sensor<E>>,
     query: Query<&Sensor<E>>,
     mut commands: Commands| {
      if let entity = trigger.entity()
        && let Ok(sensor) = query.get(entity)
        && sensor.sensor
      {
        commands.entity(entity).insert(avian2d::prelude::Sensor);
      }
    },
  );
}

#[derive(Event, Copy, Clone)]
pub struct Nothing;

#[derive(Component)]
#[require(CollidingEntities)]
pub struct Sensor<E: Event = Nothing> {
  sensor: bool,
  effect: E,
}

impl Sensor<Nothing> {
  pub fn none(sensor: bool) -> Self {
    Self { sensor, effect: Nothing }
  }
}

impl<E: Event> Sensor<E> {
  pub fn new(effect: E) -> Self {
    Self { effect, sensor: true }
  }

  pub fn with_sensor(mut self, sensor: bool) -> Self {
    self.sensor = sensor;
    self
  }
}

impl<E: Event + Clone> Sensor<E> {
  pub fn effect(&self) -> E {
    self.effect.clone()
  }
}

fn sensor<E: Event + Clone>(
  query: Query<(Entity, &Sensor<E>, &CollidingEntities)>,
  mut commands: Commands,
) {
  for (entity, sensor, entities) in query.iter() {
    commands.trigger_targets(
      sensor.effect(),
      entities.iter().copied().collect::<Vec<_>>(),
    );
    if !entities.is_empty() {
      commands.entity(entity).trigger(Affect);
    }
  }
}

pub trait AppExt {
  fn add_effect<E: Event + Clone>(&mut self) -> &mut Self;
}

impl AppExt for App {
  fn add_effect<E: Event + Clone>(&mut self) -> &mut Self {
    self.add_event::<E>().add_plugins(effect::<E>)
  }
}
