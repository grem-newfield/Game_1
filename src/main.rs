#![allow(unused)]
use std::iter::zip;

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
mod components;
use components::{
   tags::{Actor, Enemy},
   AttackTimer, Player, Projectile,
};
mod resources;
use resources::*;

mod animations;
use animations::*;

const RES_WIDTH: u32 = 192 * 2;
const RES_HEIGHT: u32 = 108 * 2;
/// Default render layers for pixel-perfect rendering.
/// You can skip adding this component, as this is the default.
const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);
/// Render layers for high-resolution rendering.
const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

// == CAMERA ==
#[derive(Component)]
struct Canvas;

#[derive(Component)]
struct InGameCamera;

#[derive(Component)]
struct OuterCamera;

// == STATES ==

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
   // Loading,
   #[default]
   MainMenu,
   InGame,
   Paused,
   GameOver,
}

// == MENU ==
#[derive(Resource)]
struct MenuData {
   button_entity: Entity,
}

const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

fn setup_menu(mut cmd: Commands) {
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
   cmd.insert_resource(MenuData { button_entity: play_button_entity });
}

fn menu(
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

fn cleanup_menu(
   mut cmd: Commands,
   menu_data: Res<MenuData>,
) {
   cmd.entity(menu_data.button_entity).despawn_recursive();
}

// == /MENU ==
fn main() {
   App::new()
      .add_plugins((
         DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            primary_window: Some(Window {
               resizable: true,
               decorations: false,
               resolution: WindowResolution::new(1920. / 2., 1080. / 2.),
               // .with_scale_factor_override(1.0),
               ..default()
            }),
            ..default()
         }),
         PhysicsPlugins::default(),
         // PhysicsDebugPlugin::default(),
         WorldInspectorPlugin::new(),
      ))
      .init_state::<AppState>()
      // .add_plugins(())
      // .insert_resource(CameraState { projection_scale: 1.0 })
      .add_systems(Startup, (load_sprites, setup_cameras))
      .add_systems(OnEnter(AppState::MainMenu), setup_menu)
      .add_systems(Update, menu.run_if(in_state(AppState::MainMenu)))
      .add_systems(OnExit(AppState::MainMenu), cleanup_menu)
      .add_systems(
         OnEnter(AppState::InGame),
         (setup_misc, setup_player, setup_weapons, setup_physics),
      )
      .add_systems(
         Update,
         (
            timeout_projectiles,
            move_projectiles,
            move_player,
            weapons_system,
            fit_canvas_to_window,
            follow_cam.after(move_player),
            // render_gizmos,
         )
            .run_if(in_state(AppState::InGame)),
      )
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
fn setup_player(
   mut cmd: Commands,
   asset_server: Res<AssetServer>,
   art: Res<Art>,
   // mut windows: Query<&mut Window>,
) {
   // let window = windows.get_single_mut().unwrap();
   // let width = window.resolution.width() / 2.;
   // let height = window.resolution.height() / 2.;
   cmd.spawn((
      Player { speed: 100.0 },
      Sprite {
         image: art.player.clone(),
         // custom_size: Some(Vec2::splat(50.)),
         ..Default::default()
      },
      Transform::from_xyz(0., 0., 0.).with_scale(Vec3::ONE),
   ));
}
fn setup_misc(
   mut cmd: Commands,
   asset_server: Res<AssetServer>,
   // mut windows: Query<&mut Window>,
) {
   cmd.spawn(Sprite::from_image(asset_server.load("test.png")));
   info!("Spawned Static Test Sprite");
}

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>) {
   if keys.just_pressed(KeyCode::Space) {
      info!("Space was pressed")
   }
   if keys.just_released(KeyCode::ControlLeft) {
      info!("Left Ctrl was released")
   }
   if keys.pressed(KeyCode::KeyW) {
      info!("W is being held down")
   }
   if keys.pressed(KeyCode::Escape) {
      info!("Exiting...")
   }
   // we can check multiple at once with `.any_*`
   if keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
      info!("Either the left or right shift are being held down")
   }
   if keys.any_just_pressed([KeyCode::Delete, KeyCode::Backspace]) {
      info!("Either delete or backspace was just pressed")
   }
}

fn move_player(
   time: Res<Time>,
   keys: Res<ButtonInput<KeyCode>>,
   // mut q: Query<&mut Transform, With<Player>>,
   // mut p: Query<&Player>,
   mut transform: Single<&mut Transform, With<Player>>,
   mut player: Single<&Player>,
) {
   // for mut velocity in &mut query {
   //         velocity.y -= 9.8 * DELTA;
   //
   // for (mut transform, player) in zip(&mut q, &p) {
   // for (mut transform, player) in zip(&mut q, &p) {
   // info!("{:?}", transform);
   // info!("{:?}", player);
   let mut move_vec = Vec3::ZERO;
   if keys.pressed(KeyCode::KeyW) {
      // transform.translation.y += player.speed * time.delta_seconds();
      move_vec.y += 1.0; // player.speed * time.delta_secs();
   }
   if keys.pressed(KeyCode::KeyA) {
      // transform.translation.x -= player.speed * time.delta_seconds();
      move_vec.x -= 1.0; //player.speed * time.delta_secs();
      transform.rotate_z(time.delta_secs() * 3.);
   }
   if keys.pressed(KeyCode::KeyS) {
      // transform.translation.y -= player.speed * time.delta_seconds();
      move_vec.y -= 1.0; //player.speed * time.delta_secs();
   }
   if keys.pressed(KeyCode::KeyD) {
      // transform.translation.x += player.speed * time.delta_seconds();
      move_vec.x += 1.0; //player.speed * time.delta_secs();
      transform.rotate_z(-time.delta_secs() * 3.);
   }
   if move_vec != Vec3::ZERO {
      transform.translation += move_vec.normalize() * player.speed * time.delta_secs();
   }
   // }
}

fn follow_cam(
   // mut cam: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
   mut main_cam: Single<
      &mut Transform,
      (With<OuterCamera>, Without<Player>, Without<InGameCamera>),
   >,
   mut pixel_cam: Single<
      &mut Transform,
      (With<InGameCamera>, Without<Player>, Without<OuterCamera>),
   >,
   mut canvas: Single<
      &mut Transform,
      (With<Canvas>, Without<Player>, Without<OuterCamera>, Without<InGameCamera>),
   >,
   // target: Query<&Transform, (With<Player>, Without<MainCamera>)>,
   target: Single<&Transform, (With<Player>, Without<OuterCamera>, Without<InGameCamera>)>,
   time: Res<Time>,
) {
   // cam.translation = cam.translation.lerp(target.translation, (-700.0 * time.delta_secs()).exp2());
   // cam.translation = cam.translation.lerp(target.translation, 1. * time.delta_secs());
   main_cam.translation = target.translation;
   canvas.translation = target.translation;
   pixel_cam.translation = target.translation;
}

fn collide_actors(
   mut c: Commands,
   q: Query<&Actor>,
) {
}

fn collide_projectiles(
   mut c: Commands,
   q: Query<&Projectile>,
) {
}

// fn collide(
//    mut c: Commands,
//    query_player: Query<&Transform, With<Player>>,
//    query_enemies: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
// ) {
// }
//
fn move_projectiles(
   mut c: Commands,
   time: Res<Time>,
   mut q: Query<(Entity, &mut Transform, &Projectile)>,
) {
   for (e, mut transform, projectile) in q.iter_mut() {
      transform.translation += Vec3::new(0.0, projectile.speed * time.delta_secs(), 0.0);
   }
}

fn timeout_projectiles(
   mut c: Commands,
   time: Res<Time>,
   mut q: Query<(Entity, &mut Projectile)>,
) {
   for (e, mut projectile) in q.iter_mut() {
      projectile.lifetime -= time.delta_secs();
      if projectile.lifetime <= 0.0 {
         c.entity(e).despawn();
      }
   }
}

fn setup_weapons(mut c: Commands) {
   c.spawn(AttackTimer { timer: Timer::from_seconds(1.0, TimerMode::Repeating) });
}

fn weapons_system(
   mut c: Commands,
   time: Res<Time>,
   ass: Res<AssetServer>,
   sprites: Res<Assets<Image>>,
   projectile_art: Res<ProjectileArt>,
   // mut q_timers: Query<(Entity, &mut AttackTimer)>,
   mut q_timers: Query<(Entity, &mut AttackTimer)>,
) {
   for (e, mut attack_timer) in q_timers.iter_mut() {
      attack_timer.timer.tick(time.delta());
      if attack_timer.timer.finished() {
         // info!("Timer Lmao");
         // summon projectile
         let mut ent_cmd = c.spawn(
            ((
               Projectile { speed: 50.0, lifetime: 1.0 },
               Transform::from_xyz(0.0, 0.0, 0.0),
               Sprite::from(projectile_art.basic_projectile.clone()),
               RigidBody::Kinematic,
               Collider::circle(2.0),
            )),
         );
         // if cfg!(debug_assertions) {
         //    ent_cmd.insert(DebugRender::default().with_collider_color(Color::srgb(0.0, 1.0, 0.0)));
         // }
      }
   }
}

fn setup_physics(mut commands: Commands) {
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
}
