// resources
use bevy::prelude::*;

// #[derive(Resource, Default)]
// pub struct CameraState {
//    pub projection_scale: f32,
// }

#[derive(Resource)]
pub struct ProjectileArt {
   pub basic_projectile: Handle<Image>,
}

#[derive(Resource)]
pub struct Art {
   pub player: Handle<Image>,
}

#[derive(Resource)]
pub enum WaveState {
   Spawning,
   Idle,
}

#[derive(Resource)]
pub enum EnemyState {}
