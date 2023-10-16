use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::PrimaryWindow;
use crate::{enemy, net::{self, lerp::PositionBuffer}, input};
use crate::game::map;
use crate::game::movement::Collider;
use crate::{Atlas, AppState};
use serde::{Deserialize, Serialize};

use super::enemy::Enemy;

pub const PLAYER_SPEED: f32 = 250.;
const PLAYER_DEFAULT_HP: f32 = 100.;
pub const PLAYER_SIZE: Vec2 = Vec2 { x: 32., y: 32. };
pub const MAX_PLAYERS: usize = 4;

//TODO public struct resource holding player count

#[derive(Component)]
pub struct LocalPlayer;

#[derive(Copy, Clone)]
pub struct PlayerTick {
    hp: f32,
    atk_frame: isize,  // -1 means ready, <-1 means cooldown, 0 and up means attacking
    pub input: input::InputState
}

impl Default for PlayerTick {
    fn default() -> Self {
        PlayerTick {
            hp: PLAYER_DEFAULT_HP,
            atk_frame: -1,
            input: input::InputState::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct PlayerInfo {
    pub pos: Vec2,
    pub dir: f32,
    pub hp: f32,
    pub attacking: bool
}


#[derive(Component)]
pub struct Player {
    pub id: usize,
    pub buffer: [PlayerTick; net::BUFFER_SIZE],
}

#[derive(Component)]
pub struct Weapon{}

#[derive(Component)]
struct DespawnWeaponTimer(Timer);


//TODO can't this be a trait or something?
impl Player {
    pub fn get(&self, tick: u16) -> &PlayerTick {
        let i = tick as usize % net::BUFFER_SIZE;
        &self.buffer[i]
    }

    //TODO pub fn getMut(&mut self, tick:u16) -> &mut PlayerTick

    pub fn set(&mut self, tick: u16, input: PlayerTick) {
        let i = tick as usize % net::BUFFER_SIZE;
        self.buffer[i] = input;
    }
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(FixedUpdate, fixed.before(enemy::fixed))
            .add_systems(Update,
            (spawn_weapon_on_click,
            despawn_after_timer))
            .add_systems(Update, (move_player).run_if(in_state(AppState::Game)))
            .add_systems(OnEnter(AppState::Game), spawn_player);

    }
}

pub fn spawn_player(
    mut commands: Commands,
    entity_atlas: Res<Atlas>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Player{
            id: 0,
            buffer: [PlayerTick::default(); net::BUFFER_SIZE],
        },
        PositionBuffer([Vec2::splat(0.0); net::BUFFER_SIZE]),
        SpatialBundle{
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        },
        Collider(PLAYER_SIZE),
        LocalPlayer,
    )).with_children(|parent| {
        parent.spawn(
            SpriteSheetBundle {
                texture_atlas: entity_atlas.handle.clone(),
                sprite: TextureAtlasSprite { index: entity_atlas.coord_to_index(0, 0), ..default()},
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            });
        })
    .with_children(|parent| {
        parent.spawn( SpriteBundle {
            texture: asset_server.load("healthbar.png").into(),
            transform: Transform {
                translation: Vec3::new(0., 24., 2.),
                ..Default::default()
            },
            ..Default::default()
        });
    });
}

pub fn spawn_weapon_on_click(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    query: Query<(Entity, &Transform), With<Player>>,
) {

    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }
    let window = window_query.get_single().unwrap();
    for (player_entity,player_transform) in query.iter() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let cursor_position = window.cursor_position().unwrap();
        let cursor_position_in_world = Vec2::new(cursor_position.x, window_size.y - cursor_position.y) - window_size * 0.5;
    
        let direction_vector = cursor_position_in_world.normalize();
        let weapon_direction = direction_vector.y.atan2(direction_vector.x);

        let circle_radius = 100.0;// position spawning the sword, make it variable later
        let offset_x = circle_radius * weapon_direction.cos();
        let offset_y = circle_radius * weapon_direction.sin();
        let offset = Vec2::new(offset_x, offset_y);
    
        commands.entity(player_entity).with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture: asset_server.load("sword01.png").into(),
                transform: Transform {
                    translation: Vec3::new(offset.x, offset.y, 1.0),
                    rotation: Quat::from_rotation_z(weapon_direction),
                    ..Default::default()
                },
                ..Default::default()
            }).insert(Weapon {}).insert(DespawnWeaponTimer(Timer::from_seconds(1.0, TimerMode::Once)));
        });
    }
    
}

fn despawn_after_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DespawnWeaponTimer)>,
) {
    for (entity, mut despawn_timer) in query.iter_mut() {
        despawn_timer.0.tick(time.delta());
        if despawn_timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn fixed(
        mut commands: Commands,
        tick: Res<net::TickNum>,
        mut players: Query<(Entity, &mut Player, &mut PositionBuffer, &Transform), With<LocalPlayer>>,
        enemys: Query<&PositionBuffer, (With<Enemy>, Without<Player>)>,
    ) {
    let atk_len = 30;
    let atk_cool = 30;
    // TODO change death effect to remove entity req
    for (entity, mut pl, mut player_pos_buffer, current_pos) in &mut players {
        // pull current position into positionbuffer
        player_pos_buffer.set(tick.0, Vec2::new(current_pos.translation.x, current_pos.translation.y));

        let prev = player_pos_buffer.get(tick.0.wrapping_sub(1)).clone();
        let mut next = pl.get(tick.0).clone();  // this has already been updated by input
        let possible_move = prev + next.input.movement * PLAYER_SPEED;
        let mut blocked = false;
        for enemy_pos in &enemys {
            let enemy_pos = enemy_pos.get(tick.0.wrapping_sub(1));
            //TODO add collider components which hold their own
            // size and location data within the player/enemy entities
            // and use those rather than these boxes made on the fly.
            // why? monster hitboxes could be larger than their sprite to
            // make them easier to hit, or we could need multiple colliders
            // on one entity eventually.
            if collide(
                Vec3::new(possible_move.x, possible_move.y, 0.0),
                PLAYER_SIZE,
                Vec3::new(enemy_pos.x, enemy_pos.y, 0.0),
                enemy::ENEMY_SIZE
            ).is_some(){
                //TODO this should happen in enemy.rs not here
                blocked = true;
                next.hp -= 0.5; //deal with damage when they collide with each others
            }
        }

        if next.atk_frame == -1 && next.input.attack {
            next.atk_frame = 0;
        }
        else if next.atk_frame > atk_len {
            next.atk_frame = -atk_cool;
        }
        else {
            next.atk_frame += 1;
        }

        // TODO: Make health a component?
        if next.hp <= 0. { // player can die
            commands.entity(entity).despawn(); 
        }

        pl.set(tick.0, next);
    }
}

/// Player movement function. Runs on Update schedule.
// TODO: Should this be in movement.rs? Would that make sense?
pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&Player, &mut Transform, &Collider), With<LocalPlayer>>,
    other_colliders: Query<(&Transform, &Collider), Without<LocalPlayer>>,
    map: Res<map::WorldMap>,
    time: Res<Time>,
    key_binds: Res<input::KeyBinds>
) {
    let mut mv: usize = keyboard_input.pressed(key_binds.up) as usize * 0b0001;
    mv |= keyboard_input.pressed(key_binds.down) as usize * 0b0010;
    mv |= keyboard_input.pressed(key_binds.left) as usize * 0b0100;
    mv |= keyboard_input.pressed(key_binds.right) as usize * 0b1000;
    let dir = input::MOVE_VECTORS[mv];
    let mut can_move = true;

    // should only be a single entry in this query (with localplayer)
    // TODO: I'd like to be able to use this line BUT it panics when trying to launch because the player isn't spawned in
    let player = players.single_mut();
    let player_struct = player.0;
    let mut pos = player.1.into_inner();
    let collider = player.2;

    let new_pos = Vec3 {
        x: pos.translation.x + dir.x * PLAYER_SPEED * time.delta_seconds(),
        y: pos.translation.y + dir.y * PLAYER_SPEED * time.delta_seconds(),
        z: 0.0,
    };

    // check collision against entities
    for (other_position, other_collider) in other_colliders.iter() {
        if let Some(collision) = collide(new_pos, collider.0, other_position.translation, other_collider.0) {
            // TODO: update movement vector to account for the collision?
            can_move = false;
            // if we've found out we can't move, we can break for now
            // if we end up trying to update movement in here, will have to not break here in case we collide in multiple places?
            break;
        } else {
            // can move
        }
    }

    // check collision against map tiles
    // TODO: Need to do some math to figure out where the entity is relative to the tile
    // TODO: This crashes if you try to move outside of the map
    /*
    let nearby = get_surrounding_tiles(&new_pos, &map.biome_map);
    if nearby[1][1] == Biome::Wall {
        can_move = false;
    }
    */

    if can_move {
        pos.translation.x = new_pos.x;
        pos.translation.y = new_pos.y;
    }
}