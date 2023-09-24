use bevy::prelude::*;
use crate::input::InputState;
use crate::net::{TICKRATE, TickNum};

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

// on Setup schedule
pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let input_state = InputState::default();
    commands.insert_resource(input_state);  // this is the host version!
    commands.spawn((
        // ONLY UPDATED ON FIXEDUPDATE SCHEDULE
        Player {
            id: 0,
            pos: Vec2::default(),
            hp: 100.,
            atk_frame: -1
        },
        input_state,

        // ONLY UPDATED ON UPDATE SCHEDULE
        // right here is where we add a spatial bundle and a bunch of sprite bundle children
        SpriteBundle {
            texture: asset_server.load("jordan.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        })
    );
}

// on FixedUpdate schedule
pub fn next(
    mut players: Query<(&mut Player, &mut InputState)>) {
    let speed = 150. / TICKRATE as f32;
    let atk_len = 30;
    let atk_cool = 30;
    for (mut pl, mut is) in &mut players {
        pl.pos.x += is.movement.x * speed;
        pl.pos.y += is.movement.y * speed;
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
