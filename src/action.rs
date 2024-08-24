use crate::control::DeleteEvent;
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
pub struct Ball;

#[derive(Default, Resource)]
pub struct BallCount(usize);

pub fn handle_mouse_left(
   mut ball_count: ResMut<BallCount>,
   mut commands: Commands,
   mut event: EventReader<MouseLeftEvent>,
   mut materials: ResMut<Assets<ColorMaterial>>,
   meshes: Res<Meshes>,
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
      ball_count.0 += 1;
   }
}

// pub fn handle_too_much_confetti(
//    mut ball_count: ResMut<BallCount>,
//    mut commands: Commands,
//    ball_ids: Query<Entity, With<Ball>>,
// ) {
//    for ball_id in ball_ids.iter().choose() {
//       if ball_count.0 > 10 {
//          commands.entity(ball_id).despawn();
//          ball_count.0 -= 1;
//       } else {
//          break;
//       }
//    }
// }

pub fn handle_delete(
   mut commands: Commands,
   mut event: EventReader<DeleteEvent>,
   ball_ids: Query<Entity, With<Ball>>,
) {
   if !event.is_empty() {
      for ball_id in ball_ids.iter() {
         commands.entity(ball_id).despawn();
      }
      event.clear();
   }
}
