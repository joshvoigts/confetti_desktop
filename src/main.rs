#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::action::*;
use crate::control::*;
use crate::screenshot::Screenshot;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window, WindowMode};
// use bevy_inspector_egui::quick::ResourceInspectorPlugin;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use std::env::consts::OS;

mod action;
mod control;
mod p;
mod screenshot;

fn main() {
   App::new()
      .add_plugins(DefaultPlugins)
      .add_plugins(
         RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
      )
      // .add_plugins(RapierDebugRenderPlugin::default())
      .add_systems(
         Startup,
         (setup, setup_physics, setup_materials_and_meshes).chain(),
      )
      .add_systems(Update, (mouse_input, handle_mouse_left))
      .add_event::<MouseLeftEvent>()
      .insert_resource(Screenshot::capture())
      .init_resource::<Screenshot>()
      // .register_type::<Screenshot>()
      // .add_plugins(ResourceInspectorPlugin::<Screenshot>::default())
      // .add_plugins(WorldInspectorPlugin::new())
      .run();
}

fn setup(
   mut commands: Commands,
   asset_server: Res<AssetServer>,
   screenshot: Res<Screenshot>,
   mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
   let mut window = window_query.get_single_mut().unwrap();
   window.mode = WindowMode::BorderlessFullscreen;

   commands.spawn(SpriteBundle {
      transform: Transform::from_scale(Vec3::splat(
         1.0 / window.scale_factor(),
      )),
      texture: asset_server.load(screenshot.path.clone()),
      ..default()
   });

   commands.spawn((MainCamera, Camera2dBundle::default()));
}

fn setup_physics(
   mut commands: Commands,
   windows: Query<&Window, With<PrimaryWindow>>,
   screenshot: Res<Screenshot>,
) {
   let window = windows.single();
   // let (camera, camera_transform) = cameras.single();

   let (win_width, win_height) = match OS {
      "macos" => (screenshot.width, screenshot.height),
      _ => (
         screenshot.width / window.scale_factor(),
         screenshot.height / window.scale_factor(),
      ),
   };

   // Debug
   //
   // p!(window.resolution.base_scale_factor());
   // p!(window.resolution.scale_factor());
   // p!(window.resolution.physical_width());
   // p!(window.resolution.physical_height());
   // p!("---");

   // p!(window.width());
   // p!(screenshot.width);
   // p!(win_width);
   // p!("---");

   // p!(window.height());
   // p!(screenshot.height);
   // p!(win_height);
   // p!("---");

   // p!(window.scale_factor());
   // p!(screenshot.scale);

   // Physics boundaries
   // Bottom
   commands.spawn((
      Collider::cuboid(win_width / 2.0, 50.0),
      TransformBundle::from(Transform::from_xyz(
         0.0,
         0.0 - (win_height / 2.0) - 50.0 + 2.0,
         0.0,
      )),
   ));
   // Left
   commands.spawn((
      Collider::cuboid(50.0, win_height / 2.0),
      TransformBundle::from(Transform::from_xyz(
         0.0 - (win_width / 2.0) - 50.0 + 2.0,
         0.0,
         0.0,
      )),
   ));
   // Right
   commands.spawn((
      Collider::cuboid(50.0, win_height),
      TransformBundle::from(Transform::from_xyz(
         (win_width / 2.0) + 50.0 - 2.0,
         0.0,
         0.0,
      )),
   ));
}

fn setup_materials_and_meshes(
   mut commands: Commands,
   mut meshes: ResMut<Assets<Mesh>>,
) {
   commands.insert_resource(Meshes {
      ball: meshes.add(Circle::new(50.0)).into(),
   });
}
