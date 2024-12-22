use super::*;
use crate::{Player, WaveSpawnConfig, WaveState};
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn setup_waves() {}
pub fn waves_s(
   mut commands: Commands,
   time: Res<Time>,
   mut wave_state: ResMut<WaveState>,
   mut wave_spawn_config: ResMut<WaveSpawnConfig>,
) {
   if wave_spawn_config.spawn_timer.just_finished() {}
}

pub fn tick_wave_timers(
   time: Res<Time>,
   mut wave_spawn_config: ResMut<WaveSpawnConfig>,
) {
   wave_spawn_config.wave_difficulty_timer.tick(time.delta());
   wave_spawn_config.spawn_timer.tick(time.delta());
   wave_spawn_config.boss_spawn_timer.tick(time.delta());
}

pub fn despawn_low_tier_mobs_outside_view() {}
pub fn despawn_mobs_far_away() {}
pub fn get_random_location_around_player(t: &Transform) -> (f32, f32) {
   (0.0, 0.0)
}
pub fn spawn_mob(
   mut ws: ResMut<WaveState>,
   wsp: Res<WaveSpawnConfig>,
   mut cmd: Commands,
   sprites_collection: Res<SpritesCollection>,
   p_t: Single<&Transform, With<Player>>,
) {
   if wsp.spawn_timer.just_finished() && ws.mob_count < 1000 {
      let (x, y) = get_random_location_around_player(&p_t);
      match ws.difficulty {
         0 => spawn_slime(&mut cmd, &sprites_collection, x, y),
         _ => {}
      }
   }
}

pub fn advance_wave(
   mut ws: ResMut<WaveState>,
   wsp: Res<WaveSpawnConfig>,
) {
   if wsp.wave_difficulty_timer.just_finished() {
      ws.difficulty += 1;
   }
}
