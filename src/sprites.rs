use bevy::prelude::*;

#[derive(Resource)]
pub struct Sprites {
   pub player: Sprite,
   pub test_doodad: Sprite,
}

#[derive(Resource)]
pub struct MySpriteAtlas {
   pub atlas_layout: Handle<TextureAtlasLayout>,
   pub atlas_texture: Handle<Image>,
}

pub fn create_sprites(
   mut cmd: Commands,
   // ass: Res<AssetServer>,
   // mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
   // mut textures: ResMut<Assets<Image>>,
   // mut sprites: Res<Sprites>,
   mut sprite_atlas: Res<MySpriteAtlas>,
) {
   let player = Sprite {
      image: sprite_atlas.atlas_texture.clone(),
      texture_atlas: Some(TextureAtlas {
         layout: sprite_atlas.atlas_layout.clone(),
         index: index_it(26, 40),
      }),

      ..Default::default()
   };
   let test_doodad = Sprite {
      image: sprite_atlas.atlas_texture.clone(),
      texture_atlas: Some(TextureAtlas {
         layout: sprite_atlas.atlas_layout.clone(),
         index: index_it(43, 37),
      }),
      ..Default::default()
   };
   cmd.insert_resource(Sprites { player, test_doodad });
}

fn index_it(
   col: usize,
   row: usize,
) -> usize {
   // NOTE: This works only with the current tileset: 103 width
   (row * 103) + col
}

pub fn load_sprite_atlas(
   mut cmd: Commands,
   ass: Res<AssetServer>,
   mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
   // mut textures: ResMut<Assets<Image>>,
   // mut sprites: Res<Sprites>,
   // mut sprite_atlas: Res<MySpriteAtlas>,
) {
   // let basic_projectile_handle: Handle<Image> = ass.load("test.png");
   // cmd.insert_resource(ProjectileArt { basic_projectile: basic_projectile_handle });
   let atlas_layout_handle = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
      UVec2::splat(12),
      103,
      50,
      Some(UVec2::splat(1)),
      Some(UVec2::splat(1)),
   ));
   let atlas_texture = ass.load("urizen_onebit_tileset.png");

   // let player = Sprite::from_atlas_image(atlas_texture, atlas_layout_handle);
   cmd.spawn(());
   cmd.insert_resource(MySpriteAtlas { atlas_layout: atlas_layout_handle, atlas_texture });
}
