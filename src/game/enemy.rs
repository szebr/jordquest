use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use serde::{Deserialize, Serialize};
use crate::{game::player, net::lerp::PositionBuffer, player::Player, net, Atlas, AppState};

pub const MAX_ENEMIES: usize = 32;
pub const ENEMY_SIZE: Vec2 = Vec2 { x: 32., y: 32. };
pub const ENEMY_SPEED: f32 = 150. / net::TICKRATE as f32;

//TODO public struct resource holding enemy count

#[derive(Copy, Clone)]
pub struct EnemyTick {
    pub health: f32,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct EnemyInfo {
    pub pos: Vec2,
    pub dir: f32,
    pub hp: f32,
    pub attacking: bool
}

#[derive(Component)]
pub struct Enemy {
    pub id: usize,
    buffer: [EnemyTick; net::BUFFER_SIZE],
}

impl Enemy {
    pub fn get(&self, tick: u16) -> &EnemyTick {
        let i = tick as usize % net::BUFFER_SIZE;
        &self.buffer[i]
    }

    pub fn set(&mut self, tick: u16, input: EnemyTick) {
        let i = tick as usize % net::BUFFER_SIZE;
        self.buffer[i] = input;
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(FixedUpdate, fixed)
            .add_systems(Update, update)
            .add_systems(OnEnter(AppState::Game), spawn_enemy);
    }
}

pub fn startup(mut commands: Commands) {
}

// on Setup schedule
pub fn spawn_enemy(mut commands: Commands, entity_atlas: Res<Atlas>) {
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0., 0., 2.),
            ..default()
        },
        Enemy {
            id: 0,
            buffer: [EnemyTick{ health: 10.0, }; net::BUFFER_SIZE]
        },
        PositionBuffer([Vec2::splat(300.0); net::BUFFER_SIZE]),
    )).with_children(|parent| {
        parent.spawn(
            SpriteSheetBundle {
                texture_atlas: entity_atlas.handle.clone(),
                sprite: TextureAtlasSprite { index: 10, ..default()},
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            });
    });
}

pub fn fixed(
    tick: Res<net::TickNum>,
    mut enemies: Query<(&Enemy, &mut PositionBuffer), Without<Player>>,
    players: Query<(&Player, &PositionBuffer), Without<Enemy>>
) {
    for (en, mut bp) in &mut enemies {
        let prev = bp.get(tick.0.wrapping_sub(1));
        let mut next = prev.clone();
        let closest_player = &players.iter().next();
        if let Some(player) = closest_player {
            let player_pos = player.1.get(tick.0.wrapping_sub(1));
            //TODO when there are multiple players, find the closest one
            /*for pl in &players {
            }*/
            let movement = *player_pos - *prev;
            if movement.length() < 0.1 { continue }
            let movement = movement.normalize();
            let possible_movement = *prev + movement * ENEMY_SPEED;
            let mut blocked = false;
            //TODO same todo as on player.rs, however additionally,
            // ideally the collision would check for all players and all
            // other enemies, etc. so we might have to break it out
            // into a function or something. what's written below
            // will work for singleplayer and 99% of the time in multiplayer
            if collide(
                Vec3::new(possible_movement.x, possible_movement.y, 0.0),
                ENEMY_SIZE,
                Vec3::new(player_pos.x, player_pos.y, 0.0),
                player::PLAYER_SIZE
            ).is_some() {
                blocked = true;
            }
            if !blocked {
                next = possible_movement;
            }
        }
        bp.set(tick.0, next);
    }
}

pub fn update(
) {
}
