#![allow(unused)]
use std::iter::zip;

use avian2d::prelude::*;
use bevy::{
   app::AppExit,
   input::keyboard::KeyboardInput,
   prelude::*,
   render::{
      mesh::{self, PrimitiveTopology},
      render_resource::{Extent3d, TextureDescriptor, TextureUsages},
      view::RenderLayers,
   },
   utils::info,
   window::WindowResolution,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod components;
use components::{
   tags::{Actor, Enemy, MainCamera},
   AttackTimer, Player, Projectile,
};
mod resources;
use resources::*;

const RES_WIDTH: u32 = 192;
const RES_HEIGHT: u32 = 108;
/// Default render layers for pixel-perfect rendering.
/// You can skip adding this component, as this is the default.
const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);
/// Render layers for high-resolution rendering.
const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

#[derive(Component)]
struct Canvas;

#[derive(Component)]
struct InGameCamera;

#[derive(Component)]
struct OuterCamera;

#[derive(Component)]
struct Rotate;

fn main() {
   App::new()
      .add_plugins((
         DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            primary_window: Some(Window {
               resizable: false,
               decorations: false,
               resolution: WindowResolution::new(1920. / 2., 1080. / 2.),
               // .with_scale_factor_override(1.0),
               ..default()
            }),
            ..default()
         }),
         PhysicsPlugins::default(),
         PhysicsDebugPlugin::default(),
         WorldInspectorPlugin::new(),
      ))
      // .add_plugins(())
      .add_systems(Startup, (setup, setup_weapons, setup_physics))
      .add_systems(
         Update,
         (timeout_projectiles, move_projectiles, move_player, weapons_system, follow_cam),
      )
      .run();
}

fn setup_camera(
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
         target: bevy::render::camera::RenderTarget::Image(image_handle.clone()),
         ..Default::default()
      },
      Msaa::Off,
      InGameCamera,
      PIXEL_PERFECT_LAYERS,
   ));
}

fn setup(
   mut cmd: Commands,
   asset_server: Res<AssetServer>,
   mut windows: Query<&mut Window>,
) {
   let window = windows.get_single_mut().unwrap();

   //camera
   cmd.spawn((MainCamera, Camera2d::default()));

   //test sprite
   let width = window.resolution.width() / 2.;
   let height = window.resolution.height() / 2.;

   cmd.spawn((Player { speed: 100.0 }, Sprite::from_image(asset_server.load("test.png"))));
   cmd.spawn(Sprite::from_image(asset_server.load("test.png")));
   // .with_children(|children| {
   // children.spawn(RigidBody::Dynamic).with_children(|children| {
   // children
   // .spawn(Collider::cuboid(1.0, 2.0))
   // .insert(Sensor)
   // .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
   // .insert(Friction::coefficient(0.7))
   // .insert(Restitution::coefficient(0.3))
   // .insert(ColliderMassProperties::Density(2.0));
   // });
   // });

   info!("Spawned Test Sprite");
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
   mut q: Query<&mut Transform, With<Player>>,
   mut p: Query<&Player>,
) {
   // for mut velocity in &mut query {
   //         velocity.y -= 9.8 * DELTA;
   //
   for (mut transform, player) in zip(&mut q, &p) {
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
      }
      if keys.pressed(KeyCode::KeyS) {
         // transform.translation.y -= player.speed * time.delta_seconds();
         move_vec.y -= 1.0; //player.speed * time.delta_secs();
      }
      if keys.pressed(KeyCode::KeyD) {
         // transform.translation.x += player.speed * time.delta_seconds();
         move_vec.x += 1.0; //player.speed * time.delta_secs();
      }
      if move_vec != Vec3::ZERO {
         transform.translation += move_vec.normalize() * player.speed * time.delta_secs();
      }
   }
}

fn follow_cam(
   mut cam: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
   target: Query<&Transform, (With<Player>, Without<MainCamera>)>,
   time: Res<Time>,
) {
   // works
   // add lerp
   cam.single_mut().translation = cam
      .single()
      .translation
      .lerp(target.single().translation, (-700.0 * time.delta_secs()).exp2());
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
   // mut q_timers: Query<(Entity, &mut AttackTimer)>,
   mut q_timers: Query<(Entity, &mut AttackTimer)>,
) {
   for (e, mut attack_timer) in q_timers.iter_mut() {
      attack_timer.timer.tick(time.delta());
      if attack_timer.timer.finished() {
         // info!("Timer Lmao");
         // summon projectile
         c.spawn(
            ((
               Projectile { speed: 50.0, lifetime: 1.0 },
               Transform::from_xyz(0.0, 0.0, 0.0),
               Sprite::from_image(ass.load("test.png")),
               RigidBody::Kinematic,
               Collider::circle(2.0),
               DebugRender::default().with_collider_color(Color::srgb(0.0, 1.0, 0.0)),
            )),
         );
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
