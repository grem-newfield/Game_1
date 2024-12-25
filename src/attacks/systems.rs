use crate::{
   get_sprite, AttackTimer, Boss, DaggerAttack, DaggerAttackConf, DaggerAttackProjectile, Debris,
   Enemy, Player, SpritesCollection,
};
use avian2d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;

pub fn tick_attack_timers(
   mut q_timers: Query<(&mut AttackTimer)>,
   time: Res<Time>,
) {
   for mut timer in q_timers.iter_mut() {
      timer.timer.tick(time.delta());
   }
}

// (AttackTimer, SomeAttackName1)
// (AttackTimer, SomeAttackName2)
// (AttackTimer, SomeAttackName3)

// pub fn attacks_system() {}

#[derive(Resource, Default, Debug)]
pub struct ClosestEnemy {
   pub x: f32,
   pub y: f32,
}

pub fn set_closest_enemy(
   mut q_enemy_transforms: Query<&Transform, (Or<(With<Enemy>, With<Boss>)>, Without<Player>)>,
   q_player_transform: Single<&Transform, (With<Player>, Without<Enemy>, Without<Boss>)>,
   mut closest_enemy: ResMut<ClosestEnemy>,
) {
   let mut closest_dist = f32::MAX;
   for enemy_trans in &q_enemy_transforms {
      let dist = enemy_trans.translation.distance(q_player_transform.translation);
      if dist < closest_dist {
         closest_dist = dist;
         closest_enemy.x = enemy_trans.translation.x;
         closest_enemy.y = enemy_trans.translation.y;
      }
   }
}

pub fn weapons_system(
   mut cmd: Commands,
   time: Res<Time>,
   sprites_collection: Res<SpritesCollection>,
   // ass: Res<AssetServer>,
   // sprites: Res<Assets<Image>>,
   // projectile_art: Res<ProjectileArt>,
   // mut q_enemies: Query<(Entity), With<Enemy>>,
   mut q_timers: Query<&mut AttackTimer, With<DaggerAttack>>,
   mut q_enemies_transforms: Query<&Transform, (With<Enemy>, With<Boss>, Without<Player>)>,
   q_player_transform: Single<&Transform, (With<Player>, Without<Enemy>)>,
) {
   for mut attack_timer in q_timers.iter_mut() {
      attack_timer.timer.tick(time.delta());
      if attack_timer.timer.finished() {
         attack_timer.timer.reset();
         // info!("moggin the opps");
         // let result = Vec3::splat(10_000_000.);
         let mut closest_dist = f32::MAX;
         let mut closest_transform: Option<&Transform> = None;
         for t in &q_enemies_transforms {
            let dist = t.translation.distance(q_player_transform.translation);
            if dist < closest_dist {
               closest_dist = dist;
               closest_transform = Some(t);
            }
         }
         match closest_transform {
            Some(t) => {
               cmd.spawn(
                  ((
                     // Name::new("Boolet"),
                     // Projectile { speed: 50.0, lifetime: 3.0 },
                     // GlobalTransform::from_translation(q_player_transform.translation),
                     // .look_to(t.translation, UP),
                     // Sprite::from(projectile_art.basic_projectile.clone()),
                     RigidBody::Kinematic,
                     Collider::circle(5.0),
                  )),
               );
               // info!("Spawned projectile");
            }
            None => return,
         };
         // let ray_dir = Dir3::new().unwrap();
         // info!("Timer Lmao");
         // summon projectile
         // let mut ent_cmd = c.spawn(
         //    ((
         //       Projectile { speed: 50.0, lifetime: 1.0 },
         //       Transform::from_xyz(0.0, 0.0, 0.0).look_to(closest_enemy, UP),
         //       Sprite::from(projectile_art.basic_projectile.clone()),
         //       RigidBody::Kinematic,
         //       Collider::circle(2.0),
         //    )),
         // );
         // if cfg!(debug_assertions) {
         //    ent_cmd.insert(DebugRender::default().with_collider_color(Color::srgb(0.0, 1.0, 0.0)));
         // }
      }
   }
}

// pub fn move_player_projectiles(
//    mut c: Commands,
//    time: Res<Time>,
//    mut q: Query<(&mut Transform, &PlayerSimpleProjectile)>,
//    // mut q: Query<(&mut Transform)>,
// ) {
//    for (e, mut transform, projectile) in q.iter_mut() {
//       // move into direction of rotation
//       let rotation = transform.rotation;
//
//       let t = transform.local_y() * projectile.speed * time.delta_secs();
//       transform.translation += t;
//    }
// }

// pub fn setup_weapons(mut c: Commands) {}

// pub fn timeout_player_projectiles(
//    mut cmd: Commands,
//    sprites_collection: Res<SpritesCollection>,
//    time: Res<Time>,
//    mut query: Query<(Entity, &mut Projectile, &Transform)>,
// ) {
//    for (e, mut p, t) in query.iter_mut() {
//       p.timeout.tick(time.delta());
//       if p.timeout.finished() {
//          cmd.entity(e).despawn();
//
//          let mut rng = rand::thread_rng();
//          let random_rotation = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
//
//          let (name, sprite) = get_sprite(&mut cmd, &sprites_collection, "broken_arrow");
//
//          cmd.spawn((
//             name,
//             sprite,
//             Transform {
//                translation: t.translation,
//                rotation: Quat::from_rotation_z(random_rotation),
//                ..Default::default()
//             },
//             Debris { timeout: Timer::from_seconds(2.0, TimerMode::Once) },
//          ));
//       }
//    }
// }
// pub fn timeout_player_projectiles(
//    mut cmd: Commands,
//    sprites_collection: Res<SpritesCollection>,
//    time: Res<Time>,
//    mut query: Query<(Entity, &mut PlayerSimpleProjectile, &Transform)>,
// ) {
//    for (e, mut p, t) in query.iter_mut() {
//       p.timeout.tick(time.delta());
//       if p.timeout.finished() {
//          cmd.entity(e).despawn();
//
//          let mut rng = rand::thread_rng();
//          let random_rotation = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
//
//          let sprite = get_sprite(&mut cmd, &sprites_collection, &p.debris_sprite_name);
//
//          cmd.spawn((
//             Name::new(format!("Debris {}", p.debris_sprite_name)),
//             sprite,
//             Transform {
//                translation: t.translation,
//                rotation: Quat::from_rotation_z(random_rotation),
//                ..Default::default()
//             },
//             Debris { timeout: Timer::from_seconds(2.0, TimerMode::Once) },
//          ));
//       }
//    }
// }
