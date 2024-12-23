// pub mod components;
// pub mod resources;
// pub mod systems;

// pub use components::*;
// pub use resources::*;
// pub use systems::*;

use crate::{get_sprite, GameState, Player, SpritesCollection};
use avian2d::prelude::*;
use bevy::prelude::{Transform, *};
use bevy_ecs_tilemap::prelude::*;
use rand::prelude::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      info!("TerrainPlugin disabled");
      return;
      app.add_systems(OnEnter(GameState::InGame), (setup_tilemap));
      app.add_systems(FixedUpdate, (update_tilemap).run_if(in_state(GameState::InGame)));
   }
}

#[derive(Resource)]
struct GrassTileData {
   texture_index: usize,
}

pub fn setup_tilemap(
   mut commands: Commands,
   sprites_collection: Res<SpritesCollection>,
   window: Single<&Window>,
) {
   let tile_size = TilemapTileSize { x: 12.0, y: 12.0 };
   let grid_size = tile_size.into();
   let map_size = TilemapSize { x: 100, y: 100 };

   let tilemap_e = commands.spawn_empty().id();
   let mut tile_storage = TileStorage::empty(map_size);
   // Populate the tilemap with initial tiles
   let mut rng = rand::thread_rng();
   for x in 0..map_size.x {
      for y in 0..map_size.y {
         let tile_pos = TilePos { x, y };
         let tile_entity = commands
            .spawn((TileBundle {
               position: tile_pos,
               tilemap_id: TilemapId(tilemap_e),
               texture_index: TileTextureIndex(rng.gen_range(0..=9)), // Grass tiles (0-9)
               ..Default::default()
            },))
            .id();
         tile_storage.set(&tile_pos, tile_entity);
      }
   }
   // Insert the tilemap
   let transform =
      get_tilemap_center_transform(&map_size, &grid_size, &TilemapType::default(), 0.0);
   commands.entity(tilemap_e).insert(TilemapBundle {
      grid_size,
      map_type: TilemapType::default(),
      size: map_size,
      storage: tile_storage,
      texture: TilemapTexture::Single(sprites_collection.tilemap.clone()),
      // texture: TilemapTexture::Vector(sprites_collection.map.values().cloned().collect()),
      tile_size,
      transform,
      ..Default::default()
   });
}
fn update_tilemap(
   mut query: Query<(&mut TileTextureIndex, &TilePos)>,
   time: Res<Time>,
   mut player: Single<&mut Player>,
   player_t: Single<&Transform, With<Player>>,
) {
   return;
   let current_time = time.elapsed_secs_f64();
   let cur_pos = player_t.translation;

   let mut rng = rand::thread_rng();
   let distance_moved = player.last_position.distance(cur_pos);

   if distance_moved > 100.0 {
      player.last_position = cur_pos;
      for (mut texture_index, tile_pos) in query.iter_mut() {
         // Change tiles near the player to something interesting
         if is_near_player(tile_pos, cur_pos) {
            texture_index.0 = rng.gen_range(10..=15); // Points of interest (10-15)
         }
      }
   }
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
