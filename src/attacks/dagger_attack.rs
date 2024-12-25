use crate::{
   get_sprite, spawn_projectile, AttackTimer, Boss, ClosestEnemy, Debris, Enemy, MyCollisionLayers,
   Player, PlayerProjectile, SpritesCollection, PROJECTILE_TIMEOUT,
};
use avian2d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;

// DATA
#[derive(Component, Clone, Debug)]
pub struct DaggerAttackProjectile {
   pub damage: i32,
   pub speed: f32,
   pub timeout: Timer,
   // pub sprite_name: String,
   // pub debris_sprite_name: String,
}

#[derive(Resource)]
pub struct DaggerAttackConf {
   pub sprite_name: String,
   pub debris_sprite_name: String,
}

impl Default for DaggerAttackConf {
   fn default() -> Self {
      DaggerAttackConf {
         sprite_name: String::from("dagger"),
         debris_sprite_name: String::from("dagger"),
      }
   }
}

impl Default for DaggerAttackProjectile {
   fn default() -> Self {
      DaggerAttackProjectile {
         damage: 100,
         speed: 100.,
         timeout: Timer::from_seconds(PROJECTILE_TIMEOUT, TimerMode::Once),
         // sprite_name: String::from("dagger"),
         // debris_sprite_name: String::from("dagger"),
      }
   }
}

#[derive(Component)]
pub struct DaggerAttack;
// {
// pub graphic: String,
// }

// impl Default for DaggerAttack {
//    fn default() -> Self {
//       DaggerAttack { graphic: String::from("dagger") }
//    }
// }

#[derive(Bundle)]
pub struct DaggerAttackBundle {
   // pub tag: PlayerProjectile,
   pub attack_timer: AttackTimer,
   pub dagger_attack: DaggerAttack,
   // pub dagger_attack_projectile: DaggerAttackProjectile,
}

impl Default for DaggerAttackBundle {
   fn default() -> Self {
      DaggerAttackBundle {
         // tag: PlayerProjectile,
         attack_timer: AttackTimer { timer: Timer::from_seconds(0.6, TimerMode::Once) },
         dagger_attack: DaggerAttack,
         // dagger_attack_projectile: DaggerAttackProjectile::default(),
      }
   }
}

// SYSTEMS
// pub fn setup_dagger_attack(
//    mut cmd: Commands,
//    sprites_collection: Res<SpritesCollection>,
// ) {
// }

pub fn dagger_attack_system(
   mut timer_q: Single<&mut AttackTimer, With<DaggerAttack>>,
   dagger_attack_conf: Res<DaggerAttackConf>,
   closest_target: Res<ClosestEnemy>,
   sprites_collection: Res<SpritesCollection>,
   mut cmd: Commands,
   p: Single<&Transform, With<Player>>,
) {
   if timer_q.timer.finished() {
      // if ranged_stats.cooldown_timer.finished() {
      timer_q.timer.reset();
      // spawn_projectile(
      //    &mut cmd,
      //    &sprites_collection,
      //    p.translation.x,
      //    p.translation.y,
      //    closest_target.x,
      //    closest_target.y,
      //    dagger_attack_projectile.damage,
      //    &dagger_attack_projectile.sprite_name,
      //    &dagger_attack_projectile.debris_sprite_name,
      //    dagger_attack_projectile.speed,
      // );
      let sprite = get_sprite(&mut cmd, &sprites_collection, &dagger_attack_conf.sprite_name);
      let dir = (Vec2::new(closest_target.x, closest_target.y)
         - Vec2::new(p.translation.x, p.translation.y))
      .normalize();
      let angle = dir.y.atan2(dir.x);
      let rot = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_4);
      cmd.spawn((
         PlayerProjectile,
         DaggerAttackProjectile::default(),
         // {
         //    damage: dagger_attack_projectile.damage,
         //    speed: dagger_attack_projectile.speed,
         //    debris_sprite_name: dagger_attack_projectile.debris_sprite_name.to_string(),
         //    timeout: Timer::from_seconds(PROJECTILE_TIMEOUT, TimerMode::Once),
         // },
         Name::new(format!("Projectile {}", dagger_attack_conf.sprite_name)),
         sprite,
         Transform::from_xyz(p.translation.x, p.translation.y, 0.0).with_rotation(rot),
         RigidBody::Kinematic,
         Friction::new(0.0),
         Collider::circle(3.0),
         LockedAxes::ROTATION_LOCKED,
         Sensor,
         CollisionLayers::new(
            MyCollisionLayers::PlayerProjectile,
            [MyCollisionLayers::Enemy, MyCollisionLayers::Doodad],
         ),
      ));
   }
}

pub fn move_dagger_projectiles(
   mut q: Query<(&mut Transform, &DaggerAttackProjectile)>,
   time: Res<Time>,
   dagger_conf: Res<DaggerAttackConf>,
) {
   for (mut transform, projectile_stats) in q.iter_mut() {
      let direction = Quat::from_rotation_z(
         transform.rotation.to_euler(EulerRot::XYZ).2 + std::f32::consts::FRAC_PI_4,
      ) * Vec3::X;
      let t = direction.normalize() * projectile_stats.speed * time.delta_secs();
      transform.translation += t;
   }
}

pub fn timeout_dagger_attack_projectiles(
   mut cmd: Commands,
   sprites_collection: Res<SpritesCollection>,
   time: Res<Time>,
   dagger_attack_conf: Res<DaggerAttackConf>,
   mut query: Query<(Entity, &mut DaggerAttackProjectile, &Transform)>,
) {
   for (e, mut p, t) in query.iter_mut() {
      p.timeout.tick(time.delta());
      if p.timeout.finished() {
         cmd.entity(e).despawn();

         let mut rng = rand::thread_rng();
         let random_rotation = rng.gen_range(0.0..std::f32::consts::PI * 2.0);

         let sprite =
            get_sprite(&mut cmd, &sprites_collection, &dagger_attack_conf.debris_sprite_name);

         cmd.spawn((
            Name::new(format!("Debris {}", dagger_attack_conf.debris_sprite_name)),
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

pub fn add_dagger_attack(mut cmd: Commands) {
   cmd.spawn(DaggerAttackBundle::default());
}

pub fn remove_dagger_attack(
   mut cmd: Commands,
   mut q: Query<Entity, With<DaggerAttack>>,
) {
   for e in q.iter() {
      cmd.entity(e).despawn();
   }
}
