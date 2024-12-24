use crate::{
   get_sprite, player::components::*, Action, AttackTimer, DaggerAttack, Enemy, EnemyProjectile,
   GamePaused, GameState, GameUnPaused, Health, PlayerMoveEvent, PlayerMovedFarEnough, PowerUp,
   ProjectileArt, SpritesCollection, XpOrb,
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

pub fn toggle_pause_ingame(
   mut paused: EventWriter<GamePaused>,
   mut unpaused: EventWriter<GameUnPaused>,
   action_state: Single<&ActionState<Action>, With<Player>>,
   state: Res<State<GameState>>,
   mut next_state: ResMut<NextState<GameState>>,
) {
   if action_state.just_pressed(&Action::Pause) {
      match state.get() {
         GameState::InGame => {
            paused.send(GamePaused);
            next_state.set(GameState::Paused);
         }
         GameState::Paused => {
            unpaused.send(GameUnPaused);
            next_state.set(GameState::InGame);
         }
         _ => {}
      }
   }
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
      (Action::Pause, KeyCode::Escape),
      (Action::Pause, KeyCode::Space),
   ]);

   let sprite = get_sprite(&mut cmd, &sprites, "player");
   cmd.spawn((
      sprite,
      Name::new("Player"),
      Player {
         life: 100,
         speed: 80.0,
         last_position: Vec3::ZERO,
         damage_cooldown: Timer::from_seconds(1.0, TimerMode::Once),
         level: 0,
         experience: 0.0,
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
   // TODO: reenable smooth / remake smooth
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
   mut collision_event_reader: EventReader<CollisionStarted>,
) {
   for CollisionStarted(e1, e2) in collision_event_reader.read() {
      info!("entities {} and {} collided 🦍", e1, e2);
   }
   return;

   // TODO: Finish this monstrocity

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

pub fn check_level_up(mut player: Single<&mut Player>) {
   if player.experience >= 100.0 {
      player.experience -= 100.0;
      player.level += 1;
      // TODO: summon screen here with weapon update
   }
}

pub fn emit_player_moved_far_enough(
   // mut cmd: Commands,
   // sprites_collection: Res<SpritesCollection>,
   mut player: Single<&mut Player>,
   player_t: Single<&Transform, With<Player>>,
   // window: Single<&Window>,
   mut ew: EventWriter<PlayerMovedFarEnough>,
) {
   let cur_pos = player_t.translation;
   // info!("curpos {}", cur_pos);
   let distance_moved = player.last_position.distance(cur_pos);
   if distance_moved > 100.0 {
      ew.send(PlayerMovedFarEnough);
      // info!("moved 10.0, doodadding");
      // player.last_position = cur_pos;
      // add_grass_cluster_out_of_sprites(&mut cmd, &sprites_collection, cur_pos, window);
      // add_points_of_interest(&mut cmd, &sprites_collection, cur_pos);
   }
}
