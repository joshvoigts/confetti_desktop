#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::action::*;
use crate::control::*;
use crate::screenshot::Screenshot;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window, WindowMode};
// use bevy_inspector_egui::quick::ResourceInspectorPlugin;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::winit::WinitWindows;
use bevy_rapier2d::prelude::*;
// use std::env::consts::OS;

mod action;
mod control;
mod p;
mod screenshot;

#[cfg(target_os = "macos")]
mod macos;

fn main() {
   if let Some(screenshot) = Screenshot::capture() {
      App::new()
         //          .add_plugins(DefaultPlugins)
         .insert_resource(ClearColor(Color::NONE))
         .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
               transparent: true,
               decorations: false,
               resizable: false,
               mode: WindowMode::BorderlessFullscreen,
               ..default()
            }),
            ..default()
         }))
         .add_plugins(
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
               100.0,
            ),
         )
         // .add_plugins(RapierDebugRenderPlugin::default())
         .add_systems(
            Startup,
            (setup, setup_physics, setup_materials_and_meshes)
               .chain(),
         )
         .add_systems(Update, (mouse_input, handle_mouse_left))
         .add_event::<MouseLeftEvent>()
         .insert_resource(screenshot)
         .init_resource::<Screenshot>()
         // .register_type::<Screenshot>()
         // .add_plugins(ResourceInspectorPlugin::<Screenshot>::default())
         // .add_plugins(WorldInspectorPlugin::new())
         .run();
   }
}

fn setup(
   mut commands: Commands,
   windows: Query<&Window, With<PrimaryWindow>>,
   asset_server: Res<AssetServer>,
) {
   let mut window = windows.single_mut();
   //    window.resolution.set_scale_factor_override(Some(1.0));
   //    window.mode = WindowMode::BorderlessFullscreen;
   //
   //    let scale_y = screenshot.height / window.physical_height() as f32;

   commands.spawn((
      MainCamera,
      Camera2dBundle {
         projection: OrthographicProjection {
            far: 1000.0,
            near: -1000.0,
            viewport_origin: Vec2::new(0.0, 0.0),
            ..default()
         },
         ..default()
      },
   ));

   // Spawn background
   commands.spawn(SpriteBundle {
      transform: Transform {
         rotation: Quat::IDENTITY,
         scale: Vec3::splat(
            1.0 / window.resolution.base_scale_factor(),
         ),
         translation: Vec3::new(
            win_width / 2.0,
            win_height / 2.0,
            0.0,
         ),
      },
      texture: asset_server.load(screenshot.path.clone()),
      ..default()
   });
}

fn setup_physics(
   mut commands: Commands,
   winit_windows: NonSend<WinitWindows>,
   windows: Query<(Entity, &Window), With<PrimaryWindow>>,
   //    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
   //    screenshot: Res<Screenshot>,
) {
   let (window_id, window) = windows.single();
   //    let (camera, camera_transform) = cameras.single();

   // Bevy reports the wrong window size in full screen mode
   // so let's calculate it ourselves.
   let winit_win = winit_windows.get_window(window_id).unwrap();
   let win_size = winit_win.inner_size();
   let scale_factor = winit_win.scale_factor() as f32;
   let win_width = win_size.width as f32 / scale_factor;
   let win_height = win_size.height as f32 / scale_factor;

   // Debug
   //
   // let right = camera
   //    .viewport_to_world_2d(
   //       camera_transform,
   //       Vec2::new(window.width() as f32, 0.0),
   //    )
   //    .unwrap()
   //    .x;

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

   // p!(right);

   // Physics boundaries
   // Bottom
   commands.spawn((
      Collider::cuboid(win_width, 2.0),
      TransformBundle::from(Transform::from_xyz(0.0, 0.0 + 2.0, 0.0)),
   ));
   // Left
   commands.spawn((
      Collider::cuboid(2.0, win_height - 4.0),
      TransformBundle::from(Transform::from_xyz(0.0 + 2.0, 0.0, 0.0)),
   ));
   // Right
   commands.spawn((
      Collider::cuboid(2.0, win_height - 4.0),
      TransformBundle::from(Transform::from_xyz(
         win_width - 2.0,
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
