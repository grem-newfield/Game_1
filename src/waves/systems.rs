use std::f32::consts::PI;

use crate::{
   get_sprite, Boss, Debris, Enemy, EnemyKind, EnemyProjectile, GameState, Melee, MyColLayers,
   Player, Ranged, SpritesCollection, WaveSpawnConfig, WaveState, WizardBoss, PROJECTILE_TIMEOUT,
   RES_WIDTH,
};
use avian2d::prelude::*;
use bevy::{
   log::tracing_subscriber::{fmt::format, Layer},
   prelude::*,
};
use rand::prelude::*;

pub fn setup_waves() {}

// pub fn waves_s(
//    mut commands: Commands,
//    time: Res<Time>,
//    mut wave_state: ResMut<WaveState>,
//    mut wave_spawn_config: ResMut<WaveSpawnConfig>,
// ) {
//    if wave_spawn_config.spawn_timer.just_finished() {}
// }

pub fn wave_system(
   time: Res<Time>,
   mut wsc: ResMut<WaveSpawnConfig>,
   mut state: ResMut<WaveState>,
   mut cmd: Commands,
   sprites_collection: Res<SpritesCollection>,
   player_transform: Single<&Transform, With<Player>>,
) {
   // tick
   wsc.wave_difficulty_timer.tick(time.delta());
   wsc.spawn_timer.tick(time.delta());
   wsc.boss_spawn_timer.tick(time.delta());
   // DIFFICULTY
   if wsc.wave_difficulty_timer.finished() {
      state.difficulty += 1;
      wsc.wave_difficulty_timer.reset();
      info!("Wave difficulty increased to {}", state.difficulty);
   }
   // BOSS
   if wsc.boss_spawn_timer.just_finished() {
      let (x, y) = get_random_location_around_player(&player_transform);
      spawn_wizard_boss(&mut cmd, &sprites_collection, x, y);
      wsc.boss_spawn_timer.reset();
      info!("Boss spawned");
   }
   // MOB
   if wsc.spawn_timer.just_finished() && state.mob_count < 1000 {
      let (x, y) = get_random_location_around_player(&player_transform);
      match state.difficulty {
         0 => spawn_slime(&mut cmd, &sprites_collection, x, y),
         1 => {
            // Mix of Slimes and Kobold Archers
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.5) {
               spawn_slime(&mut cmd, &sprites_collection, x, y);
            } else {
               spawn_kobold_archer(&mut cmd, &sprites_collection, x, y);
            }
         }
         2 => {
            // Mix of Slimes, Kobold Archers, and another enemy type
            let mut rng = rand::thread_rng();
            let mob_choice = rng.gen_range(0..3);
            match mob_choice {
               0 => spawn_slime(&mut cmd, &sprites_collection, x, y),
               1 => spawn_kobold_archer(&mut cmd, &sprites_collection, x, y),
               2 => spawn_orc_axeman(&mut cmd, &sprites_collection, x, y),
               _ => {}
            }
         }
         _ => {}
      }
      state.mob_count += 1;
      wsc.spawn_timer.reset();
   }
}

// pub fn despawn_low_tier_mobs_outside_view() {}

// pub fn despawn_mobs_far_away() {}

pub fn despawn_dead_enemies_emit_enemy_killed() {}

pub fn get_random_location_around_player(t: &Transform) -> (f32, f32) {
   // TODO: set proper radius
   let radius = (RES_WIDTH) as f32 * 0.7;
   let mut rng = rand::thread_rng();
   let theta = rng.gen::<f32>() % 2.0 * PI;
   let x = radius * theta.cos();
   let y = radius * theta.sin();
   (x, y)
}

// pub fn spawn_mob(
//    mut ws: ResMut<WaveState>,
//    wsp: Res<WaveSpawnConfig>,
//    mut cmd: Commands,
//    sprites_collection: Res<SpritesCollection>,
//    p_t: Single<&Transform, With<Player>>,
// ) {
//    if wsp.spawn_timer.just_finished() && ws.mob_count < 1000 {
//       let (x, y) = get_random_location_around_player(&p_t);
//       match ws.difficulty {
//          0 => spawn_slime(&mut cmd, &sprites_collection, x, y),
//          _ => {}
//       }
//    }
// }
//

pub fn spawn_slime(
   cmd: &mut Commands,
   sprites_collection: &Res<SpritesCollection>,
   x: f32,
   y: f32,
) {
   let sprite = get_sprite(cmd, sprites_collection, "slime");
   cmd.spawn((
      EnemyKind::Slime,
      Melee,
      Name::new("Slime Enemy"),
      sprite,
      Enemy { life: 1, speed: 50.0, damage: 1, knockback_resistance: 0.0, xp: 10 },
      Transform::from_xyz(x, y, 0.0),
      RigidBody::Dynamic,
      Friction::new(0.0),
      Collider::circle(4.0),
      LockedAxes::ROTATION_LOCKED,
   ));
}

pub fn spawn_kobold_archer(
   cmd: &mut Commands,
   sprites_collection: &Res<SpritesCollection>,
   x: f32,
   y: f32,
) {
   let sprite = get_sprite(cmd, sprites_collection, "kobold_archer");
   cmd.spawn((
      EnemyKind::KoboldArcher,
      Name::new("Kobold Archer Enemy"),
      sprite,
      Enemy { life: 1, speed: 30.0, damage: 1, knockback_resistance: 0.0, xp: 10 },
      Ranged {
         damage: 1,
         range: 150.0,
         cooldown_timer: Timer::from_seconds(2.0, TimerMode::Once),
         projectile_speed: 50.0,
         projectile_sprite_name: String::from("arrow"),
         debris_sprite_name: String::from("broken_arrow"),
      },
      Transform::from_xyz(x, y, 0.0),
      RigidBody::Dynamic,
      Friction::new(0.0),
      Collider::circle(6.0),
      LockedAxes::ROTATION_LOCKED,
   ));
}

pub fn melee_ai(
   mut q: Query<
      (&Transform, &mut LinearVelocity, &Enemy),
      (With<Enemy>, Without<Ranged>, Without<Player>),
   >,
   p: Single<&Transform, With<Player>>,
   time: Res<Time>,
) {
   // let my_span = info_span!("melee_ai_move", name = "melee_ai_move").entered();
   for (mut et, mut v, enemy) in q.iter_mut() {
      let mut direction = p.translation - et.translation;
      direction = direction.normalize();
      v.x = direction.x * enemy.speed;
      v.y = direction.y * enemy.speed;
   }
}

pub fn rangers_ai(
   // q: Query<Entity, (With<Enemy>, With<Ranged>)>,
   mut cmd: Commands,
   sprites_collection: Res<SpritesCollection>,
   mut q: Query<
      (&Transform, &EnemyKind, &mut LinearVelocity, &Enemy, &mut Ranged),
      (With<Enemy>, With<Ranged>, Without<Player>, Without<Melee>),
   >,
   p: Single<&Transform, With<Player>>,
   time: Res<Time>,
) {
   for (mut mob_t, enemy_kind, mut enemy_vel, enemy_stats, mut ranged_stats) in q.iter_mut() {
      ranged_stats.cooldown_timer.tick(time.delta());
      let mut direction = p.translation - mob_t.translation;
      direction = direction.normalize();
      let dist = mob_t.translation.distance(p.translation);
      if (dist > ranged_stats.range - 3.0) && (dist < ranged_stats.range + 3.0) {
         enemy_vel.x = 0.0;
         enemy_vel.y = 0.0;
         if ranged_stats.cooldown_timer.finished() {
            ranged_stats.cooldown_timer.reset();
            spawn_projectile(
               &mut cmd,
               &sprites_collection,
               mob_t.translation.x,
               mob_t.translation.y,
               p.translation.x,
               p.translation.y,
               ranged_stats.damage,
               &ranged_stats.projectile_sprite_name,
               &ranged_stats.debris_sprite_name,
               ranged_stats.projectile_speed,
            );
         }
      } else {
         if dist < ranged_stats.range {
            direction *= -1.0;
         }
         enemy_vel.x = direction.x * enemy_stats.speed;
         enemy_vel.y = direction.y * enemy_stats.speed;
      }
   }
}

pub fn spawn_projectile(
   cmd: &mut Commands,
   sprites_collection: &Res<SpritesCollection>,
   x: f32,
   y: f32,
   target_x: f32,
   target_y: f32,
   damage: u32,
   sprite_name: &str,
   debris_name: &str,
   projectile_speed: f32,
) {
   let sprite = get_sprite(cmd, sprites_collection, sprite_name);
   let dir = (Vec2::new(target_x, target_y) - Vec2::new(x, y)).normalize();
   let angle = dir.y.atan2(dir.x);
   let rot = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_4);
   cmd.spawn((
      EnemyProjectile {
         damage,
         speed: projectile_speed,
         debris_sprite_name: debris_name.to_string(),
         timeout: Timer::from_seconds(PROJECTILE_TIMEOUT, TimerMode::Once),
      },
      Name::new(format!("Projectile {}", sprite_name)),
      sprite,
      Transform::from_xyz(x, y, 0.0).with_rotation(rot),
      RigidBody::Kinematic,
      Friction::new(0.0),
      Collider::circle(3.0),
      LockedAxes::ROTATION_LOCKED,
      Sensor,
      CollisionLayers::new(
         MyColLayers::EnemyProjectile,
         [MyColLayers::Player, MyColLayers::Doodad],
      ),
   ));
}

pub fn move_enemy_projectiles(
   mut q: Query<(&mut Transform, &EnemyProjectile)>,
   time: Res<Time>,
) {
   for (mut transform, projectile_stats) in q.iter_mut() {
      let direction = Quat::from_rotation_z(
         transform.rotation.to_euler(EulerRot::XYZ).2 + std::f32::consts::FRAC_PI_4,
      ) * Vec3::X;
      let t = direction.normalize() * projectile_stats.speed * time.delta_secs();
      transform.translation += t;
   }
}

pub fn timeout_enemy_projectiles(
   mut cmd: Commands,
   sprites_collection: Res<SpritesCollection>,
   time: Res<Time>,
   mut query: Query<(Entity, &mut EnemyProjectile, &Transform)>,
) {
   for (e, mut p, t) in query.iter_mut() {
      p.timeout.tick(time.delta());
      if p.timeout.finished() {
         cmd.entity(e).despawn();

         let mut rng = rand::thread_rng();
         let random_rotation = rng.gen_range(0.0..std::f32::consts::PI * 2.0);

         let sprite = get_sprite(&mut cmd, &sprites_collection, &p.debris_sprite_name);

         cmd.spawn((
            Name::new(format!("Debris {}", p.debris_sprite_name)),
            sprite,
            Transform {
               translation: t.translation,
               rotation: Quat::from_rotation_z(random_rotation),
               ..Default::default()
            },
            Debris { timeout: Timer::from_seconds(2.0, TimerMode::Once) },
         ));
      }
   }
}

pub fn fade_debris_system(
   mut commands: Commands,
   time: Res<Time>,
   mut query: Query<(Entity, &mut Debris, &mut Sprite)>,
) {
   for (e, mut debris, mut sprite) in query.iter_mut() {
      debris.timeout.tick(time.delta());
      if debris.timeout.finished() {
         commands.entity(e).despawn();
      } else {
         let fade_factor =
            1.0 - (debris.timeout.elapsed_secs() / debris.timeout.duration().as_secs_f32());
         sprite.color.set_alpha(fade_factor);
      }
   }
}

pub fn spawn_mob(
   mut ws: ResMut<WaveState>,
   wsp: Res<WaveSpawnConfig>,
   mut cmd: Commands,
   sprites_collection: Res<SpritesCollection>,
   player_transform: Single<&Transform, With<Player>>,
) {
}

pub fn spawn_orc_axeman(
   cmd: &mut Commands,
   sprites_collection: &Res<SpritesCollection>,
   x: f32,
   y: f32,
) {
   let sprite = get_sprite(cmd, sprites_collection, "orc_axeman");
   cmd.spawn((
      EnemyKind::Slime,
      Melee,
      Name::new("Orc Axeman"),
      sprite,
      Enemy { life: 4, speed: 20.0, damage: 5, knockback_resistance: 0.0, xp: 20 },
      Transform::from_xyz(x, y, 0.0),
      RigidBody::Dynamic,
      Friction::new(0.0),
      Collider::circle(5.0),
      ColliderDensity(100.0),
      LockedAxes::ROTATION_LOCKED,
   ));
}

pub fn spawn_wizard_boss(
   cmd: &mut Commands,
   sprites_collection: &Res<SpritesCollection>,
   x: f32,
   y: f32,
) {
   let sprite = get_sprite(cmd, sprites_collection, "wizard_boss");
   cmd.spawn((
      sprite,
      EnemyKind::WizardBoss,
      Boss,
      Name::new("Wizard Boss"),
      Transform::from_xyz(x, y, 0.0),
      RigidBody::Dynamic,
      Friction::new(0.0),
      Collider::circle(8.0),
      ColliderDensity(100.0),
      LockedAxes::ROTATION_LOCKED,
      Enemy { life: 100, speed: 20.0, damage: 10, knockback_resistance: 0.0, xp: 100 },
      Ranged {
         damage: 1,
         range: 130.0,
         cooldown_timer: Timer::from_seconds(3.0, TimerMode::Once),
         projectile_speed: 40.0,
         projectile_sprite_name: String::from("wizard_boss_attack"),
         debris_sprite_name: String::from("wizard_boss_attack_debris"),
      },
      WizardBoss {
         ranged_attack_timer: Timer::from_seconds(5.0, TimerMode::Once),

         projectile_sprite_name: String::from("wizard_boss_attack"),
         debris_sprite_name: String::from("wizard_boss_attack_debris"),
         projectile_speed: 100.0,
      },
   ));
}

pub fn ranged_boss_attack(
   mut cmd: Commands,
   sprites_collection: Res<SpritesCollection>,
   mut query: Query<(&Transform, &mut WizardBoss, &EnemyKind)>,
   time: Res<Time>,
   player_transform: Single<&Transform, With<Player>>,
) {
   for (transform, mut boss, kind) in query.iter_mut() {
      match kind {
         EnemyKind::WizardBoss => {
            boss.ranged_attack_timer.tick(time.delta());
            if boss.ranged_attack_timer.finished() {
               // Reset the attack timer
               boss.ranged_attack_timer.reset();

               // Parameters for the arc attack
               let num_projectiles = 5;
               let spread_angle = std::f32::consts::PI / 4.0; // 45 degrees

               // Calculate the starting angle
               // let start_angle = transform.rotation.to_euler(EulerRot::XYZ).2 - spread_angle / 2.0;
               let direction_to_player =
                  (player_transform.translation - transform.translation).normalize();
               let base_angle = direction_to_player.y.atan2(direction_to_player.x);

               let start_angle = base_angle - spread_angle / 2.0;

               // Spawn projectiles in an arc
               for i in 0..num_projectiles {
                  let angle = start_angle + i as f32 * spread_angle / (num_projectiles - 1) as f32;
                  let dir = Vec2::new(angle.cos(), angle.sin());

                  // Spawn the projectile
                  spawn_projectile(
                     &mut cmd,
                     &sprites_collection,
                     transform.translation.x,
                     transform.translation.y,
                     transform.translation.x + dir.x,
                     transform.translation.y + dir.y,
                     5,
                     &boss.projectile_sprite_name,
                     &boss.debris_sprite_name,
                     boss.projectile_speed,
                  );
               }
            }
         }
         _ => {}
      }
   }
}
