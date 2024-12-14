use crate::{WaveSpawnConfig, WaveState};
use bevy::prelude::*;

pub fn setup_waves() {}
pub fn waves_s(
   mut commands: Commands,
   time: Res<Time>,
   mut wave_state: ResMut<WaveState>,
   mut wave_spawn_config: Query<&mut WaveSpawnConfig>,
) {
}

pub fn tick_wave_timers(
   time: Res<Time>,
   mut wave_spawn_config: Single<&mut WaveSpawnConfig>,
) {
   wave_spawn_config.wave_difficulty_timer.tick(time.delta());
   wave_spawn_config.spawn_timer.tick(time.delta());
   wave_spawn_config.boss_spawn_timer.tick(time.delta());
}
