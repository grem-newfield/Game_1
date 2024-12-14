// player/mod.rs
pub mod components;
pub mod resources;
pub mod systems;

pub use components::*;
pub use resources::*;
pub use systems::*;

use bevy::prelude::*;

use crate::AppState;

pub struct WavesPlugin;

impl Plugin for WavesPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      app.add_systems(OnEnter(AppState::InGame), (setup_waves))
         .add_systems(Update, (waves_s).run_if(in_state(AppState::InGame)));
   }
}
