use bevy::prelude::*;
use crate::game::input;

use self::lerp::lerp_pos;

pub mod lerp;

pub const TICKRATE: u8 = 30;
const TICKLEN_S: f32 = 1. / TICKRATE as f32;
pub const DELAY: u16 = 2;
pub const BUFFER_SIZE: usize = 32;  // matters for ACK, untested, must be pwr of 2

#[derive(Resource)]
pub struct TickNum(pub u16);  // this is the tick we're writing to, NOT playing back

pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(FixedUpdate, increment_tick)
            .add_systems(Update, lerp_pos);
    }
}

pub fn startup(mut commands: Commands) {
    commands.insert_resource(FixedTime::new_from_secs(TICKLEN_S));
    commands.insert_resource(TickNum { 0: 0 });
}

// might want to split client functions and host functions into different files

// this definitely takes an event or something LOL
/*pub fn handle_packet_client() {
    // receive UDP packet and handle it
}*/

pub fn increment_tick(mut tick_num: ResMut<TickNum>) {
    tick_num.0 += 1;
}

// these below are for conditionally running systems, they're obviously placeholders
fn is_host() -> bool {
    true
}

fn is_client() -> bool {
    false
}