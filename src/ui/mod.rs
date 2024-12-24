// ui or some shit
// pub mod components;
// pub mod resources;
// pub mod systems;

// pub use components::*;
// pub use resources::*;
// pub use systems::*;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, time::Stopwatch};

use crate::{get_sprite, move_player, GameState, MainCamera, Player, SpritesCollection};

#[derive(Component)]
pub struct FpsText;

pub struct InGameUiPlugin;
impl Plugin for InGameUiPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      app.add_plugins(FrameTimeDiagnosticsPlugin);
      app.add_systems(Startup, (setup_fps_ui,)); // setup_ui));
                                                 //setup_cursor
      app.add_systems(FixedUpdate, (draw_fps));
      app.add_systems(FixedUpdate, (tick_game_timer).run_if(in_state(GameState::InGame)));
      app.insert_resource(GameTimer::default());

      // app.add_systems(FixedUpdate, (move_cursor));
   }
}

fn setup_fps_ui(
   mut cmd: Commands,
   ass: Res<AssetServer>,
) {
   let root = cmd
      .spawn((
         Text::new("FPS: "),
         TextFont { font: ass.load("fonts/PPMondwest-Regular.otf"), font_size: 20.0, ..default() },
      ))
      .with_child((
         TextSpan::default(),
         (
            TextFont {
               font: ass.load("fonts/PPMondwest-Regular.otf"),
               font_size: 20.0,
               ..default()
            },
            TextColor(bevy::color::palettes::css::GOLD.into()),
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

// fn setup_ui(
//    mut cmd: Commands,
//    ass: Res<AssetServer>,
//    game_timer: Res<GameTimer>,
//    p: Single<&Player>,
//    // game_state: Res<GameState>,
// ) {
//    let font_handle = ass.load("fonts/PPMondwest-Regular.otf");
//
//    // Timer Text
//    cmd.spawn((
//       Text::new(
//          format!("Time: {:.2} min", game_timer.timer.elapsed_secs() / 60.0),
//          // TextStyle { font: font_handle.clone(), font_size: 33.0, color: Color::WHITE },
//       ),
//       TimerText,
//    ));
//
//    // XP Bar and Level Text
//    cmd.spawn((
//       (
//          Text::new(format!("XP: {:.2} | Level: {}", p.xp, p.level)),
//          // TextStyle { font: font_handle.clone(), font_size: 33.0, color: Color::WHITE },
//       ),
//       XpBarText,
//       LevelText,
//    ));
//
//    // Kills Text
//    cmd.spawn_bundle(TextBundle {
//       text: Text::from_section(
//          format!("Kills: {}", game_state.kills),
//          TextStyle { font: font_handle, font_size: 33.0, color: Color::WHITE },
//       ),
//       ..Default::default()
//    })
//    .insert(KillsText);
// }
//
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
