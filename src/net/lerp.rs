use bevy::prelude::*;
use crate::game::player::LocalPlayer;
use crate::net;

// TODO: Should this be moved?
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
    mut query: Query<(&mut Transform, &PositionBuffer), Without<LocalPlayer>>,
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