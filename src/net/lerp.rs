use std::{time::Duration, ops::Mul};

// TODO: Make movement.rs file in game folder
// put all of this in there?
use bevy::prelude::*;
use crate::net;

#[derive(Component)]
pub struct PositionBuffer(pub [Vec2; net::BUFFER_SIZE]);

impl PositionBuffer {
    pub fn get(&self, tick: u16) -> &Vec2 {
        let i = tick as usize % net::BUFFER_SIZE;
        &self.0[i]
    }

    //TODO pub fn getMut(&mut self, tick:u16) -> &mut PlayerTick

    pub fn set(&mut self, tick: u16, input: Vec2) {
        let i = tick as usize % net::BUFFER_SIZE;
        self.0[i] = input;
    }
}

pub fn lerp_pos(
    tick_time: Res<FixedTime>,
    tick: Res<net::TickNum>,
    mut query: Query<(&mut Transform, &PositionBuffer)>,
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

/// a generalized function to move a movable unit
/// takes a mutable reference to that unit's bufferedposition component
/// the current ticknum
/// the fixedtime resource's duration (Res<FixedTime>.period)
/// a directional vector
/// and a speed for the unit
pub fn move_unit(bp: &mut PositionBuffer, tick: u16, timestep: Duration, dir: &Vec2, speed: f32) {
    let dir = dir.normalize_or_zero();
    let prev = bp.get(tick.wrapping_sub(1));
    let new_pos: Vec2 = dir.mul(timestep.as_secs_f32() * speed);
    bp.set(tick, *prev+new_pos);
}
// TODO: check collusion and zero out movement if it would collide?

