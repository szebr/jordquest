use std::ops::Sub;
use bevy::prelude::*;
use crate::player::*;
use bevy::sprite::collide_aabb::collide;
use crate::map;
use crate::components::*;
use crate::game::buffers::PosBuffer;
use crate::game::map::Biome::Wall;
use crate::game::map::{get_pos_in_tile, get_tile_at_pos, TILESIZE};
use crate::net::TickNum;

#[derive(Resource)]
pub struct KeyBinds {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode
}

impl KeyBinds {
    // later on, we should have a constructor that reads bindings from a file
    pub fn new() -> KeyBinds {
        KeyBinds {
            up: KeyCode::W,
            down: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D
        }
    }
}

/// this lookup table prevents square root math at runtime for movement
/// each cardinal direction is given a bit and or'd together to create the index
const DIAG: f32 = std::f32::consts::SQRT_2 / 2.;
pub const MOVE_VECTORS: [Vec2; 16] = [
    Vec2 { x:0., y:0. },  // 0000
    Vec2 { x:0., y:1. },  // 0001
    Vec2 { x:0., y:-1. }, // 0010
    Vec2 { x:0., y:0. },  // 0011
    Vec2 { x:-1., y:0. },  // 0100
    Vec2 { x:-DIAG, y:DIAG },  // 0101
    Vec2 { x:-DIAG, y:-DIAG },  // 0110
    Vec2 { x:-1., y:0. },  // 0111
    Vec2 { x:1., y:0. },  // 1000
    Vec2 { x:DIAG, y:DIAG },  // 1001
    Vec2 { x:DIAG, y:-DIAG },  // 1010
    Vec2 { x:1., y:0. },  // 1011
    Vec2 { x:0., y:0. },  // 1100
    Vec2 { x:0., y:1. },  // 1101
    Vec2 { x:0., y:-1. },  // 1110
    Vec2 { x:0., y:0. },  // 1111
];

/// Player movement function. Runs on Update schedule.
pub fn handle_move(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&Player, &mut Transform, &Health, &Collider, &StoredPowerUps, &PlayerShield), With<LocalPlayer>>,
    other_colliders: Query<(&Transform, &Collider, &Health), Without<LocalPlayer>>,
    map: Res<map::WorldMap>,
    time: Res<Time>,
    key_binds: Res<KeyBinds>
) {
    // should only be a single entry in this query (with localplayer)
    let player = players.get_single_mut();
    if player.is_err() { return; }
    let player = player.unwrap();
    let pos = player.1.into_inner();
    let hp = player.2;
    let collider = player.3;
    let spu = player.4;
    let shield = player.5;

    if hp.dead || shield.active { return }

    let mut mv: usize = keyboard_input.pressed(key_binds.up) as usize * 0b0001;
    mv |= keyboard_input.pressed(key_binds.down) as usize * 0b0010;
    mv |= keyboard_input.pressed(key_binds.left) as usize * 0b0100;
    mv |= keyboard_input.pressed(key_binds.right) as usize * 0b1000;
    let dir = MOVE_VECTORS[mv];
    let can_move = true;


    let mut new_pos = Vec3 {
        x: pos.translation.x + dir.x * (PLAYER_SPEED + spu.power_ups[PowerUpType::MovementSpeedUp as usize] as f32 * MOVEMENT_SPEED_UP as f32) * time.delta_seconds(),
        y: pos.translation.y + dir.y * (PLAYER_SPEED + spu.power_ups[PowerUpType::MovementSpeedUp as usize] as f32 * MOVEMENT_SPEED_UP as f32) * time.delta_seconds(),
        z: 0.0,
    };

    // check collision against entities
    // TODO the player needs to move out of the way of serverside objects, or stay put if it can't
    for (other_position, other_collider, other_health) in other_colliders.iter() {
        if other_health.dead { continue }
        if collide(new_pos, collider.0, other_position.translation, other_collider.0).is_some() {
            // TODO this is a temporary "push away" collision resolution.
            //can_move = false;
            new_pos = pos.translation + pos.translation.sub(other_position.translation).clamp_length_max(1.0);
            // if we've found out we can't move, we can break for now
            // if we end up trying to update movement in here, will have to not break here in case we collide in multiple places?
            break;
        } else {
            // can move
        }
    }

    if can_move {
        pos.translation.x = new_pos.x;
        pos.translation.y = new_pos.y;
    }

    // Check that we aren't colliding with a wall and move out if we are
    // repeat in case we put ourselves in a wall the first time
    for _ in 0..5 {
        let mut done = true;
        let half_collider = Vec2::new(collider.0.x / 2.0, collider.0.y / 2.0);
        let player_north = pos.translation + Vec3::new(0.0, half_collider.y, 0.0);
        let player_south = pos.translation - Vec3::new(0.0, half_collider.y, 0.0);
        let player_east = pos.translation + Vec3::new(half_collider.x, 0.0, 0.0);
        let player_west = pos.translation - Vec3::new(half_collider.x, 0.0, 0.0);

        let offset: f32 = 0.1;
        if get_tile_at_pos(&player_north, &map.biome_map) == Wall {
            let tilepos = get_pos_in_tile(&player_north);
            let adjustment = tilepos.y + offset;
            pos.translation.y -= adjustment;
            done = false;
        }
        if get_tile_at_pos(&player_south, &map.biome_map) == Wall {
            let tilepos = get_pos_in_tile(&player_north);
            let adjustment = TILESIZE as f32 - tilepos.y + offset;
            pos.translation.y += adjustment;
            done = false;
        }
        if get_tile_at_pos(&player_east, &map.biome_map) == Wall {
            let tilepos = get_pos_in_tile(&player_north);
            let adjustment = tilepos.x + offset;
            pos.translation.x -= adjustment;
            done = false;
        }
        if get_tile_at_pos(&player_west, &map.biome_map) == Wall {
            let tilepos = get_pos_in_tile(&player_north);
            let adjustment = TILESIZE as f32 - tilepos.x + offset;
            pos.translation.x += adjustment;
            done = false;
        }
        if done {
            break;
        }
    }
}

pub fn update_buffer(
    tick: Res<TickNum>,
    mut players: Query<(&mut PosBuffer, &Transform), With<LocalPlayer>>,
) {
    let player = players.get_single_mut();
    if player.is_err() { return }
    let (mut pos_buffer, current_pos) = player.unwrap();
    pos_buffer.0.set(tick.0, Vec2::new(current_pos.translation.x, current_pos.translation.y));
}
