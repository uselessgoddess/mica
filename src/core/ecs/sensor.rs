use crate::prelude::*;

pub trait Effect: Default + Clone + Send + Sync + 'static {}

#[derive(Event)]
pub struct Affect<E: Effect> {
  /// Affected entity by event
  pub entity: Entity,
  /// Entity who cause this event
  pub cause: Option<Entity>,
  /// Payload of the `Effect`
  pub effect: E,
}

impl<E: Effect> Affect<E> {
  pub fn new(entity: Entity) -> Self {
    Self { entity, cause: None, effect: E::default() }
  }

  pub fn cause(mut self, cause: Entity) -> Self {
    self.cause = Some(cause);
    self
  }

  pub fn effect(mut self, effect: E) -> Self {
    self.effect = effect;
    self
  }
}

#[derive(Event)]
pub struct Affected(pub Entity);

pub fn plugin(app: &mut App) {
  app.add_event::<Affected>();
}

pub fn effect<E: Effect>(app: &mut App) {
  app
    .add_event::<Affect<E>>()
    .add_event::<Affected>()
    .add_systems(Update, sensor::<E>);
}

#[derive(Component)]
#[require(ColliderSensor, CollidingEntities)]
pub struct Sensor<E>(pub E);

impl<E: Effect> Sensor<E> {
  pub fn effect(&self) -> E {
    self.0.clone()
  }
}

fn sensor<E: Effect>(
  query: Query<(Entity, &Sensor<E>, &CollidingEntities)>,
  mut events: EventWriter<Affect<E>>,
  mut back: EventWriter<Affected>,
) {
  for (entity, sensor, entities) in query.iter() {
    events.send_batch(
      entities
        .iter()
        .map(|&who| Affect::new(who).cause(entity).effect(sensor.effect())),
    );
    if !entities.is_empty() {
      back.send(Affected(entity));
    }
  }
}

pub trait AppExt {
  fn add_effect<E: Effect>(&mut self) -> &mut Self;
}

impl AppExt for App {
  fn add_effect<E: Effect>(&mut self) -> &mut Self {
    self.add_plugins(effect::<E>)
  }
}
