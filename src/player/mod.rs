// player/mod.rs
pub mod components;
pub mod resources;
pub mod systems;

pub use components::*;
pub use resources::*;
pub use systems::*;

use bevy::prelude::*;

use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      // add_systems(OnEnter(AppState::InGame), ())
      app.add_systems(
         FixedUpdate,
         (toggle_pause_ingame).run_if(in_state(GameState::InGame).or(in_state(GameState::Paused))),
      );
      app.add_systems(OnEnter(GameState::InGame), (setup_player)).add_systems(
         FixedUpdate,
         (
            player_moves,
            move_player,
            // fit_canvas_to_window,
            emit_player_moved_far_enough,
            follow_cam.after(move_player),
            handle_player_collisions.after(move_player),
            // render_gizmos,
         )
            .run_if(in_state(GameState::InGame)),
      );
      app.add_event::<PlayerMoveEvent>();
      app.add_event::<PlayerMovedFarEnough>();
      app.add_event::<GamePaused>();
      app.add_event::<GameUnPaused>();
   }
}
