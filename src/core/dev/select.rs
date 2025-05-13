use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app
    .insert_resource(Select::Hover(None))
    .add_observer(
      |trigger: Trigger<Pointer<Over>>, mut select: ResMut<Select>| {
        *select = Select::Hover(Some(trigger.target));
      },
    )
    .add_observer(|_: Trigger<Pointer<Out>>, mut select: ResMut<Select>| {
      *select = Select::Hover(None);
    });
}

#[derive(Resource)]
pub enum Select {
  Some(Entity),
  Hover(Option<Entity>),
}
