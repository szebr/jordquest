use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::input::InputState;
use crate::net::TICKRATE;
use crate::enemy::Enemy;

use super::enemy;

pub const PLAYER_SPEED: f32 = 250. / TICKRATE as f32;
pub const PLAYER_SIZE: Vec2 = Vec2 { x: 128., y: 128. };
pub const MAX_PLAYERS: usize = 4;

#[derive(Resource)]
pub struct PlayerID(pub usize);

#[derive(Component, Default, Copy, Clone)]
pub struct Player {
    pub id: usize,
    pub pos: Vec2,
    hp: f32,
    atk_frame: isize,  // -1 means ready, <-1 means cooldown, 0 and up means attacking
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
pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PlayerID {0:0});
    let input_state = InputState::default();
    commands.insert_resource(input_state);  // this is the host version!
    commands.spawn((
        // ONLY UPDATED ON FIXEDUPDATE SCHEDULE
        Player {
            id: 0,
            pos: Vec2::default(),
            hp: 100.,
            atk_frame: -1
        }, input_state,

        // ONLY UPDATED ON UPDATE SCHEDULE
        // right here is where we add a spatial bundle and a bunch of sprite bundle children
        SpriteBundle {
            texture: asset_server.load("jordan.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        })
    );
}

// on FixedUpdate schedule
pub fn fixed(
    mut players: Query<(&mut Player, &mut InputState)>,
    enemys: Query<&Enemy>) {
    let atk_len = 30;
    let atk_cool = 30;
    'playerloop: for (mut pl, mut is) in &mut players {
        let next = pl.pos + is.movement * PLAYER_SPEED;

        for enemy in &enemys {
            //TODO add collider components which hold their own
            // size and location data within the player/enemy entities
            // and use those rather than these boxes made on the fly.
            // why? monster hitboxes could be larger than their sprite to
            // make them easier to hit, or we could need multiple colliders
            // on one entity eventually.
            if collide(
                Vec3::new(next.x, next.y, 0.0),
                PLAYER_SIZE,
                Vec3::new(enemy.pos.x, enemy.pos.y, 0.0),
                enemy::ENEMY_SIZE
            ).is_some(){
                continue 'playerloop;
            }
        }
        pl.pos = next;

        if pl.atk_frame == -1 && is.attack {
            pl.atk_frame = 0;
        }
        else if pl.atk_frame > atk_len {
            pl.atk_frame = -atk_cool;
        }
        else {
            pl.atk_frame += 1;
        }
    }
}

// on Update schedule
pub fn update(mut query: Query<(&mut Transform, &Player)>) {
    // TODO interpolate position using time until next tick
    for (mut tf, pl) in &mut query {
        tf.translation.x = pl.pos.x;
        tf.translation.y = pl.pos.y;
        // TODO if atk_frame is attacking, make him red!
    }
}
