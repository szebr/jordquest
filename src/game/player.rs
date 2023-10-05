use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::{enemy, net::{self, lerp::PositionBuffer}, input};

use super::enemy::Enemy;

pub const PLAYER_SPEED: f32 = 250. / net::TICKRATE as f32;
const PLAYER_DEFAULT_HP: f32 = 100.;
pub const PLAYER_SIZE: Vec2 = Vec2 { x: 128., y: 128. };
pub const MAX_PLAYERS: usize = 4;

//TODO public struct resource holding player count

#[derive(Resource)]
pub struct PlayerID(pub usize);

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

#[derive(Component)]
pub struct Player {
    pub id: usize,
    pub buffer: [PlayerTick; net::BUFFER_SIZE],
}

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
        app.add_systems(Startup, startup)
            .add_systems(FixedUpdate, fixed.before(enemy::fixed))
            .add_systems(Update, update);
        
    }
}

// on Setup schedule
pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(PlayerID {0:0});
    commands.spawn(SpatialBundle {
        transform: Transform::from_xyz(0., 0., 1.),
        ..default()
    }).with_children(|parent| {
        parent.spawn((
            Player {
                id: 0,
                buffer: [PlayerTick::default(); net::BUFFER_SIZE],
            },
            PositionBuffer([Vec2::splat(0.0); net::BUFFER_SIZE]),
            //TODO replace with spatialbundle parent with spritebundle and collider children
            SpriteBundle {
                texture: asset_server.load("jordan.png"),
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            })
        );

    });
}

pub fn fixed(
        mut commands: Commands,
        tick: Res<net::TickNum>,
        mut players: Query<(Entity, &mut Player, &mut PositionBuffer)>,
        enemys: Query<&PositionBuffer, (With<Enemy>, Without<Player>)>,
    ) {
    let atk_len = 30;
    let atk_cool = 30;
    // TODO change death effect to remove entity req
    for (entity, mut pl, mut player_pos) in &mut players {
        let prev = player_pos.get(tick.0.wrapping_sub(1)).clone();
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
        if blocked {
            player_pos.set(tick.0, prev);
        }
        else {
            player_pos.set(tick.0, possible_move);
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
        if next.hp <= 0. { // player can die
            commands.entity(entity).despawn(); 
        }

        pl.set(tick.0, next);
    }
}

pub fn update(
) {
}
