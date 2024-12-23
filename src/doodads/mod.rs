// mod components;
// mod resources;
// mod systems;

use std::f32::consts::PI;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{get_sprite, SpritesCollection};

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

fn spawn_doodads_2(
   mut commands: Commands,
   sprites_collection: Res<crate::SpritesCollection>,
) {
   // cmd.spawn((Name::new("Static Sprite"), Sprite::from_image(asset_server.load("test.png"))));
   // info!("Spawned Static Test Sprite");
   // test enemy
   let (sprite, name) = crate::get_sprite(&mut commands, &sprites_collection, "candelabra");
   commands.spawn((
      sprite,
      name,
      Transform { translation: Vec3::new(100., 100., 0.), ..Default::default() },
   ));
}
fn spawn_doodads(
   mut commands: Commands,
   sprites_collection: Res<crate::SpritesCollection>,
) {
   let num_candelabras = 32;
   let radius = 300.0;

   for i in 0..num_candelabras {
      // Calculate the angle for each candelabra
      let angle = 2.0 * PI * (i as f32) / (num_candelabras as f32);
      // Calculate the position based on the angle and radius
      let x = radius * angle.cos();
      let y = radius * angle.sin();

      // Get the sprite and name for the candelabra
      let (sprite, name) = crate::get_sprite(&mut commands, &sprites_collection, "candelabra");

      // Spawn the candelabra with the calculated position
      commands.spawn((
         sprite,
         name,
         Transform { translation: Vec3::new(x, y, 0.0), ..Default::default() },
      ));
   }
}
