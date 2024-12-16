#![allow(unused)]
const RES_WIDTH: u32 = 640;
const RES_HEIGHT: u32 = 360;
const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);
const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

// TODO:
// 1. Fix the STATES switching
// 2. Finish the GAME

// == STATES ==
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
   // Loading,
   #[default]
   MainMenu,
   InGame,
   Paused,
   LevelUp,
   GameOver,
}

fn main() {
   App::new()
      .add_plugins((
         // PhysicsDebugPlugin::default(),
         // SomeDiagnosticsPlugin,
         DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            primary_window: Some(Window {
               resizable: false,
               decorations: false,
               resolution: WindowResolution::new(1920. / 2., 1080. / 2.), //.with_scale_factor_override(1.0),
               ..default()
            }),
            ..default()
         }),
         PhysicsPlugins::default(),
         WorldInspectorPlugin::new(),
         AttacksPlugin,
         WavesPlugin,
         PlayerPlugin,
         MenuPlugin,
         DoodadPlugin,
      ))
      .init_state::<GameState>()
      // .add_plugins(())
      // .insert_resource(CameraState { projection_scale: 1.0 })
      .add_systems(
         Startup,
         (
            load_sprite_atlas,
            (create_sprites).after(load_sprite_atlas),
            setup_cameras,
            (fit_canvas_on_startup).after(setup_cameras),
         ),
      )
      .add_systems(OnEnter(GameState::InGame), (setup_physics))
      .add_systems(Update, (fit_canvas_to_window,).run_if(in_state(GameState::InGame)))
      .run();
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
         ..Default::default()
      },
      Msaa::Off,
      InGameCamera,
      PIXEL_PERFECT_LAYERS,
   ));
   cmd.spawn((Sprite::from_image(image_handle), Canvas, HIGH_RES_LAYERS));
   cmd.spawn((
      Name::new("Main Camera"),
      Camera2d,
      Camera {
         clear_color: ClearColorConfig::Custom(Color::srgb(0.5, 0.75, 0.75)),
         ..Default::default()
      },
      Msaa::Off,
      OuterCamera,
      HIGH_RES_LAYERS,
   ));
}

fn fit_canvas_to_window(
   mut resize_events: EventReader<WindowResized>,
   mut projection: Single<&mut OrthographicProjection, With<OuterCamera>>,
) {
   for e in resize_events.read() {
      let h_scale = e.width / RES_WIDTH as f32;
      let v_scale = e.height / RES_HEIGHT as f32;
      projection.scale = (1. / h_scale.min(v_scale).round());
   }
}

fn fit_canvas_on_startup(
   mut win: Single<&mut Window>,
   mut projection: Single<&mut OrthographicProjection, With<OuterCamera>>,
) {
   let h_scale = win.width() / RES_WIDTH as f32;
   let v_scale = win.height() / RES_HEIGHT as f32;
   projection.scale = (1. / h_scale.min(v_scale).round());
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

use avian2d::prelude::*;
use bevy::{
   app::AppExit,
   input::keyboard::KeyboardInput,
   prelude::*,
   render::{
      diagnostic::RenderDiagnosticsPlugin,
      mesh::{self, PrimitiveTopology},
      render_resource::{Extent3d, TextureDescriptor, TextureUsages},
      view::RenderLayers,
   },
   utils::info,
   window::{WindowResized, WindowResolution},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::iter::zip;

mod animations;
mod attacks;
mod components;
mod doodads;
mod leveling;
mod menu;
mod pickups;
mod player;
mod resources; // Art n shit
mod sprites;
mod waves;

use animations::*;
use attacks::*;
use doodads::*;
use leveling::*;
use menu::*;
use pickups::*;
use player::*;
use resources::*;
use sprites::*;
use waves::*;

// mod systems;
// use systems::*;
