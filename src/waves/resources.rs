use bevy::prelude::*;

pub const PROJECTILE_TIMEOUT: f32 = 3.0;

// prob dont even need this
#[derive(Resource, Default)]
pub struct WaveState {
   pub difficulty: usize,
   pub mob_count: u32,
}

#[derive(Resource)]
pub struct WaveSpawnConfig {
   pub wave_difficulty_timer: Timer,
   pub spawn_timer: Timer,
   pub boss_spawn_timer: Timer,
}

impl Default for WaveSpawnConfig {
   fn default() -> Self {
      Self {
         // // wave difficulty every 5 min
         // wave_difficulty_timer: Timer::from_seconds(5.0 * 60.0, TimerMode::Once),
         // // mob every 1 sec
         // spawn_timer: Timer::from_seconds(1.0, TimerMode::Once),
         // // boss every 10 min
         // boss_spawn_timer: Timer::from_seconds(10.0 * 60.0, TimerMode::Once),

         // test values
         wave_difficulty_timer: Timer::from_seconds(1.0, TimerMode::Once),
         spawn_timer: Timer::from_seconds(0.1, TimerMode::Once),
         boss_spawn_timer: Timer::from_seconds(10., TimerMode::Once),
      }
   }
}
