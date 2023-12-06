use std::io::Result;
use std::net::{SocketAddr, UdpSocket};
use bevy::prelude::*;
use crate::game::components::{PowerUpType, Stats, StoredPowerUps};
use crate::game::map::MAXCHESTS;
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
    pub hp: u8,
    pub events: u8
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
    pub dir: f32,
    pub events: u8,
    pub stats: Stats,
    pub powerups: StoredPowerUps
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
    pub players: Vec<PlayerTick>,
    pub powerups: Vec<(PowerUpType, Vec2)>,
    pub chests: Vec<(u8, u8)>
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
        let powerup_count = u8::from_be_bytes([buf[i]].try_into().unwrap());
        i += 1;
        let mut enemies: Vec<EnemyTick> = Vec::new();
        for _ in 0..enemy_count {
            let id = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            let pos = Vec2 {
                x: f32::from_be_bytes(buf[i..i+4].try_into().unwrap()),
                y: f32::from_be_bytes(buf[i+4..i+8].try_into().unwrap())
            };
            i += 8;
            let hp = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            let events = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            enemies.push(EnemyTick { id, pos, hp, events });
        }
        let mut players: Vec<PlayerTick> = Vec::new();
        for _ in 0..player_count {
            let id = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            let pos = Vec2 {
                x: f32::from_be_bytes(buf[i..i+4].try_into().unwrap()),
                y: f32::from_be_bytes(buf[i+4..i+8].try_into().unwrap())
            };
            i += 8;
            let hp = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            let dir = f32::from_be_bytes(buf[i..i+4].try_into().unwrap());
            i += 4;
            let events = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            let score = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            let enemies_killed = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            let players_killed = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            let camps_captured = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            let deaths = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            let kd_ratio = f32::from_be_bytes(buf[i..i+4].try_into().unwrap());
            i += 4;
            let meat = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            let damage_dealt_up = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            let damage_reduction_up = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            let attack_speed_up = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            let move_speed_up = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            players.push(PlayerTick { id, pos, hp, dir, events, stats: Stats {
                score,
                enemies_killed,
                players_killed,
                camps_captured,
                deaths,
                kd_ratio,
            }, powerups: StoredPowerUps { power_ups: [meat, damage_dealt_up, damage_reduction_up, attack_speed_up, move_speed_up] } });
        }
        let mut powerups: Vec<(PowerUpType, Vec2)> = Vec::new();
        for _ in 0..powerup_count {
            let ptype = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            let ptype = match ptype {
                0 => PowerUpType::Meat,
                1 => PowerUpType::DamageDealtUp,
                2 => PowerUpType::DamageReductionUp,
                3 => PowerUpType::AttackSpeedUp,
                4 => PowerUpType::MovementSpeedUp,
                _ => panic!()
            };
            let x = f32::from_be_bytes(buf[i..i+4].try_into().unwrap());
            i += 4;
            let y = f32::from_be_bytes(buf[i..i+4].try_into().unwrap());
            i += 4;
            powerups.push((ptype, Vec2 {x, y}));
        }
        let mut chests: Vec<(u8, u8)> = Vec::new();
        for _ in 0..MAXCHESTS {
            let id = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            let hp = u8::from_be_bytes([buf[i]].try_into().unwrap());
            i += 1;
            chests.push((id, hp));
        }
        return Ok(HostTick {
            seq_num,
            rmt_num,
            ack,
            enemies,
            players,
            powerups,
            chests
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
        bytes.extend_from_slice(&(self.powerups.len() as u8).to_be_bytes());
        for enemy in &self.enemies {
            bytes.extend_from_slice(&enemy.id.to_be_bytes());
            bytes.extend_from_slice(&enemy.pos.x.to_be_bytes());
            bytes.extend_from_slice(&enemy.pos.y.to_be_bytes());
            bytes.extend_from_slice(&enemy.hp.to_be_bytes());
            bytes.extend_from_slice(&enemy.events.to_be_bytes());
        }
        for player in &self.players {
            bytes.extend_from_slice(&player.id.to_be_bytes());
            bytes.extend_from_slice(&player.pos.x.to_be_bytes());
            bytes.extend_from_slice(&player.pos.y.to_be_bytes());
            bytes.extend_from_slice(&player.hp.to_be_bytes());
            bytes.extend_from_slice(&player.dir.to_be_bytes());
            bytes.extend_from_slice(&player.events.to_be_bytes());
            bytes.extend_from_slice(&player.stats.score.to_be_bytes());
            bytes.extend_from_slice(&player.stats.enemies_killed.to_be_bytes());
            bytes.extend_from_slice(&player.stats.players_killed.to_be_bytes());
            bytes.extend_from_slice(&player.stats.camps_captured.to_be_bytes());
            bytes.extend_from_slice(&player.stats.deaths.to_be_bytes());
            bytes.extend_from_slice(&player.stats.kd_ratio.to_be_bytes());
            for b in &player.powerups.power_ups {
                bytes.extend_from_slice(&b.to_be_bytes());
            }
        }

        for powerup in &self.powerups {
            bytes.extend_from_slice(&(powerup.0 as u8).to_be_bytes());
            bytes.extend_from_slice(&powerup.1.x.to_be_bytes());
            bytes.extend_from_slice(&powerup.1.y.to_be_bytes());
        }
        for chest in &self.chests {
            bytes.extend_from_slice(&chest.0.to_be_bytes());
            bytes.extend_from_slice(&chest.1.to_be_bytes());
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