use bevy::prelude::*;
#[derive(Component, Debug)]
pub struct Player {
   pub speed: f32,
   pub last_position: Vec3,
}
#[derive(Component)]
pub struct Canvas;
#[derive(Component)]
pub struct CanvasCamera;
#[derive(Component)]
pub struct MainCamera;
#[derive(Component)]
pub struct CursorTag;
