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
      // .add_systems(Update, ().run_if(in_state(AppState::InGame)));
      // app.add_systems(OnEnter(GameState::MainMenu), (setup_cursor)).add_systems(
      //    FixedUpdate,
      //    (draw_cursor).run_if(in_state(GameState::InGame).or(in_state(GameState::MainMenu))),
      // );
      app.add_systems(OnEnter(GameState::InGame), (setup_player)).add_systems(
         FixedUpdate,
         (
            player_moves,
            move_player,
            // fit_canvas_to_window,
            follow_cam.after(move_player),
            handle_player_collisions.after(move_player),
            // render_gizmos,
         )
            .run_if(in_state(GameState::InGame)),
      );
      app.add_event::<PlayerMoveEvent>();
   }
}
