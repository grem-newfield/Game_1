// player/mod.rs
pub mod components;
// pub mod resources;
pub mod systems;

pub use components::*;
// pub use resources::*;
pub use systems::*;

use bevy::prelude::*;

use crate::GameState;

pub struct PickupPlugin;

impl Plugin for PickupPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      app.add_systems(OnEnter(GameState::InGame), (dummy))
         .add_systems(Update, (dummy).run_if(in_state(GameState::InGame)));
   }
}
