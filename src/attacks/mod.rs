// attacks/mod.rs
pub mod components;
pub mod resources;
pub mod systems;

pub use components::*;
pub use resources::*;
pub use systems::*;

use bevy::prelude::*;

use crate::AppState;

pub struct AttacksPlugin;

impl Plugin for AttacksPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      app.add_systems(OnEnter(AppState::InGame), (setup_weapons)).add_systems(
         Update,
         (timeout_despawn_projectiles, move_projectiles, weapons_system)
            .run_if(in_state(AppState::InGame)),
      );
   }
}
