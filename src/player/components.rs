use bevy::prelude::*;
#[derive(Component, Debug)]
pub struct Player {
   pub speed: f32,
}
#[derive(Component)]
pub struct Canvas;
#[derive(Component)]
pub struct InGameCamera;
#[derive(Component)]
pub struct OuterCamera;
