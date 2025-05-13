use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app.init_resource::<Registered>().add_systems(PreUpdate, clear);
}

// TODO: find a better way to track plugin register
#[derive(Resource, Default)]
struct Registered;

#[derive(Component)]
struct Shape;

fn clear(query: Query<Entity, With<Shape>>, mut commands: Commands) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}

pub struct Shapes<'a, 'w, 's>(pub &'a mut Commands<'w, 's>);

pub struct ShapeBuilder<'a, 'w, 's> {
  commands: &'a mut Commands<'w, 's>,
  path: Path,
  color: Color,
  width: f32,
}

impl<'a, 'w, 's> ShapeBuilder<'a, 'w, 's> {
  #[must_use]
  pub fn new<G: Geometry>(
    commands: &'a mut Commands<'w, 's>,
    shape: G,
  ) -> Self {
    Self {
      commands,
      path: GeometryBuilder::build_as(&shape),
      color: Color::BLACK,
      width: 1.0,
    }
  }

  pub fn color(mut self, color: Color) -> Self {
    self.color = color;
    self
  }

  pub fn width(mut self, width: f32) -> Self {
    self.width = width;
    self
  }

  pub fn build(self) -> Entity {
    let Self { commands, path, color, width } = self;

    commands.queue(|world: &mut World| {
      if world.get_resource::<Registered>().is_none() {
        error!("make sure the shapes::plugin is loaded before using `Shapes`");
      }
    });
    commands
      .spawn(Shape)
      .insert(ShapeBundle { path, ..default() })
      .insert(Stroke::new(color, width))
      .id()
  }
}

impl<'a, 'w, 's> Shapes<'a, 'w, 's> {
  #[must_use]
  pub fn line(
    &'a mut self,
    start: Vec2,
    end: Vec2,
  ) -> ShapeBuilder<'a, 'w, 's> {
    ShapeBuilder::new(self.0, shapes::Line(start, end))
  }
}
