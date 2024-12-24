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
      app.add_systems(OnEnter(crate::GameState::InGame), (spawn_doodads));
   }
}

fn spawn_doodads(
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

// pub struct TerrainPlugin;

// impl Plugin for TerrainPlugin {
//    fn build(
//       &self,
//       app: &mut App,
//    ) {
//       info!("TerrainPlugin disabled");
//       return;
//       app.add_systems(OnEnter(GameState::InGame), (dummy));
//       app.add_systems(FixedUpdate, (dummy).run_if(in_state(GameState::InGame)));
//    }
// }

fn dummy() {}

#[derive(Resource)]
struct GrassTileData {
   texture_index: usize,
}

fn is_near_player(
   tile_pos: &TilePos,
   player_pos: Vec3,
) -> bool {
   let tile_center = Vec3::new(
      tile_pos.x as f32 * 12.0, // Assuming 16x16 tiles
      tile_pos.y as f32 * 12.0,
      0.0,
   );
   tile_center.distance(player_pos) < 100.0
}
// // fn check_player_moved_far_enough(
// //    mut cmd: Commands,
// //    sprites_collection: Res<SpritesCollection>,
// //    mut player: Single<&mut Player>,
// //    player_t: Single<&Transform, With<Player>>,
// //    window: Single<&Window>,
// // ) {
// //    let cur_pos = player_t.translation;
// //    // info!("curpos {}", cur_pos);
// //    let distance_moved = player.last_position.distance(cur_pos);
// //    if distance_moved > 100.0 {
// //       info!("moved 10.0, doodadding");
// //       player.last_position = cur_pos;
// //       add_grass_cluster_out_of_sprites(&mut cmd, &sprites_collection, cur_pos, window);
// //       add_points_of_interest(&mut cmd, &sprites_collection, cur_pos);
// //    }
// // }
// fn add_em(
//    mut cmd: Commands,
//    sprites_collection: Res<SpritesCollection>,
//    window: Single<&Window>,
// ) {
//    let mut rng = rand::thread_rng();
//    let rect = window.physical_size();
//    info!("win size: {}", window.size());
//    info!("win phys size: {}", window.physical_size());
//    let mut bounds = Vec2::new(window.size().x, window.size().y);
//    info!("bounds {}", bounds);
//    bounds.x /= 2.0;
//    // bounds.x /= 2.0;
//    bounds.y /= 2.0;
//    // bounds.y /= 2.0;
//    info!("bounds {}", bounds);
//    for x in (-bounds.x as i32..bounds.x as i32).step_by(100) {
//       for y in (-bounds.y as i32..bounds.y as i32).step_by(100) {
//          let sprite_name = format!("bones{}", rng.gen_range(1..=3));
//          let sprite = get_sprite(&mut cmd, &sprites_collection, &sprite_name);
//          cmd.spawn((
//             RigidBody::Static,
//             Friction::new(0.0),
//             Collider::circle(4.0),
//             sprite,
//             Transform::from_xyz(x as f32, y as f32, 0.0),
//          ));
//       }
//    }
// }
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
// </funcs>
// <context>
// Bevy Engine
// I have a SpriteCollection of sprites that are in a hashmap
// I have a get_sprite func that creates a sprite when called by name like: get_sprite(cmd,
// sprites, "grass1");
// there are grass{0-9}.png in there and some other doodads like bones.
// dont use the asset_server.load() at all I already handle all of that.
// Points of interest, are like a firepit doodad surrounded by bones or something, or a barrel in
// the bushes
// </context>
// <goal>
// When player moves out of some range it populates the area with more clusters of grass and points
// of interest.
// </goal>
