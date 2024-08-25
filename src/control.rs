use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct MainCamera;

#[derive(Event)]
pub struct ConfettiEvent(pub Vec2);

#[derive(Event, Default)]
pub struct ClearEvent;

#[derive(Event, Default)]
pub struct ModalEvent;

pub fn mouse_input(
   mut event: EventWriter<ConfettiEvent>,
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
         event.send(ConfettiEvent(position));
      }
   }
}

pub fn keyboard_input(
   keyboard: Res<ButtonInput<KeyCode>>,
   mut clear: EventWriter<ClearEvent>,
   mut modal: EventWriter<ModalEvent>,
) {
   if keyboard.any_pressed([
      KeyCode::Backspace,
      KeyCode::Delete,
      KeyCode::Escape,
      KeyCode::KeyC,
      KeyCode::KeyQ,
      KeyCode::KeyW,
   ]) {
      modal.send_default();
   }

   if keyboard.pressed(KeyCode::Space) {
      clear.send_default();
   }
}
