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
            wasd_hardcoded_player_movemement,
            // fit_canvas_to_window,
            follow_cam.after(wasd_hardcoded_player_movemement),
            // render_gizmos,
         )
            .run_if(in_state(GameState::InGame)),
      );
   }
}
