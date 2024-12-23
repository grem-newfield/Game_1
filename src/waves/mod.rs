// player/mod.rs
pub mod components;
pub mod resources;
pub mod systems;

use std::f32::consts::{SQRT_2, TAU};

pub use components::*;
use rand::Rng;
pub use resources::*;
pub use systems::*;

use crate::{GameState, SpritesCollection};
use bevy::prelude::*;

pub struct WavesPlugin;

impl Plugin for WavesPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      app.add_systems(OnEnter(GameState::InGame), (setup_waves)).add_systems(
         FixedUpdate,
         (
            melee_ai,
            rangers_ai,
            //
            move_enemy_projectiles,
            fade_debris_system,
            wave_system,
            timeout_enemy_projectiles,
            ranged_boss_attack,
         )
            .run_if(in_state(GameState::InGame)),
      );
      app.insert_resource(WaveSpawnConfig::default()).insert_resource(WaveState::default());
   }
}
pub fn spawn_test_enemies(
   mut cmd: Commands,
   sprites_collection: Res<SpritesCollection>,
) {
   let mut rng = rand::thread_rng();
   for _ in 0..200 {
      let x = rng.gen_range(-1000..1000) as f32;
      let y = rng.gen_range(-1000..1000) as f32;
      if rng.gen_bool(0.75) {
         spawn_slime(&mut cmd, &sprites_collection, x, y);
      } else {
         spawn_kobold_archer(&mut cmd, &sprites_collection, x, y);
      }
   }
}
