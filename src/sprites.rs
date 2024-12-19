use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;

// #[derive(Resource)]
// pub struct Sprites {
//    pub player: Sprite,
//    pub test_doodad: Sprite,
//    pub cursor: Sprite,
// }

pub struct LoadSpritesPlugin;

#[derive(AssetCollection, Resource, Reflect)]
#[reflect(Resource)]
pub struct SpritesCollection {
   #[asset(path = "sprites", collection(typed, mapped))]
   pub map: HashMap<String, Handle<Image>>,
   // pub player: Sprite,
   // pub test_doodad: Sprite,
   // pub cursor: Sprite,
}

impl Plugin for LoadSpritesPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      app.register_type::<SpritesCollection>();
      app.add_loading_state(
         LoadingState::new(crate::GameState::Loading)
            .load_collection::<SpritesCollection>()
            .continue_to_state(crate::GameState::MainMenu),
      );
   }
}

// #[derive(Resource)]
// pub struct MySpriteAtlas {
//    pub atlas_layout: Handle<TextureAtlasLayout>,
//    pub atlas_texture: Handle<Image>,
// }

// pub fn create_sprites(
//    mut cmd: Commands,
//    // ass: Res<AssetServer>,
//    // mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
//    // mut textures: ResMut<Assets<Image>>,
//    // mut sprites: Res<Sprites>,
//    mut sprite_atlas: Res<MySpriteAtlas>,
// ) {
//    fn create_sprite(
//       sprite_atlas: &Res<'_, MySpriteAtlas>,
//       row: usize,
//       col: usize,
//    ) -> Sprite {
//       Sprite {
//          image: sprite_atlas.atlas_texture.clone(),
//          texture_atlas: Some(TextureAtlas {
//             layout: sprite_atlas.atlas_layout.clone(),
//             index: index_it(col, row),
//          }),
//
//          ..Default::default()
//       }
//    }
//    let player = Sprite {
//       image: sprite_atlas.atlas_texture.clone(),
//       texture_atlas: Some(TextureAtlas {
//          layout: sprite_atlas.atlas_layout.clone(),
//          index: index_it(26, 40),
//       }),
//
//       ..Default::default()
//    };
//    let test_doodad = Sprite {
//       image: sprite_atlas.atlas_texture.clone(),
//       texture_atlas: Some(TextureAtlas {
//          layout: sprite_atlas.atlas_layout.clone(),
//          index: index_it(43, 37),
//       }),
//       ..Default::default()
//    };
//    cmd.insert_resource(SpritesCollection {
//       player: create_sprite(&sprite_atlas, 26, 40),
//       test_doodad: create_sprite(&sprite_atlas, 43, 37),
//       cursor: create_sprite(&sprite_atlas, 76, 43),
//    });
//    info!("Created sptires Res");
// }

// fn index_it(
//    col: usize,
//    row: usize,
// ) -> usize {
//    // NOTE: This works only with the current tileset: 103 width
//    (row * 103) + col
// }

// pub fn load_sprite_atlas(
//    mut cmd: Commands,
//    ass: Res<AssetServer>,
//    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
//    // mut textures: ResMut<Assets<Image>>,
//    // mut sprites: Res<Sprites>,
//    // mut sprite_atlas: Res<MySpriteAtlas>,
// ) {
//    // let basic_projectile_handle: Handle<Image> = ass.load("test.png");
//    // cmd.insert_resource(ProjectileArt { basic_projectile: basic_projectile_handle });
//    let atlas_layout_handle = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
//       UVec2::splat(12),
//       103,
//       50,
//       Some(UVec2::splat(1)),
//       Some(UVec2::splat(1)),
//    ));
//    let atlas_texture = ass.load("urizen_onebit_tileset.png");
//
//    // let player = Sprite::from_atlas_image(atlas_texture, atlas_layout_handle);
//    cmd.spawn(());
//    cmd.insert_resource(MySpriteAtlas { atlas_layout: atlas_layout_handle, atlas_texture });
//    info!("Loaded sprite atlas Res");
// }

pub fn get_sprite(
   commands: &mut Commands,
   sprites_collection: &Res<SpritesCollection>,
   sprite_name: &str,
) -> (Sprite, Name) {
   if let Some(cursor_image_handle) =
      sprites_collection.map.get(&format!("sprites/{}.png", sprite_name))
   {
      return (
         Sprite::from_image(cursor_image_handle.clone()),
         Name::new(format!("{} Sprite", sprite_name)),
      );
   } else {
      error!("missing {}.png", sprite_name);
      error!("sprites.iter -> ");
      for i in sprites_collection.map.iter() {
         info!("{:?}", i);
      }
      info!("tried to get: {}", format!("sprites/{}", sprite_name));
      return (Sprite::default(), Name::new("Missing Sprite"));
   }
}
