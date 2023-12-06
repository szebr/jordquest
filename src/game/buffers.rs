use bevy::prelude::*;

pub const BUFFER_LEN: usize = 32;

pub struct CircularBuffer<T: Default + Copy>([T; BUFFER_LEN], [u16; BUFFER_LEN]);

impl<T: Default + Copy> CircularBuffer<T> {

    pub fn new() -> CircularBuffer<T> {
        return CircularBuffer([T::default(); BUFFER_LEN], [0; BUFFER_LEN]);
    }

    pub fn new_from(item: T) -> CircularBuffer<T> {
        return CircularBuffer([item; BUFFER_LEN], [0; BUFFER_LEN]);
    }

    pub fn get(&self, tick: u16) -> &T {
        let i = tick as usize % BUFFER_LEN;
        &self.0[i]
    }

    pub fn set(&mut self, tick: u16, input: T) {
        let i = tick as usize % BUFFER_LEN;
        self.0[i] = input;
    }

    pub fn set_with_time(&mut self, tick: u16, input: T, recv_date: u16) {
        let i = tick as usize % BUFFER_LEN;
        if recv_date > self.1[i] {
            self.0[i] = input;
            self.1[i] = recv_date;
        }
    }

    pub fn get_both(&mut self, tick: u16) -> (&T, u16) {
        let i = tick as usize % BUFFER_LEN;
        (&self.0[i], self.1[i])
    }
}

#[derive(Component)]
pub struct PosBuffer(pub CircularBuffer<Option<Vec2>>);

#[derive(Component)]
pub struct DirBuffer(pub CircularBuffer<Option<f32>>);

#[derive(Component)]
pub struct EventBuffer(pub CircularBuffer<Option<u8>>);

#[derive(Component)]
pub struct HpBuffer(pub CircularBuffer<Option<u8>>);
