use bevy::prelude::*;
use bevy::prelude::shape::CapsuleUvProfile::Fixed;
use bevy::sprite::collide_aabb::collide;
use crate::player::Player;
use crate::net;

use super::map::TILE_SIZE;

pub const MAX_ENEMIES: usize = 32;

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
    let speed = 100. / net::TICKRATE as f32;
    for mut en in &mut enemies {
        let closest_player = &players.iter().next();
        if let Some(player) = closest_player{
            let player_size = Vec2::new(130.0, 130.0);//size of the player for collision detection
            let enemy_size = Vec2::new(64.0, 64.0);//size of the enemy for collision detection

            //collision detection
            if collide(
                Vec3::new(en.pos.x, en.pos.y, 0.0),
                enemy_size,
                Vec3::new(player.pos.x, player.pos.y, 0.0),
                player_size
            ).is_some() {
                continue;//if collision happened, stop moving
            }
            //TODO when there are multiple players, find the closest one
            /*for pl in &players {
            }*/
            //let closest_player = closest_player.unwrap();
            let movement = player.pos - en.pos;
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
