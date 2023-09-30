use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::input::InputState;
use crate::enemy::Enemy;
use crate::net;
use crate::net::TickNum;

use super::enemy;

pub const PLAYER_SPEED: f32 = 250. / net::TICKRATE as f32;
const PLAYER_DEFAULT_HP: f32 = 100.;
pub const PLAYER_SIZE: Vec2 = Vec2 { x: 128., y: 128. };
pub const MAX_PLAYERS: usize = 4;

#[derive(Resource)]
pub struct PlayerID(pub usize);


#[derive(Copy, Clone)]
pub struct PlayerTick {
    pub pos: Vec2,
    hp: f32,
    atk_frame: isize,  // -1 means ready, <-1 means cooldown, 0 and up means attacking
}

impl Default for PlayerTick {
    fn default() -> Self {
        PlayerTick {
            pos: Vec2::default(),
            hp: PLAYER_DEFAULT_HP,
            atk_frame: -1
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

    pub fn set(&mut self, tick: u16, input: PlayerTick) {
        let i = tick as usize % net::BUFFER_SIZE;
        self.buffer[i] = input;
    }
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, startup)
            .add_systems(FixedUpdate, fixed)
            .add_systems(Update, update);
        
    }
}

// on Setup schedule
pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(PlayerID {0:0});
    let input_state = InputState::default();
    commands.insert_resource(input_state);  // this is the host version!
    commands.spawn((
        Player {
            id: 0,
            buffer: [PlayerTick::default(); net::BUFFER_SIZE],
        }, input_state,
        //TODO replace with spatialbundle parent with spritebundle and collider children
        SpriteBundle {
            texture: asset_server.load("jordan.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        })
    );
}

pub fn fixed(
    mut commands: Commands,
    tick: Res<TickNum>,
    mut players: Query<(Entity, &mut Player, &mut InputState)>,
    enemys: Query<&Enemy>) {
    let atk_len = 30;
    let atk_cool = 30;
    for (entity, mut pl, mut is) in &mut players {
        let prev: &PlayerTick = pl.get(tick.0.wrapping_sub(1));
        let mut next = prev.clone();
        let possible_move = next.pos + is.movement * PLAYER_SPEED;
        let mut blocked = false;
        for enemy in &enemys {
            let enemy = enemy.get(tick.0.wrapping_sub(1));
            //TODO add collider components which hold their own
            // size and location data within the player/enemy entities
            // and use those rather than these boxes made on the fly.
            // why? monster hitboxes could be larger than their sprite to
            // make them easier to hit, or we could need multiple colliders
            // on one entity eventually.
            if collide(
                Vec3::new(possible_move.x, possible_move.y, 0.0),
                PLAYER_SIZE,
                Vec3::new(enemy.pos.x, enemy.pos.y, 0.0),
                enemy::ENEMY_SIZE
            ).is_some(){
                //TODO this should happen in enemy.rs not here
                blocked = true;
                next.hp -= 0.5; //deal with damage when they collide with each others
            }
        }
        if !blocked {
            next.pos = possible_move;
        }

        if next.atk_frame == -1 && is.attack {
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
    tick: Res<TickNum>,
    mut query: Query<(&mut Transform, &Player)>,
) {
    // TODO interpolate position using time until next tick
    for (mut tf, pl) in &mut query {
        let pl = pl.get(tick.0.wrapping_sub(net::DELAY));
        tf.translation.x = pl.pos.x;
        tf.translation.y = pl.pos.y;
        // TODO if atk_frame is attacking, make him red!
    }
}
