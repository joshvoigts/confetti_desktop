#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::action::*;
use crate::control::*;
use crate::modal::*;
use crate::screenshot::Screenshot;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window, WindowMode};
use bevy::winit::WinitWindows;
use bevy_rapier2d::prelude::*;

mod action;
mod control;
mod modal;
mod p;
mod screenshot;

#[cfg(target_os = "macos")]
mod macos;

fn main() {
   if let Some(screenshot) = Screenshot::capture() {
      App::new()
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
            (setup_camera, setup_physics, setup_materials_and_meshes),
         )
         .add_systems(
            Update,
            (
               keyboard_input,
               mouse_input,
               handle_confetti,
               handle_clear,
               handle_modal,
               handle_modal_button,
               // handle_too_much_confetti,
            ),
         )
         .add_event::<ConfettiEvent>()
         .add_event::<ClearEvent>()
         .add_event::<ModalEvent>()
         .insert_resource(screenshot)
         .init_resource::<Screenshot>()
         .init_resource::<BallCount>()
         .run();
   }
}

fn setup_camera(mut commands: Commands) {
   commands.spawn((
      MainCamera,
      Camera2dBundle {
         projection: OrthographicProjection {
            far: 1000.0,
            near: -1000.0,
            viewport_origin: Vec2::new(0.0, 0.5),
            ..default()
         },
         ..default()
      },
   ));
}

fn setup_physics(
   mut commands: Commands,
   asset_server: Res<AssetServer>,
   screenshot: Res<Screenshot>,
   windows: Query<(Entity, &Window), With<PrimaryWindow>>,
   winit_windows: NonSend<WinitWindows>,
) {
   let (window_id, window) = windows.single();

   // Bevy reports the wrong window size in full screen mode
   // so let's calculate it ourselves.
   let winit_win = winit_windows.get_window(window_id).unwrap();
   let win_size = winit_win.inner_size();
   let scale_factor = winit_win.scale_factor() as f32;
   let win_width = win_size.width as f32 / scale_factor;
   let win_height = win_size.height as f32 / scale_factor;

   // Physics boundaries
   // Bottom
   commands.spawn((
      Collider::cuboid(win_width, 2.0),
      TransformBundle::from(Transform::from_xyz(
         0.0,
         0.0 - (win_height / 2.0) + 2.0,
         0.0,
      )),
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

   // Spawn background
   commands.spawn(SpriteBundle {
      transform: Transform {
         rotation: Quat::IDENTITY,
         scale: Vec3::splat(
            1.0 / window.resolution.base_scale_factor(),
         ),
         translation: Vec3::new(win_width / 2.0, 0.0, 0.0),
      },
      texture: asset_server.load(screenshot.path.clone()),
      ..default()
   });
}

fn setup_materials_and_meshes(
   mut commands: Commands,
   mut meshes: ResMut<Assets<Mesh>>,
) {
   commands.insert_resource(Meshes {
      ball: meshes.add(Circle::new(50.0)).into(),
   });
}
