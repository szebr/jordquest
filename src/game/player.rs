use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::input::InputState;
use crate::map;
use crate::net::TICKRATE;
use crate::enemy::Enemy;

use super::enemy;

pub const PLAYER_SPEED: f32 = 500.;
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
    let speed = 150. / TICKRATE as f32;
    let atk_len = 30;
    let atk_cool = 30;
    for (mut pl, mut is) in &mut players {
        let prev_pos = pl.pos; // store the position of the player before collision

        pl.pos.x += is.movement.x * speed;
        pl.pos.y += is.movement.y * speed;

        let mut collision = false;//no collision with the enemy yet
        for enemy in enemys.iter(){
            if collide(
                Vec3::new(pl.pos.x, pl.pos.y, 0.0),
                Vec2::new(130.0, 130.0),//player's size for now, probably need a variable later
                Vec3::new(enemy.pos.x, enemy.pos.y, 0.0),
                Vec2::new(64.0, 64.0)
            ).is_some(){
                collision = true;//player collides with the enemy, stop moving
                break;
            }
        }
        if collision{//if it collides, stop the player movement to prevent the overlapping of enemy and player
            pl.pos = prev_pos;
        }else{
            pl.pos.x = f32::max(-(map::LEVEL_W / 2.) + map::TILE_SIZE / 2., pl.pos.x);
            pl.pos.x = f32::min(map::LEVEL_W / 2. - map::TILE_SIZE / 2., pl.pos.x);
            pl.pos.y = f32::max(-(map::LEVEL_H / 2.) + map::TILE_SIZE / 2., pl.pos.y);
            pl.pos.y = f32::min(map::LEVEL_H / 2. - map::TILE_SIZE / 2., pl.pos.y);
        }

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
