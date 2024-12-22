use std::collections::btree_set::Range;

use crate::{get_sprite, Player, SpritesCollection};
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Enemy {
   life: f32,
   speed: f32,
   damage: f32,
   knockback_resistance: f32,
   xp: u32,
}

#[derive(Component, Clone)]
pub struct Ranged {
   pub damage: f32,
   pub range: f32,
   pub cooldown: f32,
   pub projectile_speed: f32,
}

#[derive(Component, Clone)]
pub struct EnemyProjectile {
   pub damage: f32,
}

pub fn spawn_slime(
   cmd: &mut Commands,
   sprites_collection: &Res<SpritesCollection>,
   x: f32,
   y: f32,
) {
   let (name, sprite) = get_sprite(cmd, sprites_collection, "slime");
   cmd.spawn((
      name,
      sprite,
      Enemy { life: 1.0, speed: 50.0, damage: 1.0, knockback_resistance: 0.0, xp: 10 },
      Transform::from_xyz(x, y, 0.0),
      RigidBody::Dynamic,
      Friction::new(0.0),
      Collider::circle(4.0),
      LockedAxes::ROTATION_LOCKED,
   ));
}

pub fn spawn_kobold_archer(
   cmd: &mut Commands,
   sprites_collection: &Res<SpritesCollection>,
   x: f32,
   y: f32,
) {
   let (name, sprite) = get_sprite(cmd, sprites_collection, "kobold_archer");
   cmd.spawn((
      name,
      sprite,
      Enemy { life: 1.0, speed: 10.0, damage: 1.0, knockback_resistance: 0.0, xp: 10 },
      Ranged { damage: 1.0, range: 100.0, cooldown: 2.0, projectile_speed: 50.0 },
      Transform::from_xyz(x, y, 0.0),
   ));
}

pub fn primitive_melee_ai_move(
   mut q: Query<(&mut Transform, &Enemy), (With<Enemy>, Without<Ranged>, Without<Player>)>,
   p: Single<&Transform, With<Player>>,
   time: Res<Time>,
) {
   for (mut et, enemy) in q.iter_mut() {
      let direction = p.translation - et.translation;
      let distance = direction.length();
      let move_direction = direction.normalize();
      let move_amount = move_direction * enemy.speed * time.delta_secs();
      et.translation += move_amount;
   }
}

pub fn melee_ai_move(
   mut q: Query<
      (&Transform, &mut LinearVelocity, &Enemy),
      (With<Enemy>, Without<Ranged>, Without<Player>),
   >,
   p: Single<&Transform, With<Player>>,
   time: Res<Time>,
) {
   let my_span = info_span!("melee_ai_move", name = "melee_ai_move").entered();
   for (mut et, mut v, enemy) in q.iter_mut() {
      let mut direction = p.translation - et.translation;
      direction = direction.normalize();
      v.x = direction.x * enemy.speed;
      v.y = direction.y * enemy.speed;
   }
}

pub fn rangers_ai_move(
   q: Query<Entity, (With<Enemy>, With<Ranged>)>,
   p: Single<&Transform, With<Player>>,
) {
}
pub fn rangers_ai_shoot(
   q: Query<Entity, (With<Enemy>, With<Ranged>)>,
   p: Single<&Transform, With<Player>>,
) {
}

// maybe swarm_ai lmao
