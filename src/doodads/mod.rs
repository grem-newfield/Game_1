// mod components;
// mod resources;
// mod systems;

use bevy::prelude::*;
// use components::*;
// use resources::*;
// use systems::*;

pub struct DoodadPlugin;

impl Plugin for DoodadPlugin {
   fn build(
      &self,
      app: &mut bevy::prelude::App,
   ) {
      app.add_systems(OnEnter(crate::GameState::InGame), (spawn_doodads));
   }
}

fn spawn_doodads(
   mut commands: Commands,
   sprites_collection: Res<crate::SpritesCollection>,
) {
   // cmd.spawn((Name::new("Static Sprite"), Sprite::from_image(asset_server.load("test.png"))));
   // info!("Spawned Static Test Sprite");
   // test enemy
   let (sprite, name) = crate::get_sprite(&mut commands, &sprites_collection, "candelabra");
   commands.spawn((
      // crate::Enemy{...},
      // Sprite::from_image(sprites.map["candelabra"].clone()),
      sprite,
      name,
      Transform { translation: Vec3::new(100., 100., 0.), ..Default::default() },
   ));
}
