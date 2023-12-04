pub mod host;
pub mod client;
pub mod lerp;
pub mod packets;

use std::net::UdpSocket;
use bevy::prelude::*;
use crate::AppState;
use crate::game::{enemy, movement};
use packets::{PlayerTickEvent, EnemyTickEvent, UserCmdEvent};
use crate::game::buffers::{BUFFER_LEN, DirBuffer, EventBuffer, HpBuffer, PosBuffer};
use crate::game::components::Enemy;
use crate::game::player;
use crate::game::player::LocalPlayer;


pub const TICKRATE: u8 = 10;
pub const TICKLEN_S: f32 = 1. / TICKRATE as f32;
pub const DELAY: u16 = 2;
pub const MAGIC_NUMBER: u16 = 24835; // 8008135 % 69420
//pub const TIMEOUT: u16 = TICKRATE as u16 * 10;  // 10 seconds to timeout
pub const MAX_DATAGRAM_SIZE: usize = 1024;

#[derive(Resource)]
pub struct TickNum(pub u16);  // this is the tick we're writing to, NOT playing back

#[derive(Resource)]
pub struct Socket(pub Option<UdpSocket>);

#[derive(Resource)]
pub struct IsHost(pub bool);

#[derive(Resource)]
pub struct Ack {
    pub rmt_num: u16,
    pub bitfield: u32
}

pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (startup, host::startup))  // you cant conditionally run this unless you do a bunch of bullshit
            .add_systems(FixedUpdate,
                         (increment_tick.after(client::fixed).after(host::fixed).run_if(in_state(AppState::Game)),
                         client::fixed.run_if(is_client).after(movement::update_buffer),
                         host::fixed.run_if(is_host).after(enemy::fixed_move).after(movement::update_buffer),
                         lerp::resolve_collisions.run_if(is_host).run_if(in_state(AppState::Game)).after(enemy::fixed_resolve).before(increment_tick)))
            .add_systems(Update,
                         (lerp::lerp_pos.after(host::update).after(player::handle_usercmd_events),
                         client::update.run_if(is_client),
                         host::update.run_if(is_host)))
            .add_systems(OnEnter(AppState::Game), host::connect.run_if(is_host))
            .add_systems(OnExit(AppState::Game),
                     (client::disconnect.run_if(is_client),
                      host::disconnect.run_if(is_host)))
            .add_systems(OnEnter(AppState::Connecting), client::connect.run_if(is_client))
            .add_event::<EnemyTickEvent>()
            .add_event::<PlayerTickEvent>()
            .add_event::<UserCmdEvent>();
    }
}

pub fn startup(mut commands: Commands) {
    commands.insert_resource(FixedTime::new_from_secs(TICKLEN_S));
    commands.insert_resource(TickNum { 0: 0 });
    commands.insert_resource(Socket(None));
    commands.insert_resource(IsHost(true));  // gets changed when you start the game
    commands.insert_resource(Ack { rmt_num: 0, bitfield: 0 });
}

pub fn increment_tick(
    mut tick: ResMut<TickNum>,
    mut pos_buffers: Query<&mut PosBuffer>,
    mut event_buffers: Query<&mut EventBuffer>,
    mut dir_buffers: Query<(&mut DirBuffer, Option<&LocalPlayer>)>,
    mut hp_buffers: Query<(&mut HpBuffer, Option<&LocalPlayer>)>,
) {
    tick.0 += 1;
    println!("now on tick {}", tick.0);
    for mut pb in &mut pos_buffers {
        if pb.0.get(tick.0).is_none() {
            let mut prev = None;
            for i in 1..BUFFER_LEN {
                if pb.0.get(tick.0.saturating_sub(i as u16)).is_some() {
                    prev = pb.0.get(tick.0 - i as u16).clone();
                    break;
                }
            }
            pb.0.set(tick.0, prev);
        }
        pb.0.set(tick.0 + 1, None);
    }
    for mut eb in &mut event_buffers {
        if eb.0.get(tick.0).is_none() {
            eb.0.set(tick.0, Some(0));
        }
        eb.0.set(tick.0 + 1, None);
    }
    for (mut db, lp) in &mut dir_buffers {
        if db.0.get(tick.0).is_none() {
            let prev = db.0.get(tick.0- 1).clone();
            db.0.set(tick.0, prev);
            if lp.is_some() {
                println!("setting dir {} to {:?}", tick.0, prev);
            }
        }
        db.0.set(tick.0 + 1, None);
    }
    for (mut hb, lp) in &mut hp_buffers {
        if hb.0.get(tick.0).is_none() {
            let prev = hb.0.get(tick.0 - 1).clone();
            hb.0.set(tick.0, prev);
            if lp.is_some() {
                println!("setting pos {} to {:?}", tick.0, prev);
            }
        }
        hb.0.set(tick.0 + 1, None);
    }
}

// for conditionally running systems
pub fn is_host(is_host: Res<IsHost>) -> bool {
    is_host.0
}

pub fn is_client(is_host: Res<IsHost>) -> bool {
    !is_host.0
}