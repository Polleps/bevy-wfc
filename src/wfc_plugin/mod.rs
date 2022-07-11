use bevy::{prelude::*, render::render_resource::TextureUsages};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_ecs_tilemap::prelude::*;

use crate::wfc::{cell::Cell, tile_map::TileMap, tile_rules::TileRules};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum AppState {
  Loading,
  Running,
}

pub struct WFCPlugin;

struct RegenKey {
  pressed: bool,
}

const MAP_HEIGHT: f32 = 64.0;
const MAP_WIDTH: f32 = 64.0;

impl Plugin for WFCPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(JsonAssetPlugin::<TileRules>::new(&["rules"]))
      .add_plugin(TilemapPlugin)
      .add_state(AppState::Loading)
      .insert_resource(RegenKey { pressed: false })
      .insert_resource(TileMap::new(
        MAP_WIDTH.floor() as i32,
        MAP_HEIGHT.floor() as i32,
        TileRules::empty(),
      ))
      .add_startup_system(load_rules)
      .add_startup_system(build_tile_map)
      .add_system_set(SystemSet::on_update(AppState::Loading).with_system(build_map))
      .add_system(draw_map)
      .add_system(rebuild_map)
      .add_system(set_texture_filters_to_nearest);
  }
}

fn load_rules(mut commands: Commands, asset_server: Res<AssetServer>) {
  let handle: Handle<TileRules> = asset_server.load("rules/default.rules");
  commands.insert_resource(handle);
}

fn build_map(
  mut tile_map: ResMut<TileMap>,
  handle: Res<Handle<TileRules>>,
  mut rules: ResMut<Assets<TileRules>>,
  mut state: ResMut<State<AppState>>,
) {
  if let Some(rules) = rules.remove(handle.id) {
    println!("{:?}", rules);

    tile_map.rules = rules;
    tile_map.generate();
    state.set(AppState::Running).unwrap();
  }
}

fn build_tile_map(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
  let texture_handle: Handle<Image> = asset_server.load("tiles/Tileset.png");

  let map_entity = commands.spawn().id();
  let mut map = Map::new(0u16, map_entity);

  let layer_settings = LayerSettings::new(
    MapSize(1, 1),
    ChunkSize(64, 64),
    TileSize(32.0, 32.0),
    TextureSize(160.0, 160.0),
  );

  let (mut layer_builder, layer_entity) =
    LayerBuilder::<TileBundle>::new(&mut commands, layer_settings, 0u16, 0u16);

  map.add_layer(&mut commands, 0u16, layer_entity);

  layer_builder.for_each_tiles_mut(|tile_entity, tile_data| {
    // True here refers to tile visibility.
    *tile_data = Some(TileBundle::default());
    // Tile entity might not exist at this point so you'll need to create it.
    if tile_entity.is_none() {
      *tile_entity = Some(commands.spawn().id());
    }
  });

  map_query.build_layer(&mut commands, layer_builder, texture_handle);

  let center = layer_settings.get_pixel_center();

  commands
    .entity(map_entity)
    .insert(map)
    .insert(Transform::from_xyz(-center.x, -center.y, 0.0))
    .insert(GlobalTransform::default());
}

fn rebuild_map(
  mut map: ResMut<TileMap>,
  mut regen_key: ResMut<RegenKey>,
  keys: Res<Input<KeyCode>>,
) {
  let r_pressed = keys.just_pressed(KeyCode::R);

  if !regen_key.pressed {
    if r_pressed {
      map.clear();
      map.generate();
      regen_key.pressed = true;
    }
  } else if !r_pressed {
    regen_key.pressed = false;
  }
}

fn draw_map(mut commands: Commands, map: Res<TileMap>, mut map_query: MapQuery) {
  let should_redraw = map.is_changed();

  if !should_redraw {
    return;
  };
  println!("{:?}", map);

  for (pos, tile) in map.tiles.iter() {
    match tile {
      Cell::Superposition(_) => continue,
      Cell::Collapsed(tile_type) => {
        let tile_pos = TilePos(pos.x as u32, pos.y as u32);
        let _ = map_query.set_tile(
          &mut commands,
          tile_pos,
          Tile {
            texture_index: map.rules.get_tile_index(tile_type) as u16,
            ..Default::default()
          },
          0u16,
          0u16,
        );
        map_query.notify_chunk_for_tile(tile_pos, 0u16, 0u16);
      }
    }
  }
}

pub fn set_texture_filters_to_nearest(
  mut texture_events: EventReader<AssetEvent<Image>>,
  mut textures: ResMut<Assets<Image>>,
) {
  // quick and dirty, run this for all textures anytime a texture is created.
  for event in texture_events.iter() {
    if let AssetEvent::Created { handle } = event {
      if let Some(mut texture) = textures.get_mut(handle) {
        texture.texture_descriptor.usage =
          TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_SRC | TextureUsages::COPY_DST;
      }
    }
  }
}
