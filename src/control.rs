use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct MainCamera;

#[derive(Event)]
pub struct MouseLeftEvent(pub Vec2);

pub fn mouse_input(
   buttons: Res<ButtonInput<MouseButton>>,
   cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
   windows: Query<&Window, With<PrimaryWindow>>,
   mut event: EventWriter<MouseLeftEvent>,
) {
   if buttons.pressed(MouseButton::Left) {
      let (camera, camera_transform) = cameras.single();

      if let Some(position) = windows
         .single()
         .cursor_position()
         .and_then(|cursor| {
            camera.viewport_to_world(camera_transform, cursor)
         })
         .map(|ray| ray.origin.truncate())
      {
         event.send(MouseLeftEvent(position));
      }
   }
}

// pub fn keyboard_control(
//    keyboard_input: Res<Input<KeyCode>>,
//    mut camera_query: Query<&mut Transform, With<MainCamera>>,
//    timer: Res<Time>,
// ) {
//    if let Ok(mut camera) = camera_query.get_single_mut() {
// //       if keyboard_input.pressed(KeyCode::Space) {
// //          let speed = 25.0 * timer.delta_seconds();
// //          let trans = camera.up() * speed;
// //          camera.translation = camera.translation + trans;
// //       }
// //
// //       if keyboard_input.pressed(KeyCode::ShiftLeft) {
// //          let speed = 25.0 * timer.delta_seconds();
// //          let trans = camera.down() * speed;
// //          camera.translation = camera.translation + trans;
// //       }
// //
// //       if keyboard_input.pressed(KeyCode::W) {
// //          let speed = 25.0 * timer.delta_seconds();
// //          let trans = camera.forward() * speed;
// //          camera.translation = camera.translation + trans;
// //       }
// //
// //       if keyboard_input.pressed(KeyCode::S) {
// //          let speed = 25.0 * timer.delta_seconds();
// //          let trans = camera.back() * speed;
// //          camera.translation = camera.translation + trans;
// //       }
// //
// //       if keyboard_input.pressed(KeyCode::D) {
// //          let turn =
// //             Quat::from_rotation_y(-2.0 * timer.delta_seconds());
// //          camera.rotation = camera.rotation * turn;
// //       }
// //
// //       if keyboard_input.pressed(KeyCode::A) {
// //          let turn =
// //             Quat::from_rotation_y(2.0 * timer.delta_seconds());
// //          camera.rotation = camera.rotation * turn;
// //       }
// //
// //       if keyboard_input.pressed(KeyCode::Q) {
// //          let speed = 25.0 * timer.delta_seconds();
// //          let trans = camera.left() * speed;
// //          camera.translation = camera.translation + trans;
// //       }
// //
// //       if keyboard_input.pressed(KeyCode::E) {
// //          let speed = 25.0 * timer.delta_seconds();
// //          let trans = camera.right() * speed;
// //          camera.translation = camera.translation + trans;
// //       }
//    }
// }
