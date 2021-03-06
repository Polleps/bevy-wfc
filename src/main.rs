mod wfc;

use bevy::{
  diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
  prelude::*,
  render::camera::ScalingMode,
  window::PresentMode,
};
use wfc::tile_map::TileMap;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Tile;

pub struct HelloPlugin;

struct RegenKey {
  pressed: bool,
}

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const RESOLUTION: f32 = 900.0;

const MAP_HEIGHT: f32 = 50.0;
const MAP_WIDTH: f32 = 50.0;

fn main() {
  App::new()
    .insert_resource(ClearColor(CLEAR))
    .insert_resource(WindowDescriptor {
      width: RESOLUTION * ASPECT_RATIO,
      height: RESOLUTION,
      // present_mode: PresentMode::Fifo,
      ..Default::default()
    })
    .insert_resource(RegenKey { pressed: false })
    .add_plugins(DefaultPlugins)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .insert_resource(TileMap::new(
      MAP_WIDTH.floor() as i32,
      MAP_HEIGHT.floor() as i32,
      wfc::tile_type::TileType::default_rules(),
    ))
    .add_startup_system(spawn_camera)
    .add_startup_system(build_map)
    .add_system(draw_map)
    .add_system(rebuild_map)
    .add_system(move_camera)
    .run();
}

fn spawn_camera(mut commands: Commands) {
  let mut camera = OrthographicCameraBundle::new_2d();

  camera.orthographic_projection.bottom = 0.0;
  camera.orthographic_projection.top = RESOLUTION;
  camera.orthographic_projection.left = 0.0;
  camera.orthographic_projection.right = RESOLUTION * ASPECT_RATIO;
  // camera.orthographic_projection.window_origin = WindowOrigin::BottomLeft;
  camera.orthographic_projection.scaling_mode = ScalingMode::None;
  commands.spawn_bundle(camera);
}

fn move_camera(
  mut camera_q: Query<&mut Transform, With<Camera>>,
  keys: Res<Input<KeyCode>>,
  time: Res<Time>,
) {
  let mut t = camera_q.single_mut();
  let speed = 200.0;
  if keys.pressed(KeyCode::W) {
    t.translation.y += speed * time.delta_seconds();
  }
  if keys.pressed(KeyCode::A) {
    t.translation.x -= speed * time.delta_seconds();
  }
  if keys.pressed(KeyCode::S) {
    t.translation.y -= speed * time.delta_seconds();
  }
  if keys.pressed(KeyCode::D) {
    t.translation.x += speed * time.delta_seconds();
  }
}

fn build_map(mut map: ResMut<TileMap>) {
  map.generate();
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
  } else {
    if !r_pressed {
      regen_key.pressed = false;
    }
  }
}

fn draw_map(
  map: ResMut<TileMap>,
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  old_tiles_q: Query<Entity, With<Tile>>,
) {
  let should_redraw = map.is_changed();

  if !should_redraw {
    return;
  };

  for entity in (&old_tiles_q).iter() {
    commands.entity(entity).despawn();
  }

  for (position, tile) in map.tiles.iter() {
    match tile {
      wfc::cell::Cell::Superposition(_) => continue,
      wfc::cell::Cell::Collapsed(tile_type) => {
        commands
          .spawn_bundle(SpriteBundle {
            texture: asset_server.load(&wfc::tile_type::TileType::get_texture(tile_type)),
            transform: Transform::from_xyz(
              position.x as f32 * 32.0 + 16.0,
              position.y as f32 * 32.0 + 16.0,
              0.0,
            ),
            ..default()
          })
          .insert(Tile);
      }
    }
  }
}
