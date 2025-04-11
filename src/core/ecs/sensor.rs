use crate::prelude::*;

pub trait Effect: Default + Clone + Send + Sync + 'static {
  type Event: Event;

  fn affect(&self, entity: (Entity, Entity)) -> Self::Event;
}

#[derive(Event)]
pub struct Affected(pub Entity);

pub fn plugin(app: &mut App) {
  app.add_event::<Affected>();
}

pub fn effect<E: Effect>(app: &mut App) {
  app.add_event::<E::Event>().add_systems(Update, sensor::<E>);
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
  mut events: EventWriter<E::Event>,
) {
  for (entity, sensor, entities) in query.iter() {
    events.send_batch(
      entities
        .iter()
        .map(|&affected| sensor.effect().affect((entity, affected))),
    );
  }
}
