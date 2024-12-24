use bevy::prelude::*;
#[derive(Component, Debug)]
pub struct Player {
   pub life: u32,
   pub speed: f32,
   pub last_position: Vec3,
   pub damage_cooldown: Timer,
}
#[derive(Component)]
pub struct Canvas;
#[derive(Component)]
pub struct CanvasCamera;
#[derive(Component)]
pub struct MainCamera;
