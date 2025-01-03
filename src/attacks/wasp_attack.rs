use crate::{
   get_sprite, spawn_projectile, AttackTimer, Boss, ClosestEnemy, Debris, Enemy, Player,
   PlayerProjectile, SpritesCollection,
};
use avian2d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;

// DATA

#[derive(Component)]
pub struct WaspAttack;

#[derive(Component, Clone, Debug)]
pub struct WaspAttackProjectile {
   pub damage: i32,
   pub speed: f32,
   pub timeout: Timer,
   pub sprite_name: String,
   pub debris_sprite_name: String,
}

impl Default for WaspAttackProjectile {
   fn default() -> Self {
      WaspAttackProjectile {
         damage: 100,
         speed: 100.,
         timeout: Timer::from_seconds(3.0, TimerMode::Once),
         sprite_name: String::from("wasp"),
         debris_sprite_name: String::from("wasp"),
      }
   }
}

#[derive(Bundle)]
pub struct WaspAttackBundle {
   pub tag: PlayerProjectile,
   pub attack_timer: AttackTimer,
   pub wasp_attack: WaspAttack,
   pub wasp_attack_projectile: WaspAttackProjectile,
}

impl Default for WaspAttackBundle {
   fn default() -> Self {
      WaspAttackBundle {
         tag: PlayerProjectile,
         attack_timer: AttackTimer { timer: Timer::from_seconds(0.6, TimerMode::Once) },
         wasp_attack: WaspAttack,
         wasp_attack_projectile: WaspAttackProjectile::default(),
      }
   }
}
