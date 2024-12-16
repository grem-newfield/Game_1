use crate::{
   Canvas, Enemy, InGameCamera, OuterCamera, Player, Projectile, ProjectileArt, Sprites,
   TestAttackTimer,
};
use avian2d::prelude::*;
use bevy::prelude::*;

const UP: Vec3 = Vec3::new(0.0, 0.0, 1.0);

// TODO: add binding and more keys?
pub fn player_input_system() {}

pub fn wasd_hardcoded_player_movemement(
   time: Res<Time>,
   keys: Res<ButtonInput<KeyCode>>,
   // mut q: Query<&mut Transform, With<Player>>,
   // mut p: Query<&Player>,
   mut transform: Single<&mut Transform, With<Player>>,
   mut player: Single<&Player>,
   mut exit: EventWriter<AppExit>,
) {
   // for mut velocity in &mut query {
   //         velocity.y -= 9.8 * DELTA;
   //
   // for (mut transform, player) in zip(&mut q, &p) {
   // for (mut transform, player) in zip(&mut q, &p) {
   // info!("{:?}", transform);
   // info!("{:?}", player);
   let mut move_vec = Vec3::ZERO;
   if keys.pressed(KeyCode::KeyQ) {
      exit.send(AppExit::Success);
   }
   if keys.pressed(KeyCode::KeyW) {
      // transform.translation.y += player.speed * time.delta_seconds();
      move_vec.y += 1.0; // player.speed * time.delta_secs();
   }
   if keys.pressed(KeyCode::KeyA) {
      // transform.translation.x -= player.speed * time.delta_seconds();
      move_vec.x -= 1.0; //player.speed * time.delta_secs();
      transform.rotate_z(time.delta_secs() * 3.);
   }
   if keys.pressed(KeyCode::KeyS) {
      // transform.translation.y -= player.speed * time.delta_seconds();
      move_vec.y -= 1.0; //player.speed * time.delta_secs();
   }
   if keys.pressed(KeyCode::KeyD) {
      // transform.translation.x += player.speed * time.delta_seconds();
      move_vec.x += 1.0; //player.speed * time.delta_secs();
      transform.rotate_z(-time.delta_secs() * 3.);
   }
   if move_vec != Vec3::ZERO {
      transform.translation += move_vec.normalize() * player.speed * time.delta_secs();
   }
   // }
}
pub fn setup_player(
   mut cmd: Commands,
   asset_server: Res<AssetServer>,
   sprites: Res<Sprites>,
   // mut windows: Query<&mut Window>,
) {
   // let window = windows.get_single_mut().unwrap();
   // let width = window.resolution.width() / 2.;
   // let height = window.resolution.height() / 2.;
   cmd.spawn((
      Name::new("Player"),
      Player { speed: 100.0 },
      sprites.player.clone(),
      Transform::from_xyz(0., 0., 0.).with_scale(Vec3::ONE),
   ));
}
pub fn follow_cam(
   // mut cam: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
   mut main_cam: Single<
      &mut Transform,
      (With<OuterCamera>, Without<Player>, Without<InGameCamera>),
   >,
   mut pixel_cam: Single<
      &mut Transform,
      (With<InGameCamera>, Without<Player>, Without<OuterCamera>),
   >,
   mut canvas: Single<
      &mut Transform,
      (With<Canvas>, Without<Player>, Without<OuterCamera>, Without<InGameCamera>),
   >,
   // target: Query<&Transform, (With<Player>, Without<MainCamera>)>,
   target: Single<&Transform, (With<Player>, Without<OuterCamera>, Without<InGameCamera>)>,
   time: Res<Time>,
) {
   // cam.translation = cam.translation.lerp(target.translation, (-700.0 * time.delta_secs()).exp2());
   // cam.translation = cam.translation.lerp(target.translation, 1. * time.delta_secs());
   main_cam.translation = target.translation;
   canvas.translation = target.translation;
   pixel_cam.translation = target.translation;
}
