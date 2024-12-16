// pub mod components;
// pub mod resources;
// pub mod systems;
//
// pub use components::*;
// pub use resources::*;
// pub use systems::*;

use bevy::prelude::*;

pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      app.add_systems(Startup, (setup_save_load));
   }
}

pub fn setup_save_load(mut cmd: Commands) {
   info!(" TODO (Stretch goal): Save / Load ");
}
