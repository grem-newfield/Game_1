use crate::{
   get_sprite, spawn_projectile, AttackTimer, Boss, ClosestEnemy, Debris, Enemy, Player,
   SpritesCollection,
};
use avian2d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;

// DATA
#[derive(Component, Clone)]
pub struct DaggerAttackProjectile {
   pub damage: u32,
   pub speed: f32,
   pub timeout: Timer,
   pub sprite_name: String,
   pub debris_sprite_name: String,
}

impl Default for DaggerAttackProjectile {
   fn default() -> Self {
      DaggerAttackProjectile {
         damage: 1,
         speed: 100.,
         timeout: Timer::from_seconds(3.0, TimerMode::Once),
         sprite_name: String::from("dagger"),
         debris_sprite_name: String::from("dagger"),
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
   pub attack_timer: AttackTimer,
   pub dagger_attack: DaggerAttack,
   pub dagger_attack_projectile: DaggerAttackProjectile,
}

impl Default for DaggerAttackBundle {
   fn default() -> Self {
      DaggerAttackBundle {
         attack_timer: AttackTimer { timer: Timer::from_seconds(0.2, TimerMode::Once) },
         dagger_attack: DaggerAttack,
         dagger_attack_projectile: DaggerAttackProjectile::default(),
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
   dagger_attack_projectile: Single<&DaggerAttackProjectile>,
   closest_target: Res<ClosestEnemy>,
   sprites_collection: Res<SpritesCollection>,
   mut cmd: Commands,
   p: Single<&Transform, With<Player>>,
) {
   if timer_q.timer.finished() {
      // if ranged_stats.cooldown_timer.finished() {
      timer_q.timer.reset();
      spawn_projectile(
         &mut cmd,
         &sprites_collection,
         p.translation.x,
         p.translation.y,
         closest_target.x,
         closest_target.y,
         dagger_attack_projectile.damage,
         &dagger_attack_projectile.sprite_name,
         &dagger_attack_projectile.debris_sprite_name,
         dagger_attack_projectile.speed,
      );
   }
}

pub fn timeout_dagger_attack_projectiles(
   mut cmd: Commands,
   sprites_collection: Res<SpritesCollection>,
   time: Res<Time>,
   mut query: Query<(Entity, &mut DaggerAttackProjectile, &Transform)>,
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
