#![allow(unused)]
use avian2d::prelude::*;
use bevy::{
   app::AppExit,
   asset::LoadState,
   input::keyboard::KeyboardInput,
   prelude::*,
   render::{
      camera::ScalingMode,
      diagnostic::RenderDiagnosticsPlugin,
      mesh::{self, PrimitiveTopology},
      render_resource::{Extent3d, TextureDescriptor, TextureUsages},
      view::RenderLayers,
   },
   utils::{info, tracing::field::debug},
   window::{WindowResized, WindowResolution},
};
use bevy_asset_loader::prelude::*;
// use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use leafwing_input_manager::prelude::*;
use std::iter::zip;

mod animations;
mod attacks;
mod components;
mod doodads;
mod leveling;
mod loot;
mod menu;
mod pickups;
mod player;
mod postprocessing;
mod save_load;
mod sprites;
mod ui;
mod waves;

mod resources;
use resources::*;

use animations::*;
use attacks::*;
use doodads::*;
use leveling::*;
use loot::*;
use menu::*;
use pickups::*;
use player::*;
use postprocessing::*;
use save_load::*;
use sprites::*;
use ui::*;
use waves::*;

const RES_WIDTH: u32 = 640;
const RES_HEIGHT: u32 = 360;
const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);
const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

// == STATES ==
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
   #[default]
   Loading,
   MainMenu,
   InGame,
   Paused,
   LevelUp,
   GameOver,
}

#[derive(PhysicsLayer, Default)]
enum MyCollisionLayers {
   #[default]
   Default,
   Player,
   PlayerProjectile,
   Enemy,
   EnemyProjectile,
   Doodad,
   XpOrb,
   PowerUp,
}

// TODO: LAYUEARERAS

fn main() {
   let mut app = App::new();

   // BEVY PLUGINS
   app.add_plugins(
      (DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
         primary_window: Some(Window {
            resizable: false,
            decorations: false,
            // resolution: WindowResolution::new(1920., 1080.), //.with_scale_factor_override(2.0),
            ..default()
         }),
         ..default()
      })),
   );

   // LIB PLUGINS
   app.add_plugins((
      // TilemapPlugin,
      // WorldInspectorPlugin::new(),
      InputManagerPlugin::<Action>::default(),
   ));

   // PHYSICS PLUGIN
   app.add_plugins((
      // PhysicsDebugPlugin::default(),
      PhysicsPlugins::default().with_length_unit(10.0),
   ))
   .insert_resource(SubstepCount(3))
   .insert_resource(Gravity(Vec2::ZERO));

   // MY PLUGINS
   app.add_plugins((
      AttacksPlugin,
      WavesPlugin,
      PlayerPlugin,
      MenuPlugin,
      DoodadPlugin,
      PostProcessPlugin,
      LoadSpritesPlugin,
      InGameUiPlugin,
      LootPlugin,
      // SaveLoadPlugin,
      // TerrainPlugin,
   ));

   // STATE
   app.init_state::<GameState>();

   // some systems for camera and canvas
   app.add_systems(Startup, (setup_cameras, (fit_canvas_on_startup).after(setup_cameras)))
      .add_systems(FixedUpdate, (fit_canvas_to_window,).run_if(in_state(GameState::InGame)));

   // RUN
   app.run();
}
fn render_gizmos(mut gizmos: Gizmos) {
   // let sin_t_scaled = ops::sin(time.elapsed_secs()) * 50.;
   gizmos.arrow_2d(Vec2::ZERO, Vec2::ONE * 10., bevy::color::palettes::css::YELLOW);
}

fn setup_cameras(
   mut cmd: Commands,
   mut images: ResMut<Assets<Image>>,
) {
   let canvas_size = Extent3d { width: RES_WIDTH, height: RES_HEIGHT, ..Default::default() };
   let mut canvas = Image {
      texture_descriptor: TextureDescriptor {
         label: None,
         size: canvas_size,
         mip_level_count: 1,
         sample_count: 1,
         dimension: bevy::render::render_resource::TextureDimension::D2,
         format: bevy::render::render_resource::TextureFormat::Bgra8UnormSrgb,
         usage: TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_DST
            | TextureUsages::RENDER_ATTACHMENT,
         view_formats: &[],
      },
      ..Default::default()
   };
   canvas.resize(canvas_size);
   let image_handle = images.add(canvas);
   cmd.spawn((
      Name::new("Canvas Camera"),
      Camera2d,
      Camera {
         order: -1,
         clear_color: ClearColorConfig::Custom(Color::BLACK),
         target: bevy::render::camera::RenderTarget::Image(image_handle.clone()),
         ..default()
      },
      Msaa::Off,
      CanvasCamera,
      PIXEL_PERFECT_LAYERS,
   ));
   cmd.spawn((Sprite::from_image(image_handle), Canvas, HIGH_RES_LAYERS));
   let mut proj = OrthographicProjection::default_2d();
   proj.scaling_mode =
      ScalingMode::AutoMax { max_width: RES_WIDTH as f32, max_height: RES_HEIGHT as f32 };
   cmd.spawn((
      Name::new("Main Camera"),
      Camera2d,
      proj,
      Camera {
         clear_color: ClearColorConfig::Custom(Color::srgb(0.5, 0.75, 0.75)),
         hdr: true,
         ..default()
      },
      Msaa::Off,
      MainCamera,
      HIGH_RES_LAYERS,
   ));
}

fn fit_canvas_to_window(
   mut resize_events: EventReader<WindowResized>,
   mut projection: Single<&mut OrthographicProjection, With<MainCamera>>,
) {
   for e in resize_events.read() {
      info!("fit_canvas_to_window is disabled");
      return;
      let h_scale = e.width / RES_WIDTH as f32;
      let v_scale = e.height / RES_HEIGHT as f32;
      info!("switching from: {} to {}", projection.scale, (1. / h_scale.min(v_scale)));
      projection.scale = (1. / h_scale.min(v_scale));
   }
}

fn fit_canvas_on_startup(
   mut win: Single<&mut Window>,
   mut projection: Single<&mut OrthographicProjection, With<MainCamera>>,
) {
   return;
   let h_scale = win.width() / RES_WIDTH as f32;
   let v_scale = win.height() / RES_HEIGHT as f32;
   projection.scale = (1. / h_scale.min(v_scale));
}

// fn setup_misc(
//    mut cmd: Commands,
//    asset_server: Res<AssetServer>,
//    // mut windows: Query<&mut Window>,
//    art: Res<Art>,
// ) {
//    cmd.spawn((Name::new("Static Sprite"), Sprite::from_image(asset_server.load("test.png"))));
//    // info!("Spawned Static Test Sprite");
//    // test enemy
//    cmd.spawn((
//       Name::new("Some Enemy"),
//       Enemy,
//       Sprite::from_image(art.player.clone()),
//       Transform { translation: Vec3::new(100., 100., 0.), ..Default::default() },
//    ));
// }

fn setup_physics(mut commands: Commands) {
   /* Create the ground. */
   commands.spawn(Collider::rectangle(500.0, 50.0)).insert(Transform::from_xyz(0.0, -100.0, 0.0));

   /* Create the bouncing ball. */
   commands
      .spawn(RigidBody::Dynamic)
      .insert(Collider::circle(50.0))
      .insert(Restitution::new(0.7))
      .insert(Transform::from_xyz(0.0, 400.0, 0.0));
}

// fn check_loaded(
//    ass: Res<AssetServer>,
//    sprite_atlas: Res<MySpriteAtlas>,
//    sprites: Res<Sprites>,
// ) {
//     ass.get_load_state(id);
// }
