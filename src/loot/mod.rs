use crate::{
   get_sprite, Action, AttackTimer, DaggerAttack, Enemy, EnemyDied, EnemyKind, GameState,
   MyCollisionLayers, Player, PlayerMoveEvent, ProjectileArt, SpritesCollection, XpOrb,
};
use avian2d::prelude::*;
use bevy::{ecs::bundle, prelude::*};
use leafwing_input_manager::prelude::*;

pub struct LootPlugin;

impl Plugin for LootPlugin {
   fn build(
      &self,
      app: &mut App,
   ) {
      app.add_systems(
         FixedUpdate,
         (handle_enemydied, collide_orbs).run_if(in_state(GameState::InGame)),
      );
      // .add_systems(FixedUpdate, (fit_canvas_to_window,));
   }
}
fn dummy() {}

fn handle_enemydied(
   mut cmd: Commands,
   mut er: EventReader<EnemyDied>,
   sprites_collection: Res<SpritesCollection>,
   mut player: Single<&mut Player>,
) {
   for e in er.read() {
      let sprite = get_sprite(&mut cmd, &sprites_collection, "crystal");
      cmd.spawn((
         XpOrb,
         Name::new("Exp Orb"),
         sprite,
         Transform::from_xyz(e.x, e.y, 0.0),
         RigidBody::Dynamic,
         Friction::new(0.0),
         Collider::circle(12.0),
         LockedAxes::ROTATION_LOCKED,
         CollisionLayers::new(MyCollisionLayers::XpOrb, [MyCollisionLayers::Player]),
         CollidingEntities::default(),
      ));
      player.kills += 1;
      player.experience += e.experience;
      if player.experience >= 100 {
         player.experience -= 100;
         player.level += 1;
         // TODO: summon screen with weapon update
      }
   }
}

pub fn collide_orbs(
   mut cmd: Commands,
   q: Query<(Entity, &CollidingEntities), With<XpOrb>>,
   // q: Query<(Entity, &CollidingEntities), With<Player>>,
   p: Single<Entity, With<Player>>,
) {
   for (orb, coll) in q.iter() {
      if !coll.is_empty() {
         cmd.entity(orb).despawn();
      }
   }
}
