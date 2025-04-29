use mica::{
  level::{facility, turret},
  prelude::*,
};

fn main() {
  App::new()
    .insert_resource(D::L3)
    .add_plugins(GamePlugin)
    .add_systems(Startup, (setup, setup_tilemap))
    .run();
}

use bevy::core_pipeline::bloom::Bloom;

fn setup(mut commands: Commands) {
  commands.spawn((
    Camera2d,
    (Camera { hdr: true, ..default() }, Bloom::OLD_SCHOOL),
    PanCam {
      grab_buttons: vec![MouseButton::Left, MouseButton::Middle],
      speed: 500.0,
      ..default()
    },
  ));

  let center = tilemap::center();
  commands.spawn((level::Core, center));

  commands.spawn((turret::Rocket::default(), TilePos { x: 16, y: 15 }));
  commands.spawn((facility::Designator, TilePos { x: 17, y: 15 }));
  commands.spawn((turret::Rocket::default(), TilePos { x: 18, y: 15 }));

  commands.spawn((turret::Laser, TilePos { y: center.y + 1, ..center }));
  commands.spawn((turret::Laser, TilePos { y: center.y - 1, ..center }));
}

fn setup_tilemap(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut next: ResMut<NextState<GameState>>,
) {
  next.set(GameState::Playing);

  let texture: Handle<Image> = asset_server.load("tiles/default.png");

  let tilemap = commands.spawn(Name::new("Tilemap")).id();

  let size = tilemap::SIZE;
  let mut storage = TileStorage::empty(size);

  for x in 0..size.x {
    for y in 0..size.y {
      let position = TilePos { x, y };
      let tile_entity = commands
        .spawn(TileBundle {
          position,
          tilemap_id: TilemapId(tilemap),
          ..Default::default()
        })
        .insert(TileColor::from(Color::srgb(0.75, 0.75, 0.75)))
        .id();

      if rand::random_ratio(1, 12) {
        commands
          .entity(tile_entity)
          .insert(level::Wall)
          .insert(TileColor::from(Color::BLACK));
      }

      storage.set(&position, tile_entity);
      commands.entity(tilemap).add_child(tile_entity);
    }
  }

  let (map_type, tile_size) = (TilemapType::Square, TilemapTileSize {
    x: tilemap::TILE,
    y: tilemap::TILE,
  });
  let grid_size = tile_size.into();

  commands.entity(tilemap).insert(TilemapBundle {
    size,
    storage,
    tile_size,
    grid_size,
    map_type,
    texture: TilemapTexture::Single(texture),
    transform: get_tilemap_center_transform(&size, &grid_size, &map_type, -1.0),
    ..default()
  });
}
