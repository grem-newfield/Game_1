use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Shooter {
   pub attack_timer: Timer,
}
