// mod components;
// mod resources;
// mod systems;

// use components::*;
// use resources::*;
// use systems::*;

use std::f32::consts::PI;

use crate::{get_sprite, GameState, Player, SpritesCollection};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::prelude::*;

pub struct DoodadPlugin;

impl Plugin for DoodadPlugin {
   fn build(
      &self,
      app: &mut bevy::prelude::App,
   ) {
      app.add_systems(OnEnter(crate::GameState::InGame), (spawn_candelabras));
   }
}

fn spawn_candelabras(
   mut commands: Commands,
   sprites_collection: Res<crate::SpritesCollection>,
) {
   let num_candelabras = 32;
   let radius = 300.0;
   for i in 0..num_candelabras {
      let angle = 2.0 * PI * (i as f32) / (num_candelabras as f32);
      let x = radius * angle.cos();
      let y = radius * angle.sin();
      let sprite = crate::get_sprite(&mut commands, &sprites_collection, "candelabra");
      commands.spawn((Name::new("Candelabra Doodad"), sprite, Transform::from_xyz(x, y, 0.0)));
   }
}

/// spawns grass{0-9} sprites, around some <pos>
fn spawn_grass_cluster(
   cmd: &mut Commands,
   sprites_collection: &Res<SpritesCollection>,
   pos: Vec3,
   window: Single<&Window>,
) {
   let mut rng = rand::thread_rng();
   for _ in 0..=32 {
      let sprite_name = format!("grass{}", rng.gen_range(0..=9));
      let x_offset = rng.gen_range(-100..100) as f32;
      let y_offset = rng.gen_range(-100..100) as f32;
      let sprite_pos = Vec3::new(pos.x + x_offset, pos.y + y_offset, pos.z);
      let sprite = get_sprite(cmd, &sprites_collection, &sprite_name);
      cmd.spawn((Name::new("Grass Doodad"), sprite, Transform::from_translation(sprite_pos)));
   }
}

fn add_points_of_interest(
   cmd: &mut Commands,
   sprites_collection: &Res<SpritesCollection>,
   cur_pos: Vec3,
) {
}
