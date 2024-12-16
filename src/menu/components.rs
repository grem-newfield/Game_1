use bevy::prelude::*;

#[derive(Component)]
pub enum MenuButtonAction {
   Play,
   Other,
   Quit,
}

#[derive(Component)]
struct SelectedOption;
