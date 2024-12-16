// mod components;
// mod resources;
// mod systems;

use bevy::prelude::*;
// use components::*;
// use resources::*;
// use systems::*;

use crate::GameState;

pub struct LevelingPlugin;

impl Plugin for LevelingPlugin {
   fn build(
      &self,
      app: &mut bevy::prelude::App,
   ) {
      app.add_systems(OnEnter(GameState::LevelUp), (setup_levelup_ui_s))
         .add_systems(Update, (levelup_menu_s).run_if(in_state(GameState::LevelUp)));
   }
}

pub fn levelup_menu_s(commands: Commands) {}
pub fn setup_levelup_ui_s(commands: Commands) {}
pub fn clean_levelup_ui(commands: Commands) {}
