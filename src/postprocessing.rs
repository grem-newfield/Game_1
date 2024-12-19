use bevy::{prelude::*, render::render_resource::AsBindGroup};

use crate::{Canvas, GameState};

pub struct PostProcessPlugin;

impl Plugin for PostProcessPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      // TODO: later
      // app.add_systems(OnEnter(GameState::InGame), (setup_postprocessing_for_canvas));
   }
}

pub fn setup_postprocessing_for_canvas(
   mut cmd: Commands,
   mut materials: ResMut<Assets<CanvasMaterial>>,
   mut canvas: Single<&Sprite, With<Canvas>>,
) {
   let canvas_material = materials.add(CanvasMaterial {
      color_texture: Some(canvas.image.clone()),
      alpha_mode: AlphaMode::Blend,
   });
}

pub fn setup_postprocessing_for_ingame_sprites(mut cmd: Commands) {}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CanvasMaterial {
   #[texture(0)]
   #[sampler(1)]
   color_texture: Option<Handle<Image>>,
   alpha_mode: AlphaMode,
}
const SHADER_ASSET_PATH: &str = "shaders/canvas_mat.wgsl";
impl Material for CanvasMaterial {
   fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
      SHADER_ASSET_PATH.into()
   }

   fn alpha_mode(&self) -> AlphaMode {
      AlphaMode::Opaque
   }
}
