use crate::{
   get_sprite, player::components::*, Enemy, Projectile, ProjectileArt, SpritesCollection,
   TestAttackTimer,
};
use avian2d::prelude::*;
use bevy::{ecs::bundle, prelude::*, reflect::Map};

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
                         // transform.rotate_z(time.delta_secs() * 3.);
   }
   if keys.pressed(KeyCode::KeyS) {
      // transform.translation.y -= player.speed * time.delta_seconds();
      move_vec.y -= 1.0; //player.speed * time.delta_secs();
   }
   if keys.pressed(KeyCode::KeyD) {
      // transform.translation.x += player.speed * time.delta_seconds();
      move_vec.x += 1.0; //player.speed * time.delta_secs();
                         // transform.rotate_z(-time.delta_secs() * 3.);
   }
   if move_vec != Vec3::ZERO {
      transform.translation += move_vec.normalize() * player.speed * time.delta_secs();
   }
   // }
}
pub fn setup_player(
   mut cmd: Commands,
   asset_server: Res<AssetServer>,
   sprites: Res<SpritesCollection>,
   // mut windows: Query<&mut Window>,
) {
   // let window = windows.get_single_mut().unwrap();
   // let width = window.resolution.width() / 2.;
   // let height = window.resolution.height() / 2.;
   let (sprite, name) = get_sprite(&mut cmd, &sprites, "player");
   cmd.spawn((
      sprite,
      name,
      Player { speed: 100.0 },
      // Transform::from_xyz(0., 0., 0.).with_scale(Vec3::ONE),
      Transform::from_xyz(0., 0., 99.),
   ));
}
pub fn follow_cam(
   // mut cam: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
   mut main_cam: Single<&mut Transform, (With<MainCamera>, Without<Player>, Without<CanvasCamera>)>,
   mut pixel_cam: Single<
      &mut Transform,
      (With<CanvasCamera>, Without<Player>, Without<MainCamera>),
   >,
   mut canvas: Single<
      &mut Transform,
      (With<Canvas>, Without<Player>, Without<MainCamera>, Without<CanvasCamera>),
   >,
   // target: Query<&Transform, (With<Player>, Without<MainCamera>)>,
   target: Single<&Transform, (With<Player>, Without<MainCamera>, Without<CanvasCamera>)>,
   time: Res<Time>,
) {
   // cam.translation = cam.translation.lerp(target.translation, (-700.0 * time.delta_secs()).exp2());
   // cam.translation = cam.translation.lerp(target.translation, 1. * time.delta_secs());
   main_cam.translation = target.translation;
   canvas.translation = target.translation;
   pixel_cam.translation = target.translation;
}

pub fn setup_cursor(
   mut commands: Commands,
   sprites_collection: Res<SpritesCollection>,
) {
   let (sprite, name) = get_sprite(&mut commands, &sprites_collection, "cursor");
   commands.spawn((sprite, name, Transform::from_xyz(0.0, 0.0, 100.0), CursorTag));
}

pub fn draw_cursor(
   mut cmd: Commands,
   window: Single<&Window>,
   q: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
   mut cursor_sprite: Single<&mut Transform, With<CursorTag>>,
) {
   let (camera, camera_transform) = *q;
   let Some(cur_pos) = window.cursor_position() else {
      return;
   };
   let Ok(point) = camera.viewport_to_world_2d(camera_transform, cur_pos) else {
      return;
   };
   cursor_sprite.translation = Vec3::new(point.x, point.y, 100.0);

   // if let Some(cur_pos) = window.cursor_position()
   // .and_then(|cursor| cam.viewport_to_world_2d(cam_trans,cursor)).map(|ray| ray.);
}
