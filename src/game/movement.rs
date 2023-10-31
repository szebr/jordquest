use std::ops::{Div, Sub};
use bevy::prelude::*;
use crate::player::*;
use bevy::sprite::collide_aabb::collide;
use crate::map;
use crate::components::*;
use crate::game::map::Biome::Wall;
use crate::game::map::{get_pos_in_tile, get_tile_at_pos, TILESIZE};
use crate::map::{Biome, get_surrounding_tiles};

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
pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&Player, &mut Transform, &Collider), With<LocalPlayer>>,
    other_colliders: Query<(&Transform, &Collider), Without<LocalPlayer>>,
    map: Res<map::WorldMap>,
    time: Res<Time>,
    key_binds: Res<KeyBinds>
) {
    let mut mv: usize = keyboard_input.pressed(key_binds.up) as usize * 0b0001;
    mv |= keyboard_input.pressed(key_binds.down) as usize * 0b0010;
    mv |= keyboard_input.pressed(key_binds.left) as usize * 0b0100;
    mv |= keyboard_input.pressed(key_binds.right) as usize * 0b1000;
    let dir = MOVE_VECTORS[mv];
    let mut can_move = true;

    // should only be a single entry in this query (with localplayer)
    let player = players.single_mut();
    let pos = player.1.into_inner();
    let collider = player.2;

    let mut new_pos = Vec3 {
        x: pos.translation.x + dir.x * PLAYER_SPEED * time.delta_seconds(),
        y: pos.translation.y + dir.y * PLAYER_SPEED * time.delta_seconds(),
        z: 0.0,
    };

    // check collision against entities
    // TODO the player needs to move out of the way of serverside objects, or stay put if it can't
    for (other_position, other_collider) in other_colliders.iter() {
        if let Some(collision) = collide(new_pos, collider.0, other_position.translation, other_collider.0) {
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
    for i in 0..5 {
        let mut done = true;
        let half_collider = Vec2::new(collider.0.x / 2.0, collider.0.y / 2.0);
        let player_north = pos.translation + Vec3::new(0.0, half_collider.y, 0.0);
        let player_south = pos.translation - Vec3::new(0.0, half_collider.y, 0.0);
        let player_east = pos.translation + Vec3::new(half_collider.x, 0.0, 0.0);
        let player_west = pos.translation - Vec3::new(half_collider.x, 0.0, 0.0);

        let offset: f32 = 0.1;
        if get_tile_at_pos(&player_north, &map.biome_map) == Wall {
            let tilepos = get_pos_in_tile(&player_north);
            let adjustment = get_pos_in_tile(&player_north).y + offset;
            println!("In wall to north at x: {} y: {} adjusting by subtracting {} from y", tilepos.x, tilepos.y, adjustment);
            pos.translation.y -= adjustment;
            println!("new y: {:.5}", pos.translation.y);
            done = false;
        }
        if get_tile_at_pos(&player_south, &map.biome_map) == Wall {
            let tilepos = get_pos_in_tile(&player_north);
            let adjustment = TILESIZE as f32 - get_pos_in_tile(&player_south).y + offset;
            println!("In wall to south at x: {} y: {} adjusting by adding {} to y", tilepos.x, tilepos.y, adjustment);
            pos.translation.y += adjustment;
            println!("new y: {:.5}", pos.translation.y);
            done = false;
        }
        if get_tile_at_pos(&player_east, &map.biome_map) == Wall {
            let tilepos = get_pos_in_tile(&player_north);
            let adjustment = tilepos.x + offset;
            println!("In wall to east at x: {} y: {} adjusting by subtracting {} from x", tilepos.x, tilepos.y, adjustment);
            pos.translation.x -= adjustment;
            println!("new x: {:.5}", pos.translation.x);
            done = false;
        }
        if get_tile_at_pos(&player_west, &map.biome_map) == Wall {
            let tilepos = get_pos_in_tile(&player_north);
            let adjustment = TILESIZE as f32 - get_pos_in_tile(&player_west).x + offset;
            println!("In wall to west at x: {} y: {} adjusting by adding {} to x", tilepos.x, tilepos.y, adjustment);
            pos.translation.x += adjustment;
            println!("new x: {:.5}", pos.translation.x);
            done = false;
        }
        if done {
            break;
        }
    }
}
