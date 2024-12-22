// ui or some shit
// pub mod components;
// pub mod resources;
// pub mod systems;

// pub use components::*;
// pub use resources::*;
// pub use systems::*;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

use crate::GameState;

pub struct InGameUiPlugin;

#[derive(Component)]
pub struct FpsText;

impl Plugin for InGameUiPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      app.add_plugins(FrameTimeDiagnosticsPlugin);
      app.add_systems(Startup, (setup_fps_ui));
      app.add_systems(Update, (draw_fps));
   }
}

fn setup_fps_ui(
   mut cmd: Commands,
   ass: Res<AssetServer>,
) {
   let root = cmd
      .spawn((
         Text::new("FPS: "),
         TextFont { font: ass.load("fonts/PPMondwest-Regular.otf"), font_size: 33.0, ..default() },
      ))
      .with_child((
         TextSpan::default(),
         (
            TextFont {
               font: ass.load("fonts/PPMondwest-Regular.otf"),
               font_size: 33.0,
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
