use bevy::prelude::*;

pub const BUFFER_LEN: usize = 32;

pub struct CircularBuffer<T: Default>([T; BUFFER_LEN]);

impl<T: Default> CircularBuffer<T> {

    pub fn new() -> CircularBuffer<T> {
        return CircularBuffer([T::default(); BUFFER_LEN]);
    }

    pub fn get(&self, tick: u16) -> &T {
        let i = tick as usize % BUFFER_LEN;
        &self.0[i]
    }

    pub fn get_mut(&mut self, tick: u16) -> &mut T {
        let i = tick as usize % BUFFER_LEN;
        &mut self.0[i]
    }

    pub fn set(&mut self, tick: u16, input: T) {
        let i = tick as usize % BUFFER_LEN;
        self.0[i] = input;
    }
}

#[derive(Component)]
pub struct PosBuffer(CircularBuffer<Vec2>);

#[derive(Component)]
pub struct DirBuffer(CircularBuffer<f32>);

#[derive(Component)]
pub struct HpBuffer(CircularBuffer<f32>);
