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

#[derive(Component, Clone)]
pub struct Enemy {
   pub life: u32,
   pub speed: f32,
   pub damage: u32,
   pub knockback_resistance: f32,
   pub xp: u32,
}

#[derive(Component, Clone)]
pub struct Ranged {
   pub damage: u32,
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
   pub damage: u32,
   pub speed: f32,
   pub timeout: Timer,
   pub debris_sprite_name: String,
}

#[derive(Component, Clone)]
pub struct Debris {
   pub timeout: Timer,
}
