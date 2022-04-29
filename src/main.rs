mod wfc;

use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(HelloPlugin)
    .add_startup_system(setup)
    .run();
}

impl Plugin for HelloPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
      .add_startup_system(add_people)
      .add_system(greet_people);
  }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands.spawn_bundle(SpriteBundle {
    texture: asset_server.load("tiles/Grass.png"),
    transform: Transform::from_xyz(0.0, 0.0, 0.0),
    ..default()
  });
  commands.spawn_bundle(SpriteBundle {
    texture: asset_server.load("tiles/Grass.png"),
    transform: Transform::from_xyz(32.0, 0.0, 0.0),
    ..default()
  });
}

fn add_people(mut commands: Commands) {
  commands
    .spawn()
    .insert(Person)
    .insert(Name("Harry".to_string()));
  commands
    .spawn()
    .insert(Person)
    .insert(Name("Snarry".to_string()));
  commands
    .spawn()
    .insert(Person)
    .insert(Name("Plarry".to_string()));
}

struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
  if timer.0.tick(time.delta()).just_finished() {
    for name in query.iter() {
      println!("Hello {}!", name.0);
    }
  }
}
