// attacks/mod.rs
pub mod components;
pub mod resources;
pub mod systems;

pub use components::*;
pub use resources::*;
pub use systems::*;

use bevy::prelude::*;

use crate::GameState;

pub struct AttacksPlugin;

impl Plugin for AttacksPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      app.add_systems(OnEnter(GameState::InGame), (setup_weapons)).add_systems(
         Update,
         (timeout_despawn_projectiles, move_projectiles, weapons_system)
            .run_if(in_state(GameState::InGame)),
      );
   }
}
