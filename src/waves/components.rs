use bevy::prelude::*;

#[derive(Component)]
pub struct WaveSpawnConfig {
   pub wave_difficulty_timer: Timer,
   pub spawn_timer: Timer,
   pub boss_spawn_timer: Timer,
}

// game is 30 min
impl WaveSpawnConfig {
   pub fn default() -> Self {
      Self {
         // wave difficulty every 5 min
         wave_difficulty_timer: Timer::from_seconds(5.0 * 60.0, TimerMode::Once),
         // mob every 1 sec
         spawn_timer: Timer::from_seconds(1.0, TimerMode::Once),
         // boss every 10 min
         boss_spawn_timer: Timer::from_seconds(10.0 * 60.0, TimerMode::Once),
      }
   }
}

#[derive(Component, Debug)]
pub struct Enemy {
   pub speed: f32,
   pub health: u32,
   pub scale: f32,
}

impl Enemy {
   pub fn default(&self) -> Self {
      Self { speed: 100.0, health: 100, scale: 1.0 }
   }
}

#[derive(Component, Debug)]
pub struct Shooter {
   pub attack_timer: Timer,
}
