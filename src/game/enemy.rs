use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::game::movement::Collider;
use crate::{AppState, net};
use crate::Atlas;
use serde::{Deserialize, Serialize};
use crate::game::buffers::{CircularBuffer, PosBuffer};
use crate::game::components::*;
use crate::game::player;
use crate::net::is_host;

pub const MAX_ENEMIES: usize = 32;
pub const ENEMY_SIZE: Vec2 = Vec2 { x: 32., y: 32. };
pub const ENEMY_SPEED: f32 = 150. / net::TICKRATE as f32;
pub const ENEMY_MAX_HP: u8 = 100;

const SWORD_DAMAGE: u8 = 1;  //sword damage, adjust this accordingly

//TODO public struct resource holding enemy count


/// sent by network module to disperse information from the host
#[derive(Event, Debug)]
pub struct EnemyTickEvent {
    pub seq_num: u16,
    pub id: u8,
    pub tick: EnemyTick
}

/// the information that the host needs to produce on each tick
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct EnemyTick {
    pub pos: Vec2,
    pub hp: u8
}

#[derive(Component)]
pub struct EnemyWeapon;

#[derive(Component)]
struct DespawnEnemyWeaponTimer(Timer);

#[derive(Component)]
pub struct SpawnEnemyWeaponTimer(Timer);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, fixed.run_if(is_host))
            .add_systems(Update, packet)
            .add_systems(Update, spawn_weapon)
            .add_systems(Update, despawn_after_timer)
            .add_systems(Update, weapon_dealt_damage_system)
            .add_event::<EnemyTickEvent>();
    }
}

pub fn spawn_enemy(commands: &mut Commands, entity_atlas: &Res<Atlas>, id: u8, pos: Vec2, sprite: i32) {
    let pb = PosBuffer(CircularBuffer::new_from(pos));
    commands.spawn((
        Enemy(id),
        pb,
        Health {
            current: ENEMY_MAX_HP,
            max: ENEMY_MAX_HP,
        },
        SpatialBundle {
            transform: Transform::from_xyz(0., 0., 2.),
            ..default()
        },
        Collider(ENEMY_SIZE),
        SpawnEnemyWeaponTimer(Timer::from_seconds(4.0, TimerMode::Repeating)),//add a timer to spawn the enemy attack very 4 seconds
    )).with_children(|parent| {
        parent.spawn(
            SpriteSheetBundle {
                texture_atlas: entity_atlas.handle.clone(),
                sprite: TextureAtlasSprite { index: entity_atlas.coord_to_index(0, sprite), ..default()},
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            });
    });
}

pub fn spawn_weapon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut query_enemies: Query<(Entity, &Transform, &mut SpawnEnemyWeaponTimer), With<Enemy>>,
) {
    for (enemy_entity, enemy_transform, mut spawn_timer) in query_enemies.iter_mut() {
        spawn_timer.0.tick(time.delta());
        if spawn_timer.0.finished() {
            commands.entity(enemy_entity).with_children(|parent| {
                parent.spawn(SpriteBundle {
                    texture: asset_server.load("EnemyAttack01.png").into(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }).insert(EnemyWeapon).insert(DespawnEnemyWeaponTimer(Timer::from_seconds(1.0, TimerMode::Once)));
            });
        }
    }
}

fn despawn_after_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DespawnEnemyWeaponTimer)>,
) {
    for (entity, mut despawn_timer) in query.iter_mut() {
        despawn_timer.0.tick(time.delta());
        if despawn_timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn weapon_dealt_damage_system(
    mut player_query: Query<(&Transform, &Collider, &mut Health), With<Player>>,
    weapon_query: Query<&Transform, With<EnemyWeapon>>
) {
    for weapon_transform in weapon_query.iter() {
        for (player_transform, player_collider, mut player_HP) in player_query.iter_mut() {
            if let Some(_) = collide(
                weapon_transform.translation,
                weapon_transform.scale.xy(),
                player_transform.translation,
                player_collider.0,
            ) {
                match player_HP.current.checked_sub(SWORD_DAMAGE) {
                    Some(v) => {
                        player_HP.current = v;
                    }
                    None => {
                        // player would die from hit
                        player_HP.current = 0;
                    }
                }
            }
        }
    }
}

pub fn fixed(
    tick: Res<net::TickNum>,
    mut enemies: Query<&mut PosBuffer, (With<Enemy>, Without<Player>)>,
    players: Query<&PosBuffer, (With<Player>, Without<Enemy>)>
) {
    for mut epb in &mut enemies {
        let prev = epb.0.get(tick.0.wrapping_sub(1));
        let mut next = prev.clone();
        let mut closest_player = players.iter().next().unwrap();
        let mut best_distance = f32::MAX;
        for ppb in &players {
            let dist = ppb.0.get(tick.0).distance(*prev);
            if dist < best_distance {
                best_distance = dist;
                closest_player = ppb;
            }
        }
        let player_pos = closest_player.0.get(tick.0.wrapping_sub(1));
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
        epb.0.set(tick.0, next);
    }
}

pub fn packet(
    mut enemy_reader: EventReader<EnemyTickEvent>,
    mut enemy_query: Query<(&Enemy, &mut PosBuffer)>
) {
    //TODO if you receive info that your predicted local position is wrong, it needs to be corrected
    for ev in enemy_reader.iter() {
        // TODO this is slow but i have no idea how to make the borrow checker okay
        //   with the idea of an array of player PosBuffer references
        for (pl, mut pb) in &mut enemy_query {
            if pl.0 == ev.id {
                pb.0.set(ev.seq_num, ev.tick.pos);
            }
        }
    }
}
