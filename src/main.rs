use crate::action::*;
use crate::control::*;
use crate::pkv::Settings;
use crate::screenshot::Screenshot;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window, WindowMode};
use bevy_rapier2d::prelude::*;

mod action;
mod control;
mod p;
mod pkv;
mod screenshot;

fn main() {
   if let Ok(mut settings) = Settings::load() {
      if settings.first_run {
         p!("Skipping first run. Ensure screenshot permissions \
            have been granted and run again.");
         settings.first_run = false;
         let _ = settings.save();
         return;
      }
   }

   App::new()
      .add_plugins(DefaultPlugins)
      .add_plugins(
         RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
      )
      // .add_plugins(RapierDebugRenderPlugin::default())
      .add_systems(
         Startup,
         (setup_main_window, setup, setup_materials_and_meshes)
            .chain(),
      )
      .add_systems(Update, (mouse_input, handle_mouse_left))
      .add_event::<MouseLeftEvent>()
      .insert_resource(Screenshot::capture())
      .init_resource::<Screenshot>()
      .run();
}

fn setup_main_window(
   mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
   let mut window = windows.single_mut();
   window.mode = WindowMode::BorderlessFullscreen;
}

fn setup(
   mut commands: Commands,
   asset_server: Res<AssetServer>,
   screenshot: Res<Screenshot>,
) {
   commands.spawn(SpriteBundle {
      transform: Transform::from_scale(Vec3::splat(screenshot.scale)),
      texture: asset_server.load(screenshot.path.clone()),
      ..default()
   });

   // Main camera
   commands.spawn((MainCamera, Camera2dBundle::default()));

   // Physics boundaries
   commands.spawn((
      Collider::cuboid(screenshot.width - 2.0, 50.0),
      TransformBundle::from(Transform::from_xyz(
         0.0,
         0.0 - screenshot.height - 50.0 + 2.0,
         0.0,
      )),
   ));
   commands.spawn((
      Collider::cuboid(50.0, screenshot.height - 2.0),
      TransformBundle::from(Transform::from_xyz(
         0.0 - screenshot.width - 50.0 + 2.0,
         0.0,
         0.0,
      )),
   ));
   commands.spawn((
      Collider::cuboid(50.0, screenshot.height - 2.0),
      TransformBundle::from(Transform::from_xyz(
         screenshot.width + 50.0 - 2.0,
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
