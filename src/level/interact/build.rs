use crate::{
  level::{FollowMouse, turret::rocket},
  prelude::{ecs::Build, *},
};

pub fn plugin(app: &mut App) {
  app.add_systems(Update, (input, highlight, spawn));
}

#[derive(Component, Deref)]
#[require(FollowMouse)]
pub struct Select<I>(Option<Box<dyn Build<Input = I>>>);

impl<I> Select<I> {
  pub fn new(build: impl Build<Input = I>) -> Self {
    Self(Some(Box::new(build)))
  }

  pub fn assume(&mut self) -> Box<dyn Build<Input = I>> {
    self.0.take().expect(
      "this is method is safe, \
       but allow only single read",
    )
  }
}

type Selected = Select<TilePos>;

fn input(
  select: Option<Single<Entity, With<Selected>>>,
  input: Res<ButtonInput<KeyCode>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  if input.just_pressed(KeyCode::Digit1) {
    if let Some(select) = select.map(Single::into_inner) {
      commands.entity(select).despawn_recursive();
    }

    let mesh = meshes.add(Circle::new(tilemap::TILE * 0.5));
    let material = materials.add(Color::srgb(1.0, 1.0, 1.0));

    commands
      .spawn(Selected::new(rocket::Build))
      .insert((Mesh2d(mesh), MeshMaterial2d(material)));
  }
}

fn highlight() {}

fn spawn(
  select: Option<Single<(Entity, &sync::Pos, &mut Selected)>>,
  input: Res<ButtonInput<MouseButton>>,
  mut commands: Commands,
) {
  if input.just_pressed(MouseButton::Left)
    && let Some((entity, &pos, mut select)) = select.map(Single::into_inner)
  {
    commands.entity(entity).despawn_recursive();
    commands.spawn_dynamic(TilePos::from(pos), select.assume());
  }
}
