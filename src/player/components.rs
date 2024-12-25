use bevy::prelude::*;
#[derive(Component, Debug)]
pub struct Player {
   pub life: i32,
   pub speed: f32,
   pub last_position: Vec3,
   pub damage_cooldown: Timer,
   pub experience: f32,
   pub level: u32,
   pub kills: u32,
}
#[derive(Component)]
pub struct Canvas;
#[derive(Component)]
pub struct CanvasCamera;
#[derive(Component)]
pub struct MainCamera;
