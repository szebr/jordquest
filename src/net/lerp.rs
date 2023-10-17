use bevy::prelude::*;
use crate::game::buffers::PosBuffer;
use crate::game::player::LocalPlayer;
use crate::net;

pub fn lerp_pos(
    tick_time: Res<FixedTime>,
    tick: Res<net::TickNum>,
    mut query: Query<(&mut Transform, &PosBuffer), Without<LocalPlayer>>,
) {
    for (mut tf, bp) in &mut query {
        let next_state = bp.get(tick.0.wrapping_sub(net::DELAY));
        let prev_state = bp.get(tick.0.wrapping_sub(net::DELAY + 1));
        let percent: f32 = tick_time.accumulated().as_secs_f32() / tick_time.period.as_secs_f32();
        let new_state = prev_state.lerp(*next_state, percent);
        tf.translation.x = new_state.x;
        tf.translation.y = new_state.y;
    }
}