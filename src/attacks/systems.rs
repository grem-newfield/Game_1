use crate::{Enemy, Player, Projectile, ProjectileArt, TestAttackTimer};
use avian2d::prelude::*;
use bevy::prelude::*;
pub fn weapons_system(
   mut c: Commands,
   time: Res<Time>,
   ass: Res<AssetServer>,
   sprites: Res<Assets<Image>>,
   projectile_art: Res<ProjectileArt>,
   // mut q_timers: Query<(Entity, &mut AttackTimer)>,
   mut q_timers: Query<(Entity, &mut TestAttackTimer)>,
   mut q_enemies: Query<(Entity), With<Enemy>>,
   mut q_enemies_transforms: Query<&Transform, (With<Enemy>, Without<Player>)>,
   q_player_transform: Single<&Transform, (With<Player>, Without<Enemy>)>,
) {
   for (e, mut attack_timer) in q_timers.iter_mut() {
      attack_timer.timer.tick(time.delta());
      if attack_timer.timer.finished() {
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
               c.spawn(
                  ((
                     Name::new("Boolet"),
                     Projectile { speed: 50.0, lifetime: 1.0 },
                     GlobalTransform::from_translation(q_player_transform.translation),
                     // .look_to(t.translation, UP),
                     Sprite::from(projectile_art.basic_projectile.clone()),
                     RigidBody::Kinematic,
                     Collider::circle(2.0),
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

pub fn shoot_towards_closest_enemy_on_cooldown_end(
   q_player: Query<&Transform, With<Player>>,
   q_enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
}

pub fn move_projectiles(
   mut c: Commands,
   time: Res<Time>,
   mut q: Query<(Entity, &mut Transform, &Projectile)>,
) {
   for (e, mut transform, projectile) in q.iter_mut() {
      // move into direction of rotation
      let rotation = transform.rotation;

      let t = transform.local_y() * projectile.speed * time.delta_secs();
      transform.translation += t;
   }
}

pub fn timeout_despawn_projectiles(
   mut c: Commands,
   time: Res<Time>,
   mut q: Query<(Entity, &mut Projectile)>,
) {
   for (e, mut projectile) in q.iter_mut() {
      projectile.lifetime -= time.delta_secs();
      if projectile.lifetime <= 0.0 {
         c.entity(e).despawn();
      }
   }
}

pub fn setup_weapons(mut c: Commands) {
   c.spawn(TestAttackTimer { timer: Timer::from_seconds(1.0, TimerMode::Repeating) });
}
