use bevy::prelude::*;

// prob dont even need this
#[derive(Resource)]
pub enum WaveState {
   Spawning,
   Idle,
}

#[derive(Resource)]
pub enum EnemyState {}
