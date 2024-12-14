use bevy::prelude::*;

#[derive(Component)]
pub struct TestAttackTimer {
   pub timer: Timer,
}

#[derive(Component, Debug)]
pub struct Projectile {
   pub speed: f32,
   pub lifetime: f32,
}
