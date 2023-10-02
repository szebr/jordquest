use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::game::player;
use crate::player::Player;
use crate::net;

pub const MAX_ENEMIES: usize = 32;
pub const ENEMY_SIZE: Vec2 = Vec2 { x: 64., y: 64. };
pub const ENEMY_SPEED: f32 = 150. / net::TICKRATE as f32;

//TODO public struct resource holding enemy count

#[derive(Copy, Clone)]
pub struct EnemyTick {
    pub pos: Vec2
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
            .add_systems(Update, update);
    }
}
// on Setup schedule
pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Enemy {
            id: 0,
            buffer: [EnemyTick{ pos: Vec2::new(300.0, 300.0) }; net::BUFFER_SIZE]
        },
        SpriteBundle {
            transform: Transform::from_xyz(0., 100., 2.),
            texture: asset_server.load("horse.png"),
            ..default()
        })
    );
}

pub fn fixed(
    tick: Res<net::TickNum>,
    mut enemies: Query<&mut Enemy, Without<Player>>,
    players: Query<&Player, Without<Enemy>>
) {
    for mut en in &mut enemies {
        let prev = en.get(tick.0.wrapping_sub(1));
        let mut next = prev.clone();
        let closest_player = &players.iter().next();
        if let Some(player) = closest_player {
            let player = player.get(tick.0.wrapping_sub(1));
            //TODO when there are multiple players, find the closest one
            /*for pl in &players {
            }*/
            let movement = player.pos - prev.pos;
            if movement.length() < 0.1 { continue }
            let movement = movement.normalize();
            let possible_movement = prev.pos + movement * ENEMY_SPEED;
            let mut blocked = false;
            //TODO same todo as on player.rs, however additionally,
            // ideally the collision would check for all players and all
            // other enemies, etc. so we might have to break it out
            // into a function or something. what's written below
            // will work for singleplayer and 99% of the time in multiplayer
            if collide(
                Vec3::new(possible_movement.x, possible_movement.y, 0.0),
                ENEMY_SIZE,
                Vec3::new(player.pos.x, player.pos.y, 0.0),
                player::PLAYER_SIZE
            ).is_some() {
                blocked = true;
            }
            if (!blocked) {
                next.pos = possible_movement;
            }
        }
        en.set(tick.0, next);
    }
}

#[allow(arithmetic_overflow)]
pub fn update(
    tick_time: Res<FixedTime>,
    tick: Res<net::TickNum>,
    mut query: Query<(&mut Transform, &Enemy)>
) {
    // TODO interpolate position using time until next tick
    for (mut tf, en) in &mut query {
        // TODO: Can we break Lerping out into a separate functionality so we don't have this cloned between enemy and player files?:w
        let next_state = en.get(tick.0.wrapping_sub(net::DELAY));
        let prev_state = en.get(tick.0.wrapping_sub(net::DELAY + 1));
        let percent: f32 = tick_time.accumulated().as_secs_f32() / tick_time.period.as_secs_f32();
        let new_state = prev_state.pos.lerp(next_state.pos, percent);
        tf.translation.x = new_state.x;
        tf.translation.y = new_state.y;
    }
}
