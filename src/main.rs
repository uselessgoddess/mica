use mica::prelude::*;

fn main() {
  App::new()
    .add_plugins(GamePlugin)
    .add_plugins(sync::plugin)
    .add_plugins(level::plugin)
    .add_systems(Startup, (setup, setup_tilemap))
    .add_systems(Update, camera::movement)
    .insert_resource(D::L1)
    .run();
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2d);

  let center = tilemap::center();
  commands.spawn((level::Core, center));

  commands.spawn((level::Turret, TilePos { x: center.x + 1, ..center }));
  commands.spawn((level::Turret, TilePos { x: center.x - 1, ..center }));
  commands.spawn((level::Turret, TilePos { y: center.y + 1, ..center }));
  commands.spawn((level::Turret, TilePos { y: center.y - 1, ..center }));
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
