use crate::prelude::*;

pub trait Build: Send + Sync + 'static {
  type Input: Send;

  fn apply(&self, input: Self::Input, world: &mut World, entity: Entity);
}

impl<B: Build + ?Sized> Build for Box<B> {
  type Input = B::Input;

  fn apply(&self, input: Self::Input, world: &mut World, entity: Entity) {
    (**self).apply(input, world, entity);
  }
}

#[rustfmt::skip]
pub trait CommandsExt {
  fn spawn_dynamic<B: Build>(
    &mut self,
    input: B::Input,
    build: B,
  ) -> &mut Self;
}

impl CommandsExt for Commands<'_, '_> {
  fn spawn_dynamic<B: Build>(
    &mut self,
    input: B::Input,
    build: B,
  ) -> &mut Self {
    self.queue(move |world: &mut World| {
      let entity = world.spawn_empty().id();
      build.apply(input, world, entity);
    });
    self
  }
}
