use crate::{
   get_sprite, player::components::*, Action, AttackTimer, Enemy, EnemyProjectile, Health,
   PlayerMoveEvent, PowerUp, Projectile, ProjectileArt, SpritesCollection, TestAttack, XpOrb,
};
use avian2d::prelude::*;
use bevy::{ecs::bundle, prelude::*};
use leafwing_input_manager::prelude::*;

const UP: Vec3 = Vec3::new(0.0, 0.0, 1.0);

pub fn player_moves(
   mut ew: EventWriter<PlayerMoveEvent>,
   action_state: Single<&ActionState<Action>, With<Player>>,
) {
   let mut direction_vec = Vec2::ZERO;
   for input_dir in Action::DIRECTIONS {
      if action_state.pressed(&input_dir) {
         if let Some(direction) = input_dir.direction() {
            direction_vec += *direction;
         }
      }
   }
   let net_direction = Dir2::new(direction_vec);
   if let Ok(direction) = net_direction {
      ew.send(PlayerMoveEvent { direction });
   };
}

pub fn move_player(
   mut e_move: EventReader<PlayerMoveEvent>,
   time: Res<Time>,
   mut player_transform: Single<&mut Transform, With<Player>>,
   mut player: Single<&Player>,
) {
   for e in e_move.read() {
      let move_vec = e.direction * player.speed * time.delta_secs();
      player_transform.translation += Vec3::new(move_vec.x, move_vec.y, 0.0);
   }
}

pub fn player_move_old(
   time: Res<Time>,
   keys: Res<ButtonInput<KeyCode>>,
   // mut q: Query<&mut Transform, With<Player>>,
   // mut p: Query<&Player>,
   mut player_transform: Single<&mut Transform, With<Player>>,
   mut player: Single<&Player>,
   mut exit_event_writer: EventWriter<AppExit>,
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
      exit_event_writer.send(AppExit::Success);
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
      player_transform.translation += move_vec.normalize() * player.speed * time.delta_secs();
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

   let input_map = InputMap::new([
      (Action::Left, KeyCode::KeyA),
      (Action::Left, KeyCode::ArrowLeft),
      (Action::Right, KeyCode::KeyD),
      (Action::Right, KeyCode::ArrowRight),
      (Action::Up, KeyCode::KeyW),
      (Action::Up, KeyCode::ArrowUp),
      (Action::Down, KeyCode::KeyS),
      (Action::Down, KeyCode::ArrowDown),
   ]);

   let (sprite, name) = get_sprite(&mut cmd, &sprites, "player");
   cmd.spawn((
      sprite,
      name,
      Player {
         life: 100,
         speed: 80.0,
         last_position: Vec3::ZERO,
         damage_cooldown: Timer::from_seconds(1.0, TimerMode::Once),
      },
      // Transform::from_xyz(0., 0., 0.).with_scale(Vec3::ONE),
      Transform::from_xyz(0., 0., 99.),
      InputManagerBundle::with_map(input_map),
      RigidBody::Kinematic,
      Collider::circle(4.0),
      CollidingEntities::default(),
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
   {
      main_cam.translation = target.translation;
      canvas.translation = target.translation;
      pixel_cam.translation = target.translation;
   }
   return;

   let smooth_time = 0.3; // This controls how quickly the camera reaches the target. Lower values make it faster.

   let mut cam_velocity = Vec3::ZERO;
   let mut pixel_cam_velocity = Vec3::ZERO;
   let mut canvas_velocity = Vec3::ZERO;

   for (cam, velocity, target_pos) in [
      (&mut main_cam.translation, &mut cam_velocity, target.translation),
      (&mut pixel_cam.translation, &mut pixel_cam_velocity, target.translation),
      (&mut canvas.translation, &mut canvas_velocity, target.translation),
   ]
   .iter_mut()
   {
      cam.x = smooth_damp(cam.x, target_pos.x, &mut velocity.x, smooth_time, time.delta_secs());
      cam.y = smooth_damp(cam.y, target_pos.y, &mut velocity.y, smooth_time, time.delta_secs());
      cam.z = smooth_damp(cam.z, target_pos.z, &mut velocity.z, smooth_time, time.delta_secs());
   }
}

fn smooth_damp(
   current: f32,
   target: f32,
   current_velocity: &mut f32,
   smooth_time: f32,
   delta_time: f32,
) -> f32 {
   let omega = 2.0 / smooth_time;
   let x = omega * delta_time;
   let exp = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);
   let change = current - target;
   let temp = (*current_velocity + omega * change) * delta_time;
   *current_velocity = (*current_velocity - omega * temp) * exp;
   target + (change + temp) * exp
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

// pub fn handle_player_collisions(q: Query<(Entity, &CollidingEntities), With<Player>>) {
//    for (e, colliding_e) in &q {
//       info!("{} colliding with: {:?}", e, colliding_e);
//    }
// can collide with:
// enemy,enemy_projectile -> take dmg
// loot -> loot logic
// }

pub fn handle_player_collisions(
   mut commands: Commands,
   mut query: Query<(Entity, &CollidingEntities), With<Player>>,
   enemy_query: Query<&Enemy>,
   projectile_query: Query<&EnemyProjectile>,
   health_query: Query<&Health>,
   powerup_query: Query<&PowerUp>,
   xporb_query: Query<&XpOrb>,
) {
   for (player_entity, colliding_entities) in query.iter_mut() {
      for &colliding_entity in &colliding_entities.0 {
         if let Ok(_) = enemy_query.get(colliding_entity) {
            // Handle collision with enemy
            info!("Player {:?} colliding with enemy {:?}", player_entity, colliding_entity);
            // Apply damage to player or other logic
         } else if let Ok(_) = projectile_query.get(colliding_entity) {
            // Handle collision with enemy projectile
            info!(
               "Player {:?} colliding with enemy projectile {:?}",
               player_entity, colliding_entity
            );
            // Apply damage to player or other logic
         } else if let Ok(_) = health_query.get(colliding_entity) {
            // Handle collision with health loot
            info!("Player {:?} colliding with health loot {:?}", player_entity, colliding_entity);
            // Apply health to player or other logic
            commands.entity(colliding_entity).despawn(); // Example: Despawn the loot after collection
         } else if let Ok(_) = powerup_query.get(colliding_entity) {
            // Handle collision with power-up loot
            info!("Player {:?} colliding with power-up loot {:?}", player_entity, colliding_entity);
            // Apply power-up to player or other logic
            commands.entity(colliding_entity).despawn(); // Example: Despawn the loot after collection
         } else if let Ok(_) = xporb_query.get(colliding_entity) {
            // Handle collision with XP orb loot
            info!("Player {:?} colliding with XP orb loot {:?}", player_entity, colliding_entity);
            // Apply XP to player or other logic
            commands.entity(colliding_entity).despawn(); // Example: Despawn the loot after collection
         }
      }
   }
}

fn check_level_up() {}
