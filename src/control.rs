use bevy::app::AppExit;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct MainCamera;

#[derive(Event)]
pub struct MouseLeftEvent(pub Vec2);

#[derive(Event)]
pub struct DeleteEvent;

pub fn mouse_input(
   mut event: EventWriter<MouseLeftEvent>,
   buttons: Res<ButtonInput<MouseButton>>,
   cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
   windows: Query<&Window, With<PrimaryWindow>>,
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

pub fn keyboard_input(
   keyboard: Res<ButtonInput<KeyCode>>,
   mut exit: EventWriter<AppExit>,
   mut delete: EventWriter<DeleteEvent>,
) {
   if keyboard.pressed(KeyCode::Escape)
      || keyboard.pressed(KeyCode::KeyQ)
      || (keyboard.pressed(KeyCode::KeyC)
         && keyboard.any_pressed([
            KeyCode::ControlLeft,
            KeyCode::ControlRight,
         ]))
   {
      exit.send(AppExit::Success);
   }

   if keyboard.pressed(KeyCode::Backspace) {
      delete.send(DeleteEvent);
   }
}
