use {super::Percentage, crate::prelude::*, std::marker::PhantomData};

#[derive(Debug, Clone, Reflect)]
pub struct Settings {
  /// Configure the width of the bar
  pub width: f32,
  /// Configures the offset of the bar relative to the entity it's attached to.
  /// For horizontal bars, this is an offset along the y-axis, for vertical bars along the x-axis.
  pub offset: f32,
  pub height: BarHeight,
}

impl Settings {
  pub fn absolute_height(&self) -> f32 {
    match self.height {
      BarHeight::Relative(ratio) => ratio * self.width,
      BarHeight::Static(height) => height,
    }
  }
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      width: tilemap::TILE * 0.75,
      offset: tilemap::TILE * 0.25,
      height: Default::default(),
    }
  }
}

/// Describes the height of the bar
#[derive(Debug, Clone, Reflect)]
pub enum BarHeight {
  /// Bar height relative to its width
  Relative(f32),
  /// Static bar width
  Static(f32),
}

impl Default for BarHeight {
  fn default() -> Self {
    Self::Relative(0.10)
  }
}

#[derive(Debug, Clone, Reflect)]
#[non_exhaustive]
pub enum ForegroundColor {
  Static(Color),
}

#[derive(Debug, Clone, Resource, Reflect)]
pub struct ColorScheme<T: Percentage> {
  pub foreground: ForegroundColor,
  pub background: Color,
  #[reflect(ignore)]
  _marker: PhantomData<T>,
}

impl<T: Percentage> Default for ColorScheme<T> {
  fn default() -> Self {
    Self {
      foreground: ForegroundColor::Static(Color::srgba(1.0, 0.0, 0.0, 0.95)),
      background: Color::srgba(0.0, 0.0, 0.0, 0.75),
      _marker: PhantomData,
    }
  }
}
