use crate::prelude::*;

pub fn dev() -> bool {
  cfg!(debug_assertions)
}

#[derive(
  Debug, Resource, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd,
)]
#[non_exhaustive]
pub enum D {
  #[default]
  None = 0,
  L1 = 1,
  L2 = 2,
  L3 = 3,
}

pub fn in_debug(level: D) -> impl FnMut(Option<Res<D>>) -> bool + Clone {
  move |debug: Option<Res<D>>| {
    level <= debug.as_deref().copied().unwrap_or(D::None)
  }
}

pub trait AppExt {
  fn is_debug(&self, level: D) -> bool;
}

impl AppExt for App {
  fn is_debug(&self, level: D) -> bool {
    level <= *self.world().resource::<D>()
  }
}
