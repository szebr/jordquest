use bevy::prelude::*;
use bevy::prelude::shape::CapsuleUvProfile::Fixed;
use bevy::sprite::collide_aabb::collide;
use crate::game::player;
use crate::player::Player;
use crate::net;

use super::map::TILE_SIZE;

pub const MAX_ENEMIES: usize = 32;
pub const ENEMY_SIZE: Vec2 = Vec2 { x: 64., y: 64. };
pub const ENEMY_SPEED: f32 = 150. / net::TICKRATE as f32;

#[derive(Component, Default, Copy, Clone)]
pub struct Enemy {
    id: usize,
    pub pos: Vec2
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(FixedUpdate, fixed)
            .add_systems(Update, update);
    }
}
// on Setup schedule
pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        // FIXEDUPDATE
        Enemy {
            id: 0,
            pos: Vec2::new(300.0, 300.0)
        },
        // UPDATE
        SpriteBundle {
            transform: Transform::from_xyz(0., 100., 2.),
            texture: asset_server.load("horse.png"),
            ..default()
        })
    );
}

// on FixedUpdate schedule
pub fn fixed(
    mut enemies: Query<&mut Enemy, Without<Player>>,
    players: Query<&Player, Without<Enemy>>
) {
    for mut en in &mut enemies {
        let closest_player = &players.iter().next();
        if let Some(player) = closest_player{
            //TODO when there are multiple players, find the closest one
            /*for pl in &players {
            }*/
            let movement = player.pos - en.pos;
            if movement.length() < 0.1 { continue }
            let movement = movement.normalize();
            let next = en.pos + movement * ENEMY_SPEED;
            //TODO same todo as on player.rs, however additionally,
            // ideally the collision would check for all players and all
            // other enemies, etc. so we might have to break it out
            // into a function or something. what's written below
            // will work for singleplayer and 99% of the time in multiplayer
            if collide(
                Vec3::new(next.x, next.y, 0.0),
                ENEMY_SIZE,
                Vec3::new(player.pos.x, player.pos.y, 0.0),
                player::PLAYER_SIZE
            ).is_some() {
                continue;//if collision happened, stop moving
            }
            en.pos = next;
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
