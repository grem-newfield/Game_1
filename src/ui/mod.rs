// ui or some shit
// pub mod components;
// pub mod resources;
// pub mod systems;

// pub use components::*;
// pub use resources::*;
// pub use systems::*;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, time::Stopwatch};

use crate::{GameState, Player};

#[derive(Component)]
pub struct FpsText;

pub struct InGameUiPlugin;
impl Plugin for InGameUiPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      app.add_plugins(FrameTimeDiagnosticsPlugin);
      app.add_systems(Startup, (setup_fps_ui)); // setup_ui));
      app.add_systems(FixedUpdate, (draw_fps));
      app.add_systems(FixedUpdate, (tick_game_timer).run_if(in_state(GameState::InGame)));
      app.insert_resource(GameTimer::default());
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
         if let Some(value) = fps.smoothed() {
            // Update the value of the second section
            **span = format!("{value:.2}");
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
