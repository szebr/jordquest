pub mod host;
pub mod client;
pub mod lerp;

use std::net::UdpSocket;
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::AppState;
use crate::game::{enemy, player, input};


pub const TICKRATE: u8 = 30;
const TICKLEN_S: f32 = 1. / TICKRATE as f32;
pub const DELAY: u16 = 2;
pub const BUFFER_SIZE: usize = 32;  // matters for ACK, untested, must be pwr of 2
pub const MAX_PACKET_LEN: usize = 4096;  // probably should check if this is the size of a HostTick
pub const MAGIC_NUMBER: u16 = 24835; // 8008135 % 69420
pub const TIMEOUT: u16 = TICKRATE as u16 * 30;  // 30 seconds to timeout

#[derive(Resource)]
pub struct TickNum(pub u16);  // this is the tick we're writing to, NOT playing back

#[derive(Resource)]
pub struct Socket(pub Option<UdpSocket>);

#[derive(Resource)]
pub struct IsHost(pub bool);

//TODO PlayerTick vs Player vs PlayerInfo? how does that make any sense. fix names
#[derive(Serialize, Deserialize, Debug)]
pub enum PacketContents {
    ServerFull,  // sent by host every time request is received and server is full
    Disconnect,  // sent by client in disconnected state every time HostTick is received
    HostTick {  // sent by host to all connected clients individually
        seq_num: u16,
        //ack: u16,
        //ack_bits: u32,
        player_count: u8,
        enemy_count: u8,
        players: [player::PlayerInfo; player::MAX_PLAYERS],
        enemies: [enemy::EnemyInfo; enemy::MAX_ENEMIES],
    },
    ClientTick {  // sent by client to host every FixedUpdate unless ServerFull received
        seq_num: u16,
        //ack: u16,
        //ack_bits: u32,
        pos: Vec2,
        dir: f32,
        triggers: u8  // a bit for each possible action (attack, block)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    protocol: u16,
    //packet_type: u16,  // PacketContents as u16 (discriminant of enum variant)
    contents: PacketContents
}

pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,
        (startup,
        host::startup))  // you cant conditionally run this unless you do a bunch of bullshit
            .add_systems(FixedUpdate,
                         (increment_tick.before(input::update_movement_vector),
                         client::fixed.run_if(is_client),
                         host::fixed.run_if(is_host)))
            .add_systems(Update,
                         (lerp::lerp_pos,
                         client::update.run_if(is_client),
                         host::update.run_if(is_host)))
            .add_systems(OnEnter(AppState::Game),
                         (client::connect.run_if(is_client),
                         host::connect.run_if(is_host)));
    }
}

pub fn startup(mut commands: Commands) {
    commands.insert_resource(FixedTime::new_from_secs(TICKLEN_S));
    commands.insert_resource(TickNum { 0: 0 });
    commands.insert_resource(Socket(None));
    commands.insert_resource(IsHost(true));
}

pub fn increment_tick(mut tick_num: ResMut<TickNum>) {
    tick_num.0 += 1;
}

// for conditionally running systems
fn is_host(is_host: Res<IsHost>) -> bool {
    is_host.0
}

fn is_client(is_host: Res<IsHost>) -> bool {
    !is_host.0
}