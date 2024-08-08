use crate::action::*;
use crate::control::*;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window, WindowMode};
use bevy_rapier2d::prelude::*;

mod action;
mod control;
mod p;

use std::env;
use xcap::Monitor;

fn main() {
   App::new()
      .add_plugins(DefaultPlugins)
      .add_plugins(
         RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
      )
      .add_plugins(RapierDebugRenderPlugin::default())
      .add_systems(
         Startup,
         (setup, (setup_sprites, setup_materials_and_meshes)).chain(),
      )
      .add_systems(Update, (mouse_input, handle_mouse_left))
      .add_event::<MouseLeftEvent>()
      .run();
}

fn setup(
   mut commands: Commands,
   mut windows: Query<&mut Window, With<PrimaryWindow>>,
   asset_server: Res<AssetServer>,
) {
   let mut window = windows.single_mut();
   window.mode = WindowMode::BorderlessFullscreen;

   // Screenshot background
   let monitors = Monitor::all().unwrap();
   let monitor = monitors.iter().find(|&m| m.is_primary()).unwrap();

   let image = monitor.capture_image().unwrap();
   let mut path_buf = env::temp_dir();
   path_buf.push("tmp_screenshot.png");
   let path = path_buf.to_string_lossy().into_owned();
   image.save(path.clone()).unwrap();

   let scale = 1.0 / monitor.scale_factor();
   let width = (monitor.width() as f32) / monitor.scale_factor();
   let height = (monitor.height() as f32) / monitor.scale_factor();

   commands.spawn(SpriteBundle {
      transform: Transform::from_scale(Vec3::splat(scale)),
      texture: asset_server.load(path),
      ..default()
   });

   // Main camera
   commands.spawn((MainCamera, Camera2dBundle::default()));

   // Physics boundaries
   commands.spawn((
      Collider::cuboid(width - 2.0, 50.0),
      TransformBundle::from(Transform::from_xyz(
         0.0,
         0.0 - height - 50.0 + 2.0,
         0.0,
      )),
   ));
   commands.spawn((
      Collider::cuboid(50.0, height - 2.0),
      TransformBundle::from(Transform::from_xyz(
         0.0 - width - 50.0 + 2.0,
         0.0,
         0.0,
      )),
   ));
   commands.spawn((
      Collider::cuboid(50.0, height - 2.0),
      TransformBundle::from(Transform::from_xyz(
         width + 50.0 - 2.0,
         0.0,
         0.0,
      )),
   ));
}

fn setup_materials_and_meshes(
   mut commands: Commands,
   mut meshes: ResMut<Assets<Mesh>>,
   mut materials: ResMut<Assets<ColorMaterial>>,
) {
   commands.insert_resource(Meshes {
      ball: meshes.add(Circle::new(50.0)).into(),
   });

   commands.insert_resource(Materials {
      green: materials.add(Color::srgb(0.0, 0.5, 0.0)),
   });
}

fn setup_sprites(
   mut commands: Commands,
   asset_server: Res<AssetServer>,
   mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
   let texture = asset_server.load("destruction.png");
   let layout = TextureAtlasLayout::from_grid(
      UVec2::splat(100),
      7,
      1,
      None,
      None,
   );
   let texture_atlas_layout = texture_atlas_layouts.add(layout);
   commands.insert_resource(SpriteSheets {
      destruct: (texture, texture_atlas_layout),
   });
}
