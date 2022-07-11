mod wfc;
mod wfc_plugin;

use bevy::{
  diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
  prelude::*,
  render::camera::ScalingMode,
  window::PresentMode,
};

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const RESOLUTION: f32 = 900.0;
#[derive(Component, Debug)]
struct Speed(f32);

fn main() {
  App::new()
    .insert_resource(ClearColor(CLEAR))
    .insert_resource(WindowDescriptor {
      width: RESOLUTION * ASPECT_RATIO,
      height: RESOLUTION,
      present_mode: PresentMode::Mailbox,
      ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(wfc_plugin::WFCPlugin)
    .add_startup_system(spawn_camera)
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
  commands.spawn_bundle(camera).insert(Speed(200.0));
  println!("SPAWNING CAMERA");
}

fn move_camera(
  mut camera_q: Query<(&mut Transform, &Speed), With<Camera>>,
  keys: Res<Input<KeyCode>>,
  time: Res<Time>,
) {
  let mut t = camera_q.single_mut();
  let mut speed = t.1 .0;

  if keys.pressed(KeyCode::LShift) {
    speed *= 2.0;
  }
  if keys.pressed(KeyCode::W) {
    t.0.translation.y += speed * time.delta_seconds();
  }
  if keys.pressed(KeyCode::A) {
    t.0.translation.x -= speed * time.delta_seconds();
  }
  if keys.pressed(KeyCode::S) {
    t.0.translation.y -= speed * time.delta_seconds();
  }
  if keys.pressed(KeyCode::D) {
    t.0.translation.x += speed * time.delta_seconds();
  }
}
