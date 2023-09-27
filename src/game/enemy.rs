use bevy::prelude::*;
use crate::player::Player;
use crate::net;

pub const MAX_ENEMIES: usize = 32;

#[derive(Component, Default, Copy, Clone)]
pub struct Enemy {
    id: usize,
    pos: Vec2
}

// on Setup schedule
pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        // FIXEDUPDATE
        Enemy {
            id: 0,
            pos: Vec2::default()
        },
        // UPDATE
        SpriteBundle {
            transform: Transform::from_xyz(0., 100., 0.),
            texture: asset_server.load("horse.png"),
            ..default()
        })
    );
}

// on FixedUpdate schedule
pub fn next(
    mut enemies: Query<&mut Enemy, Without<Player>>,
    players: Query<&Player, Without<Enemy>>
) {
    let speed = 100. / net::TICKRATE as f32;
    for mut en in &mut enemies {
        let closest_player = &players.iter().next();
        if !closest_player.is_none() {
            //TODO when there are multiple players, find the closest one
            /*for pl in &players {
            }*/
            let closest_player = closest_player.unwrap();
            let movement = closest_player.pos - en.pos;
            if movement.length() < 0.1 { continue }
            let movement = movement.normalize();
            en.pos.x += movement.x * speed;
            en.pos.y += movement.y * speed;
        }
    }
}

// on Update schedule
pub fn update(mut query: Query<(&mut Transform, &Enemy)>) {
    // TODO interpolate position using time until next tick
    for (mut tf, en) in &mut query {
        tf.translation.x = en.pos.x;
        tf.translation.y = en.pos.y;
    }
}
