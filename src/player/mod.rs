// player/mod.rs
pub mod components;
pub mod resources;
pub mod systems;

pub use components::*;
pub use resources::*;
pub use systems::*;

use bevy::prelude::*;

use crate::AppState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      app
         // add_systems(OnEnter(AppState::InGame), ())
         // .add_systems(Update, ().run_if(in_state(AppState::InGame)));
         .add_systems(OnEnter(AppState::InGame), (setup_player))
         .add_systems(
            Update,
            (
               wasd_hardcoded_player_movemement,
               // fit_canvas_to_window,
               follow_cam.after(wasd_hardcoded_player_movemement),
               // render_gizmos,
            )
               .run_if(in_state(AppState::InGame)),
         );
   }
}
