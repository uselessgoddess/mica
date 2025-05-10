use crate::{
  level::{
    Damage, Enemy,
    turret::{MonitorTargets, Target},
  },
  prelude::*,
};

pub fn plugin(app: &mut App) {
  app.register_type::<Laser>().add_systems(Update, (spawn, attack));
}

#[derive(Component, Reflect)]
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
    commands.entity(entity).insert((Mesh2d(mesh), MeshMaterial2d(material)));
  }
}

fn attack(
  turrets: Query<(&Transform2D, &MonitorTargets), With<Laser>>,
  enemies: Query<&Transform2D, With<Enemy>>,
  mut commands: Commands,
  mut gizmos: Gizmos,
  time: Res<Time>,
) {
  let damage = Damage(10.0 * time.delta_secs());

  for (from, monitor) in turrets.iter() {
    if let Some(Target { entity: Some(target), .. }) = monitor.first().copied()
      && let Ok(to) = enemies.get(target)
    {
      commands.entity(target).trigger(damage);
      gizmos.line_2d(
        from.translation,
        to.translation,
        Color::srgb(0.0, 1.0, 0.0),
      );
    }
  }
}
