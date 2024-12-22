// pub mod components;
// pub mod resources;
// pub mod systems;

// pub use components::*;
// pub use resources::*;
// pub use systems::*;

use crate::{get_sprite, GameState, Player, SpritesCollection};
use avian2d::prelude::*;
use bevy::prelude::{Transform, *};
use log::info;
use rand::prelude::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      app.add_systems(OnEnter(GameState::InGame), (add_em));
      app.add_systems(
         FixedUpdate,
         (check_player_moved_far_enough).run_if(in_state(GameState::InGame)),
      );
   }
}

// prompt:
// i am making a bevy game and i need some help.
// <funcs>
fn check_player_moved_far_enough(
   mut cmd: Commands,
   sprites_collection: Res<SpritesCollection>,
   mut player: Single<&mut Player>,
   player_t: Single<&Transform, With<Player>>,
   window: Single<&Window>,
) {
   let cur_pos = player_t.translation;
   // info!("curpos {}", cur_pos);
   let distance_moved = player.last_position.distance(cur_pos);
   if distance_moved > 100.0 {
      info!("moved 10.0, doodadding");
      player.last_position = cur_pos;
      add_grass_cluster_out_of_sprites(&mut cmd, &sprites_collection, cur_pos, window);
      add_points_of_interest(&mut cmd, &sprites_collection, cur_pos);
   }
}
fn add_em(
   mut cmd: Commands,
   sprites_collection: Res<SpritesCollection>,
   window: Single<&Window>,
) {
   let mut rng = rand::thread_rng();
   let rect = window.physical_size();
   info!("win size: {}", window.size());
   info!("win phys size: {}", window.physical_size());
   let mut bounds = Vec2::new(window.size().x, window.size().y);
   info!("bounds {}", bounds);
   bounds.x /= 2.0;
   // bounds.x /= 2.0;
   bounds.y /= 2.0;
   // bounds.y /= 2.0;
   info!("bounds {}", bounds);
   for x in (-bounds.x as i32..bounds.x as i32).step_by(100) {
      for y in (-bounds.y as i32..bounds.y as i32).step_by(100) {
         let sprite_name = format!("bones{}", rng.gen_range(1..=3));
         let (name, sprite) = get_sprite(&mut cmd, &sprites_collection, &sprite_name);
         cmd.spawn((
            RigidBody::Static,
            Friction::new(0.0),
            Collider::circle(4.0),
            name,
            sprite,
            Transform::from_xyz(x as f32, y as f32, 0.0),
         ));
      }
   }
}
fn add_grass_cluster_out_of_sprites(
   cmd: &mut Commands,
   sprites_collection: &Res<SpritesCollection>,
   cur_pos: Vec3,
   window: Single<&Window>,
) {

   // for _ in 0..=32 {
   //    let sprite_name = format!("grass{}", rng.gen_range(0..=9));
   //    let (x, y);
   //    // if rng.gen_bool(0.5) {
   //    if rng.gen_bool(0.5) {
   //       x = rng.gen_range(-100..0) as f32;
   //    } else {
   //       x = rng.gen_range(rect.x..rect.x + 100) as f32;
   //    }
   //    // }
   //    // if rng.gen_bool(0.5) {
   //    if rng.gen_bool(0.5) {
   //       y = rng.gen_range(-100..0) as f32;
   //    } else {
   //       y = rng.gen_range(rect.y..rect.y + 100) as f32;
   //    }
   //    // }
   //    let pos = cur_pos + Vec3::new(x, y, 0.0);
   //    let (name, sprite) = get_sprite(cmd, &sprites_collection, &sprite_name);
   //    cmd.spawn((name, sprite, Transform::from_translation(pos)));
   // }
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
