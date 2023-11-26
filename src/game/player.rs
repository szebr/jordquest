use std::time::Duration;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::render::view::visibility;
use bevy::window::PrimaryWindow;
use crate::enemy;
use crate::game::movement::*;
use crate::{Atlas, AppState};
use crate::buffers::*;
use crate::game::camera::SpatialCameraBundle;
use crate::game::components::*;
use crate::game::enemy::LastAttacker;
use crate::game::PlayerId;
use crate::net::{is_client, is_host};
use crate::net::packets::{PlayerTickEvent, UserCmdEvent};
use crate::menus::layout::{toggle_leaderboard, update_leaderboard};


pub const PLAYER_SPEED: f32 = 250.;
pub const PLAYER_DEFAULT_HP: u8 = 100;
pub const PLAYER_DEFAULT_DEF: f32 = 1.;
pub const PLAYER_SIZE: Vec2 = Vec2 { x: 32., y: 32. };
pub const MAX_PLAYERS: usize = 4;
pub const SWORD_DAMAGE: u8 = 40;
const DEFAULT_COOLDOWN: f32 = 0.8;

#[derive(Event)]
pub struct SetIdEvent(pub u8);

#[derive(Event)]
pub struct LocalPlayerDeathEvent;

#[derive(Event)]
pub struct LocalPlayerSpawnEvent;

/// Marks the player controlled by the local computer
#[derive(Component)]
pub struct LocalPlayer;

#[derive(Component)]
pub struct PlayerWeapon {
    pub active: bool,
    pub enemies_hit: Vec<u8>,
}

#[derive(Component)]
pub struct Cooldown(pub Timer);

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct Shield;

#[derive(Component)]
pub struct PlayerShield {
    pub active: bool,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(FixedUpdate, update_buffer.before(enemy::fixed_move))
            .add_systems(Update,
                (update_health_bars,
                update_score,
                update_players,
                handle_attack,
                animate_sword.after(handle_attack),
                check_sword_collision.after(handle_attack),
                grab_powerup,
                handle_move,
                spawn_shield_on_right_click,
                despawn_shield_on_right_click_release.after(spawn_shield_on_right_click),
                handle_tick_events.run_if(is_client),
                handle_usercmd_events.run_if(is_host)).run_if(in_state(AppState::Game)))
            .add_systems(Update, handle_id_events.run_if(is_client).run_if(in_state(AppState::Connecting)))
            .add_systems(OnEnter(AppState::Game), (spawn_players, reset_cooldowns))
            .add_systems(OnEnter(AppState::GameOver), remove_players.after(toggle_leaderboard).after(update_leaderboard))
            .add_event::<SetIdEvent>()
            .add_event::<PlayerTickEvent>()
            .add_event::<UserCmdEvent>()
            .add_event::<LocalPlayerDeathEvent>()
            .add_event::<LocalPlayerSpawnEvent>();
    }
}

pub fn reset_cooldowns(mut query: Query<&mut Cooldown, With<Player>>) {
    for mut c in &mut query {
        c.0.tick(Duration::from_secs_f32(100.));
    }
}

pub fn spawn_players(
    mut commands: Commands,
    entity_atlas: Res<Atlas>,
    asset_server: Res<AssetServer>,
    res_id: Res<PlayerId>
) {
    for i in 0..MAX_PLAYERS {
        let mut pl;
        // TODO part of the bandaid syncing test stuff
        if i == 0 && res_id.0 == 1 {
            pl = commands.spawn((
                Player(i as u8),
                PosBuffer(CircularBuffer::new()),
                Stats {
                    score: 0,
                    enemies_killed: 0,
                    players_killed: 0,
                    camps_captured: 0,
                    deaths: 0,
                    kd_ratio: 0.
                },
                Health {
                    current: PLAYER_DEFAULT_HP,
                    max: PLAYER_DEFAULT_HP,
                    dead: false
                },
                SpriteSheetBundle {
                    texture_atlas: entity_atlas.handle.clone(),
                    sprite: TextureAtlasSprite { index: entity_atlas.coord_to_index(i as i32, 0), ..default()},
                    visibility: Visibility::Hidden,
                    transform: Transform::from_xyz(0., 0., 1.),
                    ..default()
                },
                Collider(PLAYER_SIZE),
                Cooldown(Timer::from_seconds(DEFAULT_COOLDOWN, TimerMode::Once)),
                StoredPowerUps {
                    power_ups: [0; NUM_POWERUPS],
                },
                PlayerShield {
                    active: false,
                },
            )).id();
        }
        else {
            pl = commands.spawn((
                Player(i as u8),
                PosBuffer(CircularBuffer::new()),
                Stats {
                    score: 0,
                    enemies_killed: 0,
                    players_killed: 0,
                    camps_captured: 0,
                    deaths: 0,
                    kd_ratio: 0.
                },
                Health {
                    current: 0,
                    max: PLAYER_DEFAULT_HP,
                    dead: true
                },
                SpriteSheetBundle {
                    texture_atlas: entity_atlas.handle.clone(),
                    sprite: TextureAtlasSprite { index: entity_atlas.coord_to_index(i as i32, 0), ..default()},
                    visibility: Visibility::Hidden,
                    transform: Transform::from_xyz(0., 0., 1.),
                    ..default()
                },
                Collider(PLAYER_SIZE),
                Cooldown(Timer::from_seconds(DEFAULT_COOLDOWN, TimerMode::Once)),
                StoredPowerUps {
                    power_ups: [0; NUM_POWERUPS],
                },
                PlayerShield {
                    active: false,
                },
            )).id();
        }

        if i as u8 == res_id.0 {
            commands.entity(pl).insert(LocalPlayer);
        }

        let health_bar = commands.spawn((
            SpriteBundle {
                texture: asset_server.load("healthbar.png"),
                transform: Transform {
                    translation: Vec3::new(0., 24., 2.),
                    ..Default::default()
                },
                ..Default::default()},
            HealthBar,
        )).id();

        commands.entity(pl).push_children(&[health_bar]);
    }
}

pub fn remove_players(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
) {
    for e in players.iter() {
        commands.entity(e).despawn_recursive();
    }
}

// Update the health bar child of player entity to reflect current hp
pub fn update_health_bars(
    mut health_bar_query: Query<&mut Transform, With<HealthBar>>,
    mut player_health_query: Query<(&mut Health, &Children, &StoredPowerUps), With<Player>>,
) {
    for (mut health, children, player_power_ups) in player_health_query.iter_mut() {
        health.max = PLAYER_DEFAULT_HP + player_power_ups.power_ups[PowerUpType::MaxHPUp as usize] * MAX_HP_UP;
        for child in children.iter() {
            let tf = health_bar_query.get_mut(*child);
            if let Ok(mut tf) = tf {
                tf.scale = Vec3::new((health.current as f32) / (health.max as f32), 1.0, 1.0);
            }
        }
    }
}

// Update the score displayed during the game
pub fn update_score(
    stats_query: Query<&Stats, With<LocalPlayer>>,
    mut score_displays: Query<&mut Text, With<ScoreDisplay>>,
) {
    for mut text in score_displays.iter_mut() {
        for stat in stats_query.iter() {
            let score = stat.score;
            text.sections[0].value = format!("Score: {}", score);
        }
    }
}

// If player hp <= 0, reset player position and subtract 1 from player score if possible
pub fn update_players(
    mut players: Query<(&mut Health, &mut Visibility, Option<&LocalPlayer>, &mut Stats, &Player)>,
    mut death_writer: EventWriter<LocalPlayerDeathEvent>,
) {
    for (mut health, mut vis, lp, mut stats, pl) in players.iter_mut() {
        if health.current <= 0 && !health.dead {
            health.dead = true;
            *vis = Visibility::Hidden;
            if lp.is_some() {
                death_writer.send(LocalPlayerDeathEvent);
            }
            if stats.deaths.checked_add(1).is_some() {
                stats.deaths += 1;
            }
            if stats.deaths != 0 {
                stats.kd_ratio = stats.players_killed as f32 / stats.deaths as f32;
            } 
            else {
                stats.kd_ratio = stats.players_killed as f32;
            }
        }
        else if health.current > 0 && health.dead {
            health.dead = false;
            *vis = Visibility::Visible;
        }
    }
}

// if the player collides with a powerup, add it to the player's powerup list and despawn the powerup entity
pub fn grab_powerup(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Health, &mut Cooldown, &mut StoredPowerUps), With<Player>>,
    powerup_query: Query<(Entity, &Transform, &PowerUp), With<PowerUp>>,
    mut powerup_displays: Query<(&mut Text, &PowerupDisplayText), With<PowerupDisplayText>>,
) {
    for (player_transform, mut player_health, mut cooldown, mut player_power_ups) in player_query.iter_mut() {
        for (powerup_entity, powerup_transform, power_up) in powerup_query.iter() {
            // check detection
            let player_pos = player_transform.translation.truncate();
            let powerup_pos = powerup_transform.translation.truncate();
            if player_pos.distance(powerup_pos) < 16. {
                // add powerup to player
                // player_power_ups.power_ups[power_up.0 as usize] += 1; // THIS DOES NOT WORK! I have no idea why
                for (mut powerup, index) in &mut powerup_displays {
                    if power_up.0 == PowerUpType::DamageDealtUp && index.0 == 0 {
                        if Some(player_power_ups.power_ups[PowerUpType::DamageDealtUp as usize].checked_add(1)) != None {
                            player_power_ups.power_ups[PowerUpType::DamageDealtUp as usize] += 1;
                        }
                        powerup.sections[0].value = format!("{:.2}x", 
                        (SWORD_DAMAGE as f32 + player_power_ups.power_ups[PowerUpType::DamageDealtUp as usize] as f32 * DAMAGE_DEALT_UP as f32) as f32
                        / SWORD_DAMAGE as f32);
                    }
                    else if power_up.0 == PowerUpType::DamageReductionUp && index.0 == 1 {
                        if Some(player_power_ups.power_ups[PowerUpType::DamageReductionUp as usize].checked_add(1)) != None {
                            player_power_ups.power_ups[PowerUpType::DamageReductionUp as usize] += 1;
                        }
                        // Defense multiplier determined by DAMAGE_REDUCTION_UP ^ n, where n is stacks of damage reduction
                        powerup.sections[0].value = format!("{:.2}x", 
                        (PLAYER_DEFAULT_DEF as f32
                        / (PLAYER_DEFAULT_DEF * DAMAGE_REDUCTION_UP.powf(player_power_ups.power_ups[PowerUpType::DamageReductionUp as usize] as f32))));
                    }
                    else if power_up.0 == PowerUpType::MaxHPUp && index.0 == 2 {
                        if Some(player_power_ups.power_ups[PowerUpType::MaxHPUp as usize].checked_add(1)) != None {
                            player_power_ups.power_ups[PowerUpType::MaxHPUp as usize] += 1;
                        }
                        if Some(player_health.current.checked_add(MAX_HP_UP)) != None {
                            player_health.current += MAX_HP_UP;
                        }
                        powerup.sections[0].value = format!("{:.2}x", 
                        (PLAYER_DEFAULT_HP as f32 + player_power_ups.power_ups[PowerUpType::MaxHPUp as usize] as f32 * MAX_HP_UP as f32) as f32
                        / PLAYER_DEFAULT_HP as f32);
                    }
                    else if power_up.0 == PowerUpType::AttackSpeedUp && index.0 == 3 {
                        if Some(player_power_ups.power_ups[PowerUpType::AttackSpeedUp as usize].checked_add(1)) != None {
                            player_power_ups.power_ups[PowerUpType::AttackSpeedUp as usize] += 1;
                        }
                        let updated_duration = cooldown.0.duration().mul_f32(1. / ATTACK_SPEED_UP);
                        cooldown.0.set_duration(updated_duration);
                        powerup.sections[0].value = format!("{:.2}x",
                        (DEFAULT_COOLDOWN
                        / (cooldown.0.duration().as_millis() as f32 / 1000.)));
                    }
                    else if power_up.0 == PowerUpType::MovementSpeedUp && index.0 == 4 {
                        if Some(player_power_ups.power_ups[PowerUpType::MovementSpeedUp as usize].checked_add(1)) != None {
                            player_power_ups.power_ups[PowerUpType::MovementSpeedUp as usize] += 1;
                        }
                        powerup.sections[0].value = format!("{:.2}x", 
                        (PLAYER_SPEED as f32 + (player_power_ups.power_ups[PowerUpType::MovementSpeedUp as usize] as f32 * MOVEMENT_SPEED_UP as f32) as f32) as f32
                        / PLAYER_SPEED as f32);
                    }
                }
                // despawn powerup
                commands.entity(powerup_entity).despawn();
            }
        }
    }
}

fn trace_attack_line(player_transform: &Transform, weapon_offset: Vec2) -> (Vec2, Vec2) {
    let start = player_transform.translation.truncate();
    let end = start + weapon_offset;
    (start, end)
}

fn line_intersects_aabb(start: Vec2, end: Vec2, box_center: Vec2, box_size: Vec2) -> bool {
    let dir = (end - start).normalize();

    let t1 = (box_center.x - box_size.x / 2.0 - start.x) / dir.x;
    let t2 = (box_center.x + box_size.x / 2.0 - start.x) / dir.x;
    let t3 = (box_center.y - box_size.y / 2.0 - start.y) / dir.y;
    let t4 = (box_center.y + box_size.y / 2.0 - start.y) / dir.y;

    let tmin = t1.min(t2).max(t3.min(t4));
    let tmax = t1.max(t2).min(t3.max(t4));

    if tmax < 0.0 || tmin > tmax {
        return false;
    }

    let t = if tmin < 0.0 { tmax } else { tmin };
    return t > 0.0 && t * t < (end - start).length_squared();
}

pub fn handle_attack(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut players: Query<(Entity, &Transform, &Player, &mut Cooldown, &StoredPowerUps, &PlayerShield), With<LocalPlayer>>,
    cameras: Query<&Transform, With<SpatialCameraBundle>>
) {
    let player = players.get_single_mut();
    if player.is_err() { return }
    let (e, tf, p, mut c, spu, shield) = player.unwrap();
    if shield.active { return }
    let camera = cameras.get_single();
    if camera.is_err() { return }
    let camera = camera.unwrap();
    c.0.tick(time.delta());
    if !(mouse_button_inputs.pressed(MouseButton::Left) && c.0.finished()) {
        return;
    }
    c.0.reset();

    let window = window_query.single();
    let cursor_position = window.cursor_position();
    if cursor_position.is_none() { return }
    let mut cursor_position = cursor_position.unwrap();
    cursor_position.x = (cursor_position.x - window.width() / 2.0) / 2.0;
    cursor_position.y = (window.height() / 2.0 - cursor_position.y) / 2.0;
    cursor_position += camera.translation.xy();
    let direction_vector = (cursor_position - tf.translation.xy()).normalize();

    commands.entity(e).with_children(|parent| {
        parent.spawn((SpriteBundle {
            texture: asset_server.load("sword01.png").into(),
            visibility: Visibility::Hidden,
            ..Default::default()
        },
        PlayerWeapon {
            active: true,
            enemies_hit: Vec::new(),
        },
        SwordAnimation {
            current: 0.0, 
            max: c.0.duration().as_secs_f32(),
            cursor_direction: direction_vector,
        },));
    });
}

// animate the sword swing when the player attacks
pub fn animate_sword(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut SwordAnimation, &mut Visibility), With<PlayerWeapon>>,
) {
    for (e, mut tf, mut animation, mut vis) in query.iter_mut() {
        // add in the direction vector to the translation
        
        let attack_radius = 50.0;
        let current_step = animation.current / animation.max;

        let cursor_angle = animation.cursor_direction.y.atan2(animation.cursor_direction.x);
        let sword_translation_angle;
        if animation.cursor_direction.x > 0.0 {
            sword_translation_angle = current_step * std::f32::consts::PI * 0.75 - std::f32::consts::PI * 0.375 - cursor_angle;
        } else {
            sword_translation_angle = current_step * std::f32::consts::PI * 0.75 - std::f32::consts::PI * 0.375 + cursor_angle;
        } 
        let sword_rotation_vector = Vec3::new(sword_translation_angle.cos(), sword_translation_angle.sin(), 0.0);
        let sword_rotation_angle = sword_rotation_vector.y.atan2(sword_rotation_vector.x);

        tf.translation.x = sword_translation_angle.cos() * attack_radius;
        if animation.cursor_direction.x > 0.0 {
            tf.rotation = Quat::from_rotation_z(-1.0 * sword_rotation_angle);
            tf.translation.y = -1.0 * sword_translation_angle.sin() * attack_radius;
        } else {
            tf.rotation = Quat::from_rotation_z(sword_rotation_angle);
            tf.translation.y = sword_translation_angle.sin() * attack_radius;
            tf.scale.y = -1.0;
        }
        if animation.current == 0.0 {
            *vis = Visibility::Visible;
        }

        animation.current += time.delta_seconds();
        if animation.current >= animation.max {
            commands.entity(e).despawn_recursive();
        }
    }
}

// TODO use aabb collision instead of distance
pub fn check_sword_collision(
    mut enemies: Query<(&Enemy, &Transform, &mut Health, &mut LastAttacker), With<Enemy>>,
    mut players: Query<(&Player, &StoredPowerUps), With<LocalPlayer>>,
    mut sword: Query<(&GlobalTransform, &mut PlayerWeapon), With<PlayerWeapon>>,
) {
    for (sword_transform, mut player_wep) in sword.iter_mut() {
        if player_wep.active == false { continue; }
        for (enemy_id, enemy_transform, mut enemy_health, mut last_attacker) in enemies.iter_mut() {
            for (player_id, spu) in players.iter_mut() {
                let sword_position = sword_transform.translation().truncate();
                let enemy_position = enemy_transform.translation.truncate();
                if sword_position.distance(enemy_position) < 30.0 && !player_wep.enemies_hit.contains(&enemy_id.0){
                    player_wep.enemies_hit.push(enemy_id.0);
                    last_attacker.0 = Some(player_id.0);
                    match enemy_health.current.checked_sub(SWORD_DAMAGE + spu.power_ups[PowerUpType::DamageDealtUp as usize] * DAMAGE_DEALT_UP) {
                        Some(v) => {
                            enemy_health.current = v;
                        }
                        None => {
                            enemy_health.current = 0;
                        }
                    }
                }
            }
        }
    }
}

pub fn spawn_shield_on_right_click(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut query: Query<(Entity, &Transform, &mut PlayerShield), With<LocalPlayer>>,
) {
    if mouse_button_inputs.just_pressed(MouseButton::Right) {
        let shield_texture_handle = asset_server.load("shield01.png"); //where to replace the shield image

        for (player_entity, _player_transform, mut shield) in query.iter_mut() {
            shield.active = true;
            commands.entity(player_entity).with_children(|parent| {
                parent.spawn(SpriteBundle {
                    texture: shield_texture_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 0.5),
                        ..Default::default()
                    },
                    ..Default::default()
                }).insert(Shield);
            });
        }
    }
}

pub fn despawn_shield_on_right_click_release(
    mut commands: Commands,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut query: Query<(&Children, &mut PlayerShield), With<LocalPlayer>>,
    shield_query: Query<Entity, With<Shield>>,
) {
    let player = query.get_single_mut();
    if player.is_err() { return; }
    let (player_children, mut player_shield) = player.unwrap();
    if !mouse_button_inputs.pressed(MouseButton::Right) {
        player_shield.active = false;
        for &child in player_children.iter() {
            if shield_query.get(child).is_ok() {
                commands.entity(child).despawn();
            }
        }
    }
}

// EVENT HANDLERS

pub fn handle_tick_events(
    mut player_reader: EventReader<PlayerTickEvent>,
    mut player_query: Query<(&Player, &mut PosBuffer, &Health)>,
) {
    //TODO if you receive info that your predicted local position is wrong, it needs to be corrected
    for ev in player_reader.iter() {
        // TODO this is slow but i have no idea how to make the borrow checker okay
        //   with the idea of an array of player PosBuffer references
        for (pl, mut pb, hp) in &mut player_query {
            if pl.0 == ev.tick.id {
                //println!("player {:?} at {:?} during tick {:?}, dead={:?}", pl.0, ev.tick.pos, ev.seq_num, hp.dead);
                pb.0.set(ev.seq_num, ev.tick.pos);
            }
        }
    }
}

pub fn handle_id_events(
    mut id_reader: EventReader<SetIdEvent>,
    mut res_id: ResMut<PlayerId>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for ev in &mut id_reader {
        res_id.0 = ev.0;
        app_state_next_state.set(AppState::Game);
    }
}

pub fn handle_usercmd_events(
    mut usercmd_reader: EventReader<UserCmdEvent>,
    mut player_query: Query<(&Player, &mut PosBuffer)>
) {
    for ev in usercmd_reader.iter() {
        // TODO this is slow but i have no idea how to make the borrow checker okay
        //   with the idea of an array of player PosBuffer references
        for (pl, mut pb) in &mut player_query {
            if pl.0 == ev.id {
                pb.0.set(ev.seq_num, ev.tick.pos);
            }
        }
    }
}

// RUN CONDITIONS

pub fn local_player_dead(health: Query<&Health, With<LocalPlayer>>) -> bool {
    let health = health.get_single();
    if health.is_err() { return false; }
    let health = health.unwrap();
    return health.dead;
}