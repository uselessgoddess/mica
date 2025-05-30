mod bullet;

use crate::{
  level::{
    SpawnSet,
    turret::{Cooldown, FollowTarget, Fov, Target},
  },
  prelude::*,
};

pub fn plugin(app: &mut App) {
  app
    .register_type::<Rifle>()
    .register_type::<Burst>()
    .add_plugins(bullet::plugin)
    .add_systems(Update, (spawn.in_set(SpawnSet), attack));
}

#[derive(Component, Reflect)]
struct Burst {
  cooldown: Timer,
  amount: usize,
  remain: usize,
}

impl Burst {
  pub fn new(amount: usize, cooldown: Timer) -> Self {
    Self { cooldown, amount, remain: amount }
  }

  pub fn reset(&mut self) {
    self.remain = self.amount;
    self.cooldown.reset();
  }
}

impl Default for Burst {
  fn default() -> Self {
    Self::new(10, timer::repeat(1.0 / 30.0))
  }
}

#[derive(Component, Reflect)]
#[require(super::Turret, Burst)]
pub struct Rifle {
  pub cooldown: Timer,
}

impl Default for Rifle {
  fn default() -> Self {
    Self { cooldown: timer::repeat(1.0) }
  }
}

fn spawn(
  query: Query<Entity, Added<Rifle>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  for entity in query.iter() {
    let mesh = meshes.add(Circle::new(tilemap::TILE * 0.25));
    let material = materials.add(Color::srgb(0.75, 0.75, 0.75));
    commands
      .entity(entity)
      .insert(Name::new("Rifle"))
      .insert((Mesh2d(mesh), MeshMaterial2d(material)))
      .insert((Fov::new(5.0), FollowTarget { speed: 0.5 }));
  }
}

fn attack(
  mut turrets: Query<(
    &Transform2D,
    &Target,
    &Cooldown,
    Option<&Fov>,
    (&mut Rifle, &mut Burst),
  )>,
  mut commands: Commands,
  time: Res<Time>,
) {
  for (
    &transform,
    &Target { entity, target, .. },
    cooldown,
    fov,
    (mut rifle, mut burst),
  ) in turrets.iter_mut()
  {
    let delta = time.delta();

    if let Some(fov) = fov
      && !fov.in_fov(transform, target)
    {
      continue;
    }

    if entity.is_none() || !cooldown.allow() {
      return;
    }

    if burst.remain > 0 && burst.cooldown.tick(delta).just_finished() {
      commands.spawn((
        transform,
        bullet::Bullet { damage: 1.0 },
        LinearVelocity(transform.up() * tilemap::TILE * 32.0),
      ));
      burst.remain -= 1;
    } else if rifle.cooldown.tick(delta).just_finished() {
      burst.reset();
    }
  }
}
