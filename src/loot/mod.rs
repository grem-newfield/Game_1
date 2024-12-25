use crate::{
   get_sprite, Action, AttackTimer, DaggerAttack, Enemy, EnemyDied, EnemyKind, GameState,
   PlayerMoveEvent, ProjectileArt, SpritesCollection,
};
use avian2d::prelude::*;
use bevy::{ecs::bundle, prelude::*};
use leafwing_input_manager::prelude::*;

pub struct EnemyLootPlugin;

impl Plugin for EnemyLootPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      app.add_systems(FixedUpdate, (dummy).run_if(in_state(GameState::InGame)));
      // .add_systems(FixedUpdate, (fit_canvas_to_window,));
   }
}
fn dummy() {}

fn spawn_exp_orb(
   mut cmd: Commands,
   mut er: EventReader<EnemyDied>,
   sprites_collection: &Res<SpritesCollection>,
) {
   for e in er.read() {
      let sprite = get_sprite(&mut cmd, &sprites_collection, "crystal");
      cmd.spawn((
         Name::new("Exp Orb"),
         sprite,
         Transform::from_xyz(e.x, e.y, 0.0),
         RigidBody::Dynamic,
         Friction::new(0.0),
         Collider::circle(4.0),
         LockedAxes::ROTATION_LOCKED,
      ));
   }
}
