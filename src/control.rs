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
