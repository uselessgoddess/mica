use crate::{
  level::{
    Damage, Enemy,
    turret::{Cooldown, Fov, MonitorTargets, Target},
  },
  prelude::*,
};

pub fn plugin(app: &mut App) {
  app
    .register_type::<Laser>()
    .add_systems(PreUpdate, clear_laser)
    .add_systems(Update, (spawn, attack));
}

#[derive(Component, Reflect, Default)]
#[require(super::Turret)]
pub struct Laser;

fn spawn(
  query: Query<Entity, Added<Laser>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  for entity in query.iter() {
    let mesh = meshes.add(Circle::new(tilemap::TILE * 0.25));
    let material = materials.add(Color::srgb(0.0, 0.75, 0.25));
    commands
      .entity(entity)
      .insert(Name::new("Laser"))
      .insert((Mesh2d(mesh), MeshMaterial2d(material)))
      .insert(Fov::new(15.0));
  }
}

fn clear_laser(turrets: Query<&Children, With<Laser>>, mut commands: Commands) {
  for children in turrets.iter() {
    for &child in children.iter() {
      commands.entity(child).despawn_recursive();
    }
  }
}

fn attack(
  turrets: Query<(&Transform2D, &MonitorTargets, &Cooldown), With<Laser>>,
  enemies: Query<&Transform2D, With<Enemy>>,
  mut commands: Commands,
  time: Res<Time>,
) {
  let damage = Damage(10.0 * time.delta_secs());

  for (from, monitor, cooldown) in turrets.iter() {
    if let Some(Target { entity: Some(target), .. }) = monitor.first().copied()
      && let Ok(to) = enemies.get(target)
      && cooldown.allow()
    {
      commands.entity(target).trigger(damage);

      Shapes(&mut commands)
        .line(from.translation, to.translation)
        .color(Color::srgb(0.0, 5.0, 3.0))
        .width(0.5)
        .build();
    }
  }
}
