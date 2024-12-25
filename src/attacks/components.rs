use bevy::prelude::*;

#[derive(Component)]
pub struct AttackTimer {
   pub timer: Timer,
}

#[derive(Component, Clone)]
pub struct PlayerProjectile;
// {
//    pub damage: u32,
//    pub speed: f32,
//    pub timeout: Timer,
//    pub debris_sprite_name: String,
// }
