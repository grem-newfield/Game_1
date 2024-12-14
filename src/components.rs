use std::time::Duration;

use bevy::prelude::*;
// pub mod tags {
// use bevy::prelude::*;
// #[derive(Component)]
// pub struct MainCamera;

// #[derive(Component)]
// pub struct Actor;
// }

#[derive(Component, Debug)]
pub struct Player {
   pub speed: f32,
   pub health: u32,
}

// #[derive(Component)]
// pub struct TestAttackTimer {
//    pub timer: Timer,
// }
//
// #[derive(Component, Debug)]
// pub struct Projectile {
//    pub speed: f32,
//    pub lifetime: f32,
// }
