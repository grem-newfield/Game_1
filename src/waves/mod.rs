// player/mod.rs
pub mod components;
pub mod resources;
pub mod systems;

use std::f32::consts::{SQRT_2, TAU};

pub use components::*;
pub use resources::*;
pub use systems::*;

use crate::GameState;
use bevy::prelude::*;

pub struct WavesPlugin;

impl Plugin for WavesPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      app.add_systems(OnEnter(GameState::InGame), (setup_waves))
         .add_systems(Update, (waves_s).run_if(in_state(GameState::InGame)));
   }
}
