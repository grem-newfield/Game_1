use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Shooter {
   pub attack_timer: Timer,
}

#[derive(Component, Copy, Clone)]
pub enum EnemyKind {
   Slime,
   KoboldArcher,
   WizardBoss,
}

#[derive(Component, Clone)]
pub struct WizardBoss {
   pub ranged_attack_timer: Timer,
   pub projectile_speed: f32,
   pub projectile_sprite_name: String,
   pub debris_sprite_name: String,
}

#[derive(Component, Clone)]
pub struct Boss;

//  NOTE: maybe redo to have separate components
// #[derive(Component, Clone)]
// pub struct Life(f32);
// #[derive(Component, Clone)]
// pub struct Speed(f32);
// #[derive(Component, Clone)]
// pub struct Damage(u32);

#[derive(Component, Clone)]
pub struct Enemy {
   pub health: i32,
   pub speed: f32,
   pub damage: i32,
   pub knockback_resistance: f32,
   pub xp: i32,
}

#[derive(Component, Clone)]
pub struct Ranged {
   pub damage: i32,
   pub range: f32,
   pub cooldown_timer: Timer,
   pub projectile_speed: f32,
   pub projectile_sprite_name: String,
   pub debris_sprite_name: String,
}

#[derive(Component, Clone)]
pub struct Melee;

#[derive(Component, Clone)]
pub struct EnemyProjectile {
   pub damage: i32,
   pub speed: f32,
   pub timeout: Timer,
   pub debris_sprite_name: String,
}

#[derive(Component, Clone)]
pub struct Debris {
   pub timeout: Timer,
}

#[derive(Component)]
pub struct HitCooldowns {
   pub dagger_cooldown: Timer,
   pub book_cooldown: Timer,
}

impl Default for HitCooldowns {
   fn default() -> Self {
      HitCooldowns {
         dagger_cooldown: Timer::from_seconds(0.3, TimerMode::Once),
         book_cooldown: Timer::from_seconds(0.3, TimerMode::Once),
      }
   }
}
#[derive(Event)]
pub struct EnemyDied {
   pub enemy_kind: EnemyKind,
   pub x: f32,
   pub y: f32,
   pub experience: i32,
}
