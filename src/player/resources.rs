use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
   Up,
   Down,
   Left,
   Right,
   Pause, // TODO: maybe add pause for menu in game pause ?
}

impl Action {
   pub const DIRECTIONS: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];
   pub fn direction(self) -> Option<Dir2> {
      match self {
         Self::Up => Some(Dir2::Y),
         Self::Down => Some(Dir2::NEG_Y),
         Self::Left => Some(Dir2::NEG_X),
         Self::Right => Some(Dir2::X),
         _ => None,
      }
   }
}

#[derive(Event)]
pub struct PlayerMoveEvent {
   pub direction: Dir2,
}

#[derive(Event)]
pub struct GamePaused;

#[derive(Event)]
pub struct GameUnPaused;

#[derive(Event)]
pub struct PlayerMovedFarEnough;
