use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct AnimationConfig {
   pub first_sprite_idx: usize,
   pub last_sprite_idx: usize,
   pub fps: u8,
   pub frame_timer: Timer,
}

impl AnimationConfig {
   pub fn new(
      first_sprite_idx: usize,
      last_sprite_idx: usize,
      fps: u8,
   ) -> Self {
      Self { first_sprite_idx, last_sprite_idx, fps, frame_timer: Self::timer_from_fps(fps) }
   }
   pub fn timer_from_fps(fps: u8) -> Timer {
      Timer::new(Duration::from_secs_f32(1. / (fps as f32)), TimerMode::Once)
   }
}

#[derive(Component)]
struct LeftSprite;

#[derive(Component)]
struct RightSprite;
