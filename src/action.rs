use crate::control::MouseLeftEvent;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::sprite::Mesh2dHandle;
use bevy_rapier2d::prelude::*;
use rand::prelude::random;

#[derive(Resource)]
pub struct Meshes {
   pub ball: Mesh2dHandle,
}

#[derive(Component)]
struct Ball;

pub fn handle_mouse_left(
   mut commands: Commands,
   meshes: Res<Meshes>,
   mut materials: ResMut<Assets<ColorMaterial>>,
   mut event: EventReader<MouseLeftEvent>,
) {
   for ev in event.read() {
      let position = ev.0;
      let jitter_x = random::<f32>();
      let jitter_y = random::<f32>();
      commands.spawn((
         Ball,
         MaterialMesh2dBundle {
            mesh: meshes.ball.clone(),
            material: materials.add(Color::srgb(
               random::<f32>(),
               random::<f32>(),
               random::<f32>(),
            )),
            transform: Transform {
               translation: vec3(
                  position.x + jitter_x,
                  position.y + jitter_y,
                  999.0,
               ),
               scale: Vec3::splat(random::<f32>() + 0.1),
               ..default()
            },
            ..default()
         },
         RigidBody::Dynamic,
         Collider::ball(50.0),
         Restitution::coefficient(0.7),
      ));
   }
}
