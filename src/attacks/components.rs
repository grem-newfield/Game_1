use bevy::prelude::*;

#[derive(Component)]
pub struct AttackTimer {
   pub timer: Timer,
}

#[derive(Component)]
pub struct TestAttack {
   pub damage: u32,
   pub graphic: String,
   pub debree: String,
}

#[derive(Component, Debug)]
pub struct Projectile {
   pub speed: f32,
   pub lifetime: f32,
}
