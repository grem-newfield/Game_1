use super::*;
use bevy::prelude::*;
// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
pub fn execute_animations(
   time: Res<Time>,
   mut query: Query<(&mut AnimationConfig, &mut Sprite)>,
) {
   for (mut config, mut sprite) in &mut query {
      // we track how long the current sprite has been displayed for
      config.frame_timer.tick(time.delta());

      // If it has been displayed for the user-defined amount of time (fps)...
      if config.frame_timer.just_finished() {
         if let Some(atlas) = &mut sprite.texture_atlas {
            if atlas.index == config.last_sprite_idx {
               // ...and it IS the last frame, then we move back to the first frame and stop.
               atlas.index = config.first_sprite_idx;
            } else {
               // ...and it is NOT the last frame, then we move to the next frame...
               atlas.index += 1;
               // ...and reset the frame timer to start counting all over again
               config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
         }
      }
   }
}

fn trigger_animation<S: Component>(mut anim: Single<&mut AnimationConfig, With<S>>) {
   anim.frame_timer = AnimationConfig::timer_from_fps(anim.fps);
}

fn setup(
   mut cmd: Commands,
   ass: Res<AssetServer>,
   mut atlas: ResMut<Assets<TextureAtlasLayout>>,
) {
   let player_atlas_tex: Handle<Image> = ass.load("SpaceSoldier.png");
   let layout = TextureAtlasLayout::from_grid(
      UVec2::splat(50),
      8,
      2,
      Some(UVec2::new(1, 1)),
      Some(UVec2::new(1, 1)),
   );
   let player_atlas_layout: Handle<TextureAtlasLayout> = atlas.add(layout);
}
