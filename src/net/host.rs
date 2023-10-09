use std::net::*;
use std::str::FromStr;
use bevy::prelude::*;
use bincode::{deserialize, serialize};
use crate::game::{enemy, player};
use crate::menus;
use crate::net;

#[derive(Copy, Clone)]
pub struct Connection {
    pub socket: SocketAddr,
    pub ack: u16,  // if the ack is older than TIMEOUT ticks ago, disconnect the player
    pub ack_bits: u32
}
#[derive(Resource)]
pub struct Connections(pub [Option<Connection>; player::MAX_PLAYERS]);

pub fn startup(mut commands: Commands) {
    commands.insert_resource(Connections { 0: [None; player::MAX_PLAYERS] })
}

pub fn connect(addresses: Res<menus::NetworkAddresses>,
    mut sock: ResMut<net::Socket>
) {
    let host_ip = Ipv4Addr::new(127,0,0,1);
    //let host_port = u16::from_str(&addresses.host_port).expect("bad host port");
    let host_port: u16 = 8085;
    let host_addr = SocketAddr::new(IpAddr::from(host_ip), host_port);
    sock.0 = Some(UdpSocket::bind(host_addr).expect("host port in use"));
    sock.0.as_mut().unwrap().set_nonblocking(true).expect("can't set nonblocking");
    println!("host successful");
}

pub fn fixed(
    mut sock: ResMut<net::Socket>,
    tick_num: Res<net::TickNum>,
    conns: Res<Connections>
) {
    if sock.0.is_none() { return }
    let sock = sock.0.as_mut().unwrap();
    //TODO get all enemies and get all players, how tf am i gonna do that
    let players = [player::PlayerInfo {
        pos: Default::default(),
        dir: 0.0,
        hp: 0.0,
        attacking: false,
    }; player::MAX_PLAYERS];
    let enemies = [enemy::EnemyInfo {
        pos: Default::default(),
        dir: 0.0,
        hp: 0.0,
        attacking: false,
    }; enemy::MAX_ENEMIES];
    let packet = net::Packet {
        protocol: net::MAGIC_NUMBER,
        //packet_type: net::PacketContents::HostTick as u16,
        contents: net::PacketContents::HostTick {
            seq_num: tick_num.0,
            player_count: 1,
            enemy_count: 1,
            players,
            enemies,
        },
    };
    for conn in &conns.0 {
        if conn.is_none() { continue; }
        let socket = conn.unwrap().socket;
        sock.send_to(serialize(&packet).expect("couldn't serialize").as_slice(), socket).expect("send failed");
    }
}

pub fn update(mut sock: ResMut<net::Socket>) {
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
            // TODO this section
            net::PacketContents::ClientTick {
                seq_num, pos, dir, triggers
            } => {
                // if this is a new origin, check if server is full
                //   if full, respond with ServerFull
                //   otherwise add this origin to your connection list
                // for ClientTickPacket, check if they missed the window first, otherwise
                //   limit their movement vector to their speed
                //   and then update the buffer with their inputs.
                //   every ClientTickPacket should also update that connection's alive time
            },
            net::PacketContents::Disconnect => {
                // for disconnect packet, check if they are still connected and remove their connection
            },
            p => panic!("client sent unexpected packet {:?}", p)
        }
    }
}