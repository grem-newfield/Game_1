// ui or some shit
// pub mod components;
// pub mod resources;
// pub mod systems;

// pub use components::*;
// pub use resources::*;
// pub use systems::*;

use bevy::{
   color::palettes::css::GOLD, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, time::Stopwatch,
};

use crate::{get_sprite, move_player, GameState, MainCamera, Player, SpritesCollection};

#[derive(Component)]
pub struct FpsText;

pub struct InGameUiPlugin;
impl Plugin for InGameUiPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      // app.add_plugins(FrameTimeDiagnosticsPlugin);
      // app.add_systems(Startup, (setup_fps_ui));
      // app.add_systems(FixedUpdate, (draw_fps));
      app.add_systems(OnEnter(GameState::InGame), (setup_hud));
      app.add_systems(
         FixedUpdate,
         (tick_game_timer, update_timer, update_xpbar, update_kills, update_level)
            .run_if(in_state(GameState::InGame)),
      );
      app.insert_resource(GameTimer::default());
   }
}

fn setup_fps_ui(
   mut cmd: Commands,
   ass: Res<AssetServer>,
) {
   let root = cmd
      .spawn((
         Node { align_items: AlignItems::End, ..default() },
         Text::new("FPS: "),
         TextFont { font: ass.load("fonts/PPMondwest-Regular.otf"), font_size: 30.0, ..default() },
      ))
      .with_child((
         TextSpan::default(),
         (
            TextFont {
               font: ass.load("fonts/PPMondwest-Regular.otf"),
               font_size: 30.0,
               ..default()
            },
            TextColor(GOLD.into()),
         ),
         FpsText,
      ));
}
fn draw_fps(
   diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
   mut query: Query<&mut TextSpan, With<FpsText>>,
) {
   for mut span in &mut query {
      if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
         if let Some(fps) = fps.smoothed() {
            **span = format!("{fps:.0}");
         }
      }
   }
}

#[derive(Component)]
struct TimerText;

#[derive(Component)]
struct XpBarText;

#[derive(Component)]
struct LevelText;

#[derive(Component)]
struct KillsText;

#[derive(Resource)]
struct GameTimer {
   timer: Stopwatch,
}
impl Default for GameTimer {
   fn default() -> GameTimer {
      GameTimer { timer: Stopwatch::new() }
   }
}
fn tick_game_timer(
   mut t: ResMut<GameTimer>,
   time: Res<Time>,
) {
   t.timer.tick(time.delta());
}

fn setup_hud(
   mut cmd: Commands,
   ass: Res<AssetServer>,
   game_timer: Res<GameTimer>,
) {
   let font_handle = ass.load("fonts/PPMondwest-Regular.otf");

   let timer_text = (
      Text::new("Time: mm.ss"),
      TimerText,
      (
         TextFont { font: font_handle.clone(), font_size: 30.0, ..default() },
         TextColor(GOLD.into()),
      ),
   );
   let xpbar_text = (
      Text::new("XP: 0000"),
      XpBarText,
      (
         TextFont { font: font_handle.clone(), font_size: 30.0, ..default() },
         TextColor(GOLD.into()),
      ),
   );
   let level_text = (
      Text::new("Level 0"),
      LevelText,
      (
         TextFont { font: font_handle.clone(), font_size: 30.0, ..default() },
         TextColor(GOLD.into()),
      ),
   );
   let kills_text = (
      Text::new("Kills: 0"),
      KillsText,
      (
         TextFont { font: font_handle.clone(), font_size: 30.0, ..default() },
         TextColor(GOLD.into()),
      ),
   );

   let mut top_bar = cmd
      .spawn((
         Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_items: JustifyItems::Center,
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Relative,
            ..default()
         },
         // BackgroundColor(Color::WHITE),
      ))
      .with_children(|parent| {
         parent.spawn(timer_text);
         parent.spawn(xpbar_text);
         parent.spawn(level_text);
         parent.spawn(kills_text);
      })
      .id();
   info!("spawned: {:?}", top_bar)
}

fn update_timer(
   game_timer: Res<GameTimer>,
   player: Single<&Player>,
   mut timer_t: Single<&mut Text, With<TimerText>>,
) {
   let mut ss = game_timer.timer.elapsed_secs();
   let mm = (ss / 60.0).floor();
   ss = ss % 60.0;
   timer_t.0 = format!("Time: {:02}:{:02}", mm as u32, ss as u32);
}
fn update_xpbar(
   game_timer: Res<GameTimer>,
   player: Single<&Player>,
   mut xpbar_t: Single<&mut Text, With<XpBarText>>,
) {
   xpbar_t.0 = format!("XP: {:.0}", player.experience);
}

fn update_level(
   game_timer: Res<GameTimer>,
   player: Single<&Player>,
   mut level_t: Single<&mut Text, With<LevelText>>,
) {
   level_t.0 = format!("Level: {}", player.level);
}

fn update_kills(
   game_timer: Res<GameTimer>,
   player: Single<&Player>,
   mut kills_t: Single<&mut Text, With<KillsText>>,
) {
   kills_t.0 = format!("Kills: {}", player.kills);
}

// #[derive(Component)]
// pub struct CursorTag;

// pub fn setup_cursor(
//    mut commands: Commands,
//    sprites_collection: Res<SpritesCollection>,
//    mut win: Single<&mut Window>,
// ) {
//    win.cursor_options.visible = false;
//    if let Some(cur_img) = sprites_collection.map.get("cursor") {
//       commands
//          .spawn(
//             (Node {
//                width: Val::Percent(100.0),
//                height: Val::Percent(100.0),
//                flex_direction: FlexDirection::Column,
//                justify_content: JustifyContent::Center,
//                align_items: AlignItems::Center,
//                ..default()
//             }),
//          )
//          .with_children(|p| {
//             p.spawn((
//                Node {
//                   position_type: PositionType::Absolute,
//                   height: Val::Px(12.0),
//                   width: Val::Px(12.0),
//                   ..default()
//                },
//                MyCursor,
//                ZIndex(100),
//                ImageNode { image: cur_img.clone(), ..default() },
//             ));
//          });
//    }
//    // let sprite = get_sprite(&mut commands, &sprites_collection, "cursor");
//    // commands.spawn((sprite, Name::new("Cursor"), Transform::from_xyz(0.0, 0.0, 1000.0), CursorTag));
// }

// #[derive(Component)]
// pub struct MyCursor;
//
// pub fn move_cursor(
//    window: Single<&Window>,
//    mut node: Single<&mut Node, With<MyCursor>>,
//    // q: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
// ) {
//    if let Some(cur_pos) = window.cursor_position() {
//       node.left = Val::Px(cur_pos.x);
//       node.bottom = Val::Px(cur_pos.y);
//    };
//    // let (camera, camera_transform) = *q;
//    // return;
//    // let Ok(point) = camera.viewport_to_world_2d(camera_transform, cur_pos) else {
//    // return;
//    // };
//    // cursor_sprite.translation.x = point.x;
//    // cursor_sprite.translation.y = point.y;
//
//    // if let Some(cur_pos) = window.cursor_position()
//    // .and_then(|cursor| cam.viewport_to_world_2d(cam_trans,cursor)).map(|ray| ray.);
// }
