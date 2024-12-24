// attacks/mod.rs
pub mod components;
pub mod resources;
pub mod systems;

pub use components::*;
pub use resources::*;
pub use systems::*;

// NOTE: Possible attacks:
// Whip
// TestDagger intp Wand
//

pub mod dagger_attack;
pub use dagger_attack::*;
pub mod wasp_attack;
pub use wasp_attack::*;

use bevy::prelude::*;

use crate::GameState;

pub struct AttacksPlugin;

impl Plugin for AttacksPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      // RES
      app.insert_resource(ClosestEnemy::default());
      // ENTER
      app.add_systems(OnEnter(GameState::InGame), (add_dagger_attack));
      // UPDATE
      app.add_systems(
         FixedUpdate,
         (
            tick_attack_timers,
            timeout_dagger_attack_projectiles,
            set_closest_enemy,
            dagger_attack_system.after(set_closest_enemy),
         )
            .run_if(in_state(GameState::InGame)),
      );
   }
}
