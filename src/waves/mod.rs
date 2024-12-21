// player/mod.rs
pub mod components;
pub mod resources;
pub mod systems;

use std::f32::consts::{SQRT_2, TAU};

pub use components::*;
use rand::Rng;
pub use resources::*;
pub use systems::*;

pub mod enemies;
pub use enemies::*;

use crate::{GameState, SpritesCollection};
use bevy::prelude::*;

pub struct WavesPlugin;

impl Plugin for WavesPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      app.add_systems(OnEnter(GameState::InGame), (setup_waves, spawn_test_enemies))
         .add_systems(FixedUpdate, (waves_s, melee_ai_move).run_if(in_state(GameState::InGame)));
   }
}
pub fn spawn_test_enemies(
   mut cmd: Commands,
   sprites_collection: Res<SpritesCollection>,
) {
   let mut rng = rand::thread_rng();
   for _ in 0..2000 {
      let x = rng.gen_range(-1000..1000) as f32;
      let y = rng.gen_range(-1000..1000) as f32;
      spawn_slime(&mut cmd, &sprites_collection, x, y);
   }
}
