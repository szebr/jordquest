use std::io::Result;
use std::net::{SocketAddr, UdpSocket};
use bevy::prelude::*;
use crate::net::MAGIC_NUMBER;


pub enum PacketType {
    ServerFull,  // sent by host every time request is received and server is full
    Disconnect,  // sent by client in disconnected state every time HostTick is received
    ConnectionRequest,  // sent by client to request connection to a host
    ConnectionResponse,  // sent by a host to a client who has requested connection
    HostTick,  // sent by host to all connected clients individually
    ClientTick,  // sent by client to host every FixedUpdate unless ServerFull received
}

/// sent over the network to describe an enemy
pub struct EnemyTick {
    pub id: u8,
    pub pos: Vec2,
    pub hp: u8
}

/// sent by network module to disperse enemy information from the host
#[derive(Event)]
pub struct EnemyTickEvent {
    pub seq_num: u16,
    pub tick: EnemyTick
}

/// sent over the network to describe a player
pub struct PlayerTick {
    pub id: u8,
    pub pos: Vec2,
    pub hp: u8,
}

/// sent by network module to disperse player information from the host
#[derive(Event)]
pub struct PlayerTickEvent {
    pub seq_num: u16,
    pub tick: PlayerTick
}


/// the information that the client needs to produce on each tick
pub struct UserCmd {
    pub pos: Vec2,
    pub dir: f32,
    pub events: u8,
}

/// sent by network module to disperse networked inputs received on the host
#[derive(Event)]
pub struct UserCmdEvent {
    pub seq_num: u16,
    pub id: u8,
    pub tick: UserCmd
}

pub fn send_buf(buf: &[u8], local: &UdpSocket, peer: &SocketAddr) -> Result<usize> {
    if local.peer_addr().is_ok() {
        return local.send(buf);
    }
    return local.send_to(buf, peer);
}

pub trait Packet {
    fn from_buf(buf: &[u8]) -> Result<Self> where Self: Sized;
    fn to_buf(&self, bytes: &mut Vec<u8>);
}

pub struct HostTick {
    pub seq_num: u16,
    pub rmt_num: u16,
    pub ack: u32,
    pub enemies: Vec<EnemyTick>,
    pub players: Vec<PlayerTick>
}

impl Packet for HostTick {
    fn from_buf(buf: &[u8]) -> Result<Self> {
        let mut i: usize = 0;
        let seq_num = u16::from_be_bytes(buf[i..i+2].try_into().unwrap());
        i += 2;
        let rmt_num = u16::from_be_bytes(buf[i..i+2].try_into().unwrap());
        i += 2;
        let ack = u32::from_be_bytes(buf[i..i+4].try_into().unwrap());
        i += 4;
        let enemy_count = u8::from_be_bytes([buf[i]].try_into().unwrap());
        i += 1;
        let player_count = u8::from_be_bytes([buf[i]].try_into().unwrap());
        i += 1;
        let mut enemies: Vec<EnemyTick> = Vec::new();
        for _ in 0..enemy_count {
            let id = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            let pos = Vec2{
                x: f32::from_be_bytes(buf[i..i+4].try_into().unwrap()),
                y: f32::from_be_bytes(buf[i+4..i+8].try_into().unwrap())
            };
            i += 8;
            let hp = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            enemies.push(EnemyTick { id, pos, hp });
        }
        let mut players: Vec<PlayerTick> = Vec::new();
        for _ in 0..player_count {
            let id = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            let pos = Vec2{
                x: f32::from_be_bytes(buf[i..i+4].try_into().unwrap()),
                y: f32::from_be_bytes(buf[i+4..i+8].try_into().unwrap())
            };
            i += 8;
            let hp = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            players.push(PlayerTick { id, pos, hp });
        }
        return Ok(HostTick {
            seq_num,
            rmt_num,
            ack,
            enemies,
            players,
        })
    }

    fn to_buf(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());
        bytes.extend_from_slice(&(PacketType::HostTick as u8).to_be_bytes());
        bytes.extend_from_slice(&self.seq_num.to_be_bytes());
        bytes.extend_from_slice(&self.rmt_num.to_be_bytes());
        bytes.extend_from_slice(&self.ack.to_be_bytes());
        bytes.extend_from_slice(&(self.enemies.len() as u8).to_be_bytes());
        bytes.extend_from_slice(&(self.players.len() as u8).to_be_bytes());
        for enemy in &self.enemies {
            bytes.extend_from_slice(&enemy.id.to_be_bytes());
            bytes.extend_from_slice(&enemy.pos.x.to_be_bytes());
            bytes.extend_from_slice(&enemy.pos.y.to_be_bytes());
            bytes.extend_from_slice(&enemy.hp.to_be_bytes());
        }
        for player in &self.players {
            bytes.extend_from_slice(&player.id.to_be_bytes());
            bytes.extend_from_slice(&player.pos.x.to_be_bytes());
            bytes.extend_from_slice(&player.pos.y.to_be_bytes());
            bytes.extend_from_slice(&player.hp.to_be_bytes());
        }
    }
}

pub struct ClientTick {
    pub seq_num: u16,
    pub rmt_num: u16,
    pub ack: u32,
    pub tick: UserCmd
}

impl Packet for ClientTick {
    fn from_buf(buf: &[u8]) -> Result<Self> {
        let mut i: usize = 0;
        let seq_num = u16::from_be_bytes(buf[i..i+2].try_into().unwrap());
        i += 2;
        let rmt_num = u16::from_be_bytes(buf[i..i+2].try_into().unwrap());
        i += 2;
        let ack = u32::from_be_bytes(buf[i..i+4].try_into().unwrap());
        i += 4;
        let pos = Vec2{
            x: f32::from_be_bytes(buf[i..i+4].try_into().unwrap()),
            y: f32::from_be_bytes(buf[i+4..i+8].try_into().unwrap())
        };
        i += 8;
        let dir = f32::from_be_bytes(buf[i..i+4].try_into().unwrap());
        i += 4;
        let events = u8::from_be_bytes([buf[i]].try_into().unwrap());

        return Ok(ClientTick {
            seq_num,
            rmt_num,
            ack,
            tick: UserCmd {
                pos,
                dir,
                events
            }
        })
    }

    fn to_buf(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());
        bytes.extend_from_slice(&(PacketType::ClientTick as u8).to_be_bytes());
        bytes.extend_from_slice(&self.seq_num.to_be_bytes());
        bytes.extend_from_slice(&self.rmt_num.to_be_bytes());
        bytes.extend_from_slice(&self.ack.to_be_bytes());
        bytes.extend_from_slice(&self.tick.pos.x.to_be_bytes());
        bytes.extend_from_slice(&self.tick.pos.y.to_be_bytes());
        bytes.extend_from_slice(&self.tick.dir.to_be_bytes());
        bytes.extend_from_slice(&self.tick.events.to_be_bytes());
    }
}

pub struct ConnectionResponse {
    pub player_id: u8,
    pub seed: u64
}

impl Packet for ConnectionResponse {
    fn from_buf(buf: &[u8]) -> Result<Self> {
        let player_id = u8::from_be_bytes([buf[0]].try_into().unwrap());
        let seed = u64::from_be_bytes(buf[1..9].try_into().unwrap());
        return Ok(ConnectionResponse { player_id, seed });
    }

    fn to_buf(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());
        bytes.extend_from_slice(&(PacketType::ConnectionResponse as u8).to_be_bytes());
        bytes.extend_from_slice(&self.player_id.to_be_bytes());
        bytes.extend_from_slice(&self.seed.to_be_bytes());
    }
}

pub fn send_empty_packet(pt: PacketType, local: &UdpSocket, peer: &SocketAddr) -> Result<usize> {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());
    bytes.extend_from_slice(&(pt as u8).to_be_bytes());
    if local.peer_addr().is_ok() {
        return local.send(bytes.as_slice());
    }
    return local.send_to(bytes.as_slice(), peer);
}