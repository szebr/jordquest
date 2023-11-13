use std::net::*;
use std::str::FromStr;
use bevy::prelude::*;
use bincode::{deserialize, serialize};
use crate::game::{enemy, player};
use crate::{menus, net};
use crate::game::buffers::PosBuffer;
use crate::game::player::UserCmdEvent;
use crate::components::*;

#[derive(Copy, Clone)]
pub struct Connection {
    pub socket: SocketAddr,
    pub player_id: u8,
    pub ack: u16,  // if the ack is older than TIMEOUT ticks ago, disconnect the player
    pub ack_bits: u32
}

#[derive(Resource)]
pub struct Connections(pub [Option<Connection>; player::MAX_PLAYERS-1]); // -1 because host not included

pub fn startup(mut commands: Commands) {
    commands.insert_resource(Connections { 0: [None; player::MAX_PLAYERS-1] });
}

pub fn connect(addresses: Res<menus::NetworkAddresses>,
    mut sock: ResMut<net::Socket>
) {
    let host_ip = Ipv4Addr::new(127,0,0,1);
    let host_port = u16::from_str(&addresses.host_port).expect("bad host port");
    let host_addr = SocketAddr::new(IpAddr::from(host_ip), host_port);
    sock.0 = Some(UdpSocket::bind(host_addr).expect("host port in use"));
    sock.0.as_mut().unwrap().set_nonblocking(true).expect("can't set nonblocking");
    println!("host successful");
}

pub fn disconnect(
    mut sock: ResMut<net::Socket>,
    mut conns: ResMut<Connections>
) {
    sock.0.take();
    for conn in conns.0.iter_mut() {
        conn.take();
    }
}

//TODO this needs to run after every other fixed has run
pub fn fixed(
    mut sock: ResMut<net::Socket>,
    tick: Res<net::TickNum>,
    conns: Res<Connections>,
    player_query: Query<(&PosBuffer, &Health, &Player)>,
    enemy_query: Query<(&PosBuffer, &Health, &Enemy)>
) {
    if sock.0.is_none() { return }
    let sock = sock.0.as_mut().unwrap();
    //TODO get all enemies and get all players, how tf am i gonna do that
    let mut players = [player::PlayerTick {
        pos: Default::default(),
        hp: 0,
    }; player::MAX_PLAYERS];
    for (pb, hp, pl) in &player_query {
        players[pl.0 as usize] = player::PlayerTick {
            pos: *pb.0.get(tick.0),
            hp: hp.current
        }
    }
    let mut enemies = [enemy::EnemyTick {
        pos: Default::default(),
        hp: 0,
    }; enemy::MAX_ENEMIES];
    for (pb, hp, en) in &enemy_query {
        enemies[en.0 as usize] = enemy::EnemyTick {
            pos: *pb.0.get(tick.0),
            hp: hp.current
        }
    }
    let packet = net::Packet {
        protocol: net::MAGIC_NUMBER,
        contents: net::PacketContents::HostTick {
            seq_num: tick.0,
            players,
            enemies,
        },
    };
    for conn in conns.0.iter() {
        if conn.is_none() { continue; }
        let socket = conn.unwrap().socket;
        sock.send_to(serialize(&packet).expect("couldn't serialize").as_slice(), socket).expect("send failed");
    }
}

/// tries to find a player id given an origin
/// returns Some(player id) if successful, otherwise None
fn get_id_of_origin(conns: &Connections, origin: &SocketAddr) -> Option<u8> {
    for conn in &conns.0 {
        if conn.is_some() {
            let conn = conn.unwrap();
            if conn.socket == *origin {
                return Some(conn.player_id);
            }
        }
    }
    return None;
}

/// tries to add a connection using the given origin
/// returns Some(player id) if successful, otherwise None
fn add_connection(mut conns: &mut Connections, origin: &SocketAddr) -> Option<u8> {
    let mut unused = [false; player::MAX_PLAYERS - 1];
    for conn in &conns.0 {
        if let Some(conn) = conn {
            unused[conn.player_id as usize] = true;
        }
    }
    let mut fresh_id: u8 = 1;
    for (i, b) in unused.into_iter().enumerate() {
        if i == fresh_id as usize && b {
            fresh_id += 1;
        }
    }
    for conn in &mut conns.0 {
        if conn.is_none() {
            let _ = conn.insert(Connection {
                socket: *origin,
                player_id: fresh_id,
                ack: 0,
                ack_bits: 0,
            });
            return Some(fresh_id);
        }
    }
    return None;
}

/// sends a standard ServerFull packet
fn send_server_full(from: &UdpSocket, to: &SocketAddr) {
    let packet = net::Packet {
        protocol: net::MAGIC_NUMBER,
        contents: net::PacketContents::ServerFull
    };
    let ser = serialize(&packet).expect("couldn't serialize");
    from.send_to(ser.as_slice(), to).expect("send failed");
}

/// sends a connection response packet informing a client of its player id
fn send_connection_response(from: &UdpSocket, to: &SocketAddr, player_id: u8) {
    let packet = net::Packet {
        protocol: net::MAGIC_NUMBER,
        contents: net::PacketContents::ConnectionResponse {
            player_id
        }
    };
    let ser = serialize(&packet).expect("couldn't serialize");
    from.send_to(ser.as_slice(), to).expect("send failed");
    println!("sent connection response");
}

pub fn update(
    mut sock: ResMut<net::Socket>,
    mut conns: ResMut<Connections>,
    tick_num: Res<net::TickNum>,
    mut usercmd_writer: EventWriter<UserCmdEvent>,
) {
    if sock.0.is_none() { return }
    let sock = sock.0.as_mut().unwrap();
    loop {
        let mut buf = [0; net::MAX_PACKET_LEN];
        // this error checking is really shitty
        let recv = sock.recv_from(&mut buf);
        if recv.is_err() { break; }
        let (size, origin) = recv.unwrap();
        if size <= 0 { break; }
        let packet = deserialize::<net::Packet>(&buf);
        if packet.is_err() { break; }
        let packet = packet.unwrap();
        if packet.protocol != net::MAGIC_NUMBER { continue; }
        match packet.contents {
            net::PacketContents::ConnectionRequest => {
                println!("ConnectionRequest received");
                let mut maybe_id = get_id_of_origin(&conns, &origin);
                if maybe_id.is_some() {
                    continue;  // this user is already in the server
                }
                maybe_id = add_connection(&mut conns, &origin);
                if maybe_id.is_none() {
                    send_server_full(sock, &origin);
                }
                send_connection_response(sock, &origin, maybe_id.unwrap());
            },
            net::PacketContents::ClientTick {
                seq_num, tick
            } => {
                let maybe_id = get_id_of_origin(&conns, &origin);
                if maybe_id.is_none() {
                    continue;  // ignore packets from non connected clients
                }
                let id = maybe_id.unwrap();
                if seq_num < tick_num.0 - net::DELAY {
                    // TODO deal with packet misses
                    println!("packet late, local is {} remote is {}", tick_num.0, seq_num);
                    continue
                }
                // send event that this player has moved to this location
                usercmd_writer.send(UserCmdEvent {
                    seq_num,
                    id,
                    tick
                });
            },
            net::PacketContents::Disconnect => {
                // for disconnect packet, check if they are still connected and remove their connection
                // TODO make player dead
                for conn in &mut conns.0 {
                    if conn.is_some() {
                        let s = conn.unwrap().socket;
                        if s == origin {
                            conn.take();
                        }
                    }
                }
            },
            p => panic!("client sent unexpected packet {:?}", p)
        }
    }
}
