use mica::prelude::*;

fn main() {
  App::new()
    .add_plugins(GamePlugin)
    .add_plugins(sync::plugin)
    .add_plugins(level::plugin)
    .add_systems(Startup, setup)
    .add_systems(Update, camera::movement)
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(Camera2d);

  let texture: Handle<Image> = asset_server.load("tiles/default.png");

  let tilemap = commands.spawn(Name::new("Tilemap")).id();

  let size = core::tilemap::SIZE;
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

      if rand::random_ratio(1, 5) {
        commands
          .entity(tile_entity)
          .insert(level::Wall)
          .insert(TileColor::from(Color::BLACK));
      }

      storage.set(&position, tile_entity);
      commands.entity(tilemap).add_child(tile_entity);
    }
  }

  let (map_type, tile_size) =
    (TilemapType::Square, TilemapTileSize { x: 32.0, y: 32.0 });
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
