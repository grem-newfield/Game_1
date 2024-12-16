use crate::{Credit, Health, Player, PlayerState, PowerUp, Sprites};
use bevy::prelude::*;

pub fn spawn_pickups_s(
   mut c: Commands,
   mut art: Res<Sprites>,
) {
   // c.spawn((Sprite::from_image(art.  a  .clone())));
}

pub fn collect_pickups_system(
   mut c: Commands,
   pickup_q_credits: Query<(Entity, &Transform), With<Credit>>,
   pickup_q_health: Query<(Entity, &Transform), With<Health>>,
   pickup_q_powerup: Query<(Entity, &Transform), With<PowerUp>>,
   player: Single<&Transform, With<Player>>,
   player_state: Res<PlayerState>,
) {
   for (e, pickup_t) in pickup_q_credits.iter() {
      if player.translation.distance(pickup_t.translation) < player_state.pickup_range {
         c.entity(e).despawn();
         // credits logic
      }
   }
   for (e, pickup_t) in pickup_q_health.iter() {
      if player.translation.distance(pickup_t.translation) < player_state.pickup_range {
         c.entity(e).despawn();
         // health logic
      }
   }
   for (e, pickup_t) in pickup_q_powerup.iter() {
      if player.translation.distance(pickup_t.translation) < player_state.pickup_range {
         c.entity(e).despawn();
         // powerup logic
      }
   }
}
pub fn dummy() {}
