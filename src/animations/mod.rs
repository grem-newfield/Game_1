mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use components::*;
use resources::*;
use systems::*;

pub struct AnimationPlugin;

impl Plugin for AnimationConfig {
   fn build(
      &self,
      app: &mut bevy::prelude::App,
   ) {
      app.add_systems(Update, execute_animations);
   }
}
