#![allow(unused)]
const RES_WIDTH: u32 = 192 * 2;
const RES_HEIGHT: u32 = 108 * 2;
const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);
const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

// == STATES ==

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
   // Loading,
   #[default]
   MainMenu,
   InGame,
   Paused,
   LevelUp,
   GameOver,
}

// == MENU ==
mod menu {
   use super::AppState;
   use bevy::prelude::*;
   #[derive(Resource)]
   pub struct MenuData {
      pub menu_entity: Entity,
   }

   const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
   const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
   const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

   pub fn setup_menu(mut cmd: Commands) {
      let play_button_entity = cmd
         .spawn(Node {
            // center button
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
         })
         .with_children(|parent| {
            parent
               .spawn((
                  Button,
                  Node {
                     width: Val::Px(150.0),
                     height: Val::Px(65.0),
                     justify_content: JustifyContent::Center,
                     align_items: AlignItems::Center,
                     ..Default::default()
                  },
                  BackgroundColor(NORMAL_BUTTON_COLOR),
               ))
               .with_children(|parent| {
                  parent.spawn((
                     Text::new("Play"),
                     TextFont { font_size: 30.0, ..Default::default() },
                     TextColor(Color::WHITE),
                  ));
               });
         })
         .id();
      let menu_horizontal_container_e = cmd
         .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
         })
         .with_children(|parent| {
            // parent.enqueue_command(cmd.entity(play_button_entity));
         })
         .id();
      cmd.insert_resource(MenuData { menu_entity: menu_horizontal_container_e });
   }

   pub fn menu(
      mut next_state: ResMut<NextState<AppState>>,
      mut interaction_query: Query<
         (&Interaction, &mut BackgroundColor),
         (Changed<Interaction>, With<Button>),
      >,
   ) {
      for (inter, mut color) in &mut interaction_query {
         match *inter {
            Interaction::Pressed => {
               *color = PRESSED_BUTTON_COLOR.into();
               next_state.set(AppState::InGame);
            }
            Interaction::Hovered => {
               *color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
               *color = NORMAL_BUTTON_COLOR.into();
            }
         }
      }
   }

   pub fn cleanup_menu(
      mut cmd: Commands,
      menu_data: Res<MenuData>,
   ) {
      cmd.entity(menu_data.menu_entity).despawn_recursive();
   }
}
// == /MENU ==
fn main() {
   App::new()
      .add_plugins((
         DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            primary_window: Some(Window {
               resizable: false,
               decorations: false,
               resolution: WindowResolution::new(1920., 1080.).with_scale_factor_override(1.0),
               ..default()
            }),
            ..default()
         }),
         PhysicsPlugins::default(),
         // PhysicsDebugPlugin::default(),
         WorldInspectorPlugin::new(),
         // SomeDiagnosticsPlugin,
         AttacksPlugin,
         WavesPlugin,
         PlayerPlugin,
      ))
      .init_state::<AppState>()
      // .add_plugins(())
      // .insert_resource(CameraState { projection_scale: 1.0 })
      .add_systems(Startup, (load_sprites, setup_cameras))
      .add_systems(OnEnter(AppState::MainMenu), setup_menu)
      .add_systems(Update, menu.run_if(in_state(AppState::MainMenu)))
      .add_systems(OnExit(AppState::MainMenu), cleanup_menu)
      // .add_systems(OnEnter(AppState::InGame), (setup_physics))
      .add_systems(Update, (fit_canvas_to_window,).run_if(in_state(AppState::InGame)))
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
         clear_color: ClearColorConfig::Custom(Color::srgb(0.5, 0.75, 0.75)),
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
      Camera { clear_color: ClearColorConfig::Custom(Color::BLACK), ..Default::default() },
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
fn load_sprites(
   mut cmd: Commands,
   ass: Res<AssetServer>,
   // mut sprites: Res<Assets<Image>>,
) {
   let basic_projectile_handle: Handle<Image> = ass.load("test.png");
   cmd.insert_resource(ProjectileArt { basic_projectile: basic_projectile_handle });
   let player_sprite_handle: Handle<Image> = ass.load("test.png");
   cmd.insert_resource(Art { player: player_sprite_handle });
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

// fn collide_actors(
//    mut c: Commands,
//    q: Query<&Actor>,
// ) {
// }

// fn collide_projectiles(
//    mut c: Commands,
//    q: Query<&Projectile>,
// ) {
// }
//
// fn collide(
//    mut c: Commands,
//    query_player: Query<&Transform, With<Player>>,
//    query_enemies: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
// ) {
// }
//

// fn setup_physics(mut commands: Commands) {
/* Create the ground. */
// commands
//    .spawn(Collider::cuboid(500.0, 50.0))
//    .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));
//
// /* Create the bouncing ball. */
// commands
//    .spawn(RigidBody::Dynamic)
//    .insert(Collider::ball(50.0))
//    .insert(Restitution::coefficient(0.7))
//    .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
// }

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
use menu::*;
use std::iter::zip;
mod components;

// mod systems;
// use systems::*;
mod resources; // Art n shit
use resources::*;
mod animations;
use animations::*;
mod attacks;
use attacks::*;
mod player;
use player::*;
mod waves;
use waves::*;
