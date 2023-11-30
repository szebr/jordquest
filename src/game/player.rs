use std::time::Duration;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::{enemy, net};
use crate::game::movement::*;
use crate::{Atlas, AppState};
use crate::buffers::*;
use crate::game::camera::SpatialCameraBundle;
use crate::game::components::*;
use crate::game::enemy::LastAttacker;
use crate::game::PlayerId;
use crate::net::{is_client, is_host, IsHost, TICKLEN_S};
use crate::net::packets::{PlayerTickEvent, UserCmdEvent};
use crate::menus::layout::{toggle_leaderboard, update_leaderboard};

pub const PLAYER_SPEED: f32 = 250.;
pub const PLAYER_DEFAULT_HP: u8 = 100;
pub const PLAYER_DEFAULT_DEF: f32 = 1.;
pub const PLAYER_SIZE: Vec2 = Vec2 { x: 32., y: 32. };
pub const MAX_PLAYERS: usize = 4;
pub const SWORD_DAMAGE: u8 = 40;
const DEFAULT_COOLDOWN: f32 = 0.8;
pub const SPAWN_BITFLAG: u8 = 1;
pub const ATTACK_BITFLAG: u8 = 2;

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
pub struct PlayerWeapon;

#[derive(Component)]
pub struct SwordAnimation{
    pub current: f32,
    pub cursor_vector: Vec2,
    pub max: f32,
}

#[derive(Resource)]
pub struct LocalEvents {
    pub attack: bool,
    pub spawn: bool
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
        app.add_systems(FixedUpdate, update_buffer.before(net::client::fixed).before(enemy::fixed_move))
            .add_systems(Update,
                (update_health_bars,
                update_score,
                update_players,
                handle_attack_input,
                animate_sword.after(handle_attack),
                grab_powerup,
                handle_move,
                spawn_shield_on_right_click,
                despawn_shield_on_right_click_release.after(spawn_shield_on_right_click),
                handle_tick_events.run_if(is_client),
                handle_usercmd_events.run_if(is_host)).run_if(in_state(AppState::Game)))
            .add_systems(FixedUpdate, handle_attack.before(net::client::fixed))
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
        let pl;
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
        health.max = (PLAYER_DEFAULT_HP as f32 + player_power_ups.power_ups[PowerUpType::MaxHPUp as usize] as f32 * MAX_HP_UP as f32) as u8;
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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut death_writer: EventWriter<LocalPlayerDeathEvent>,
    mut spawn_writer: EventWriter<LocalPlayerSpawnEvent>,
) {
    for (mut health, mut vis, lp, mut stats, _) in players.iter_mut() {
        if health.current <= 0 && !health.dead {
            commands.spawn(AudioBundle {
                source: asset_server.load("dead-2.ogg"),
                ..default()
            });
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
            spawn_writer.send(LocalPlayerSpawnEvent);
            *vis = Visibility::Visible;
        }
    }
}

// if the player collides with a powerup, add it to the player's powerup list and despawn the powerup entity
pub fn grab_powerup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
                        (SWORD_DAMAGE as f32 + player_power_ups.power_ups[PowerUpType::DamageDealtUp as usize] as f32 * DAMAGE_DEALT_UP as f32)
                        / SWORD_DAMAGE as f32);
                    }
                    else if power_up.0 == PowerUpType::DamageReductionUp && index.0 == 1 {
                        if Some(player_power_ups.power_ups[PowerUpType::DamageReductionUp as usize].checked_add(1)) != None {
                            player_power_ups.power_ups[PowerUpType::DamageReductionUp as usize] += 1;
                        }
                        // Defense multiplier determined by DAMAGE_REDUCTION_UP ^ n, where n is stacks of damage reduction
                        powerup.sections[0].value = format!("{:.2}x", 
                        (PLAYER_DEFAULT_DEF
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
                        (PLAYER_DEFAULT_HP as f32 + player_power_ups.power_ups[PowerUpType::MaxHPUp as usize] as f32 * MAX_HP_UP as f32)
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
                        (PLAYER_SPEED + (player_power_ups.power_ups[PowerUpType::MovementSpeedUp as usize] as f32 * MOVEMENT_SPEED_UP as f32))
                        / PLAYER_SPEED);
                    }
                }
                commands.spawn(AudioBundle {
                    source: asset_server.load("powerup.ogg"),
                    ..default()
                });
                // despawn powerup
                commands.entity(powerup_entity).despawn();
            }
        }
    }
}

pub fn handle_attack_input(
    mut local_events: ResMut<LocalEvents>,
    time: Res<Time>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut players: Query<&mut Cooldown, With<LocalPlayer>>,
) {
    let player = players.get_single_mut();
    if player.is_err() { return }
    let mut c = player.unwrap();
    c.0.tick(time.delta());
    if !(mouse_button_inputs.pressed(MouseButton::Left) && c.0.finished()) {
        return;
    }
    local_events.attack = true;
}

pub fn handle_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut local_events: ResMut<LocalEvents>,
    is_host: Res<IsHost>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut players: Query<(Entity, &Player, &Transform, &mut Cooldown, &PlayerShield, &StoredPowerUps), With<LocalPlayer>>,
    cameras: Query<&Transform, With<SpatialCameraBundle>>,
    mut enemies: Query<(&Transform, &mut Health, &mut LastAttacker), With<Enemy>>,
    mut chest: Query<(&Transform, &mut Health), (With<ItemChest>, Without<Enemy>)>,
) {
    let player = players.get_single_mut();
    if player.is_err() { return }
    let (e, pl, tf, mut c, shield, spu) = player.unwrap();
    if shield.active { return }
    let camera = cameras.get_single();
    if camera.is_err() { return }
    let camera = camera.unwrap();
    if !local_events.attack { return }
    c.0.reset();

    let window = window_query.single();
    let cursor_position = window.cursor_position();
    if cursor_position.is_none() { return }
    let mut cursor_position = cursor_position.unwrap();
    cursor_position.x = (cursor_position.x - window.width() / 2.0) / 2.0;
    cursor_position.y = (window.height() / 2.0 - cursor_position.y) / 2.0;
    cursor_position += camera.translation.xy();
    let cursor_vector = (cursor_position - tf.translation.xy()).normalize();

    commands.entity(e).with_children(|parent| {
        parent.spawn((SpriteBundle {
            texture: asset_server.load("sword01.png").into(),
            visibility: Visibility::Hidden,
            ..Default::default()
        },
        PlayerWeapon,
        SwordAnimation {
            current: 0.0,
            cursor_vector,
            max: TICKLEN_S,
        },));
    });
    let player_pos = tf.translation.truncate();
    let sword_angle = cursor_vector.y.atan2(cursor_vector.x);
    for (enemy_tf, mut enemy_hp, mut last_attacker) in enemies.iter_mut() {
        let enemy_pos = enemy_tf.translation.truncate();
        if player_pos.distance(enemy_pos) > 32.0 + 50.0 { continue; } // enemy too far

        let combat_angle = (enemy_pos - player_pos).y.atan2((enemy_pos - player_pos).x);
        let angle_diff = sword_angle - combat_angle;
        if angle_diff.abs() > std::f32::consts::PI * 0.375 { continue; } // enemy not in sector

        last_attacker.0 = Some(pl.0);
        let damage = SWORD_DAMAGE + spu.power_ups[PowerUpType::DamageDealtUp as usize] * DAMAGE_DEALT_UP;
        enemy_hp.current = enemy_hp.current.saturating_sub(damage);
        commands.spawn(AudioBundle {
            source: asset_server.load("hitHurt.ogg"),
            ..default()
        });
    }
    for (chest_tf, mut chest_hp) in chest.iter_mut() {
        let chest_pos = chest_tf.translation.truncate();
        if player_pos.distance(chest_pos) > 32.0 + 50.0 { continue; } // chest too far

        let combat_angle = (chest_pos - player_pos).y.atan2((chest_pos - player_pos).x);
        let angle_diff = sword_angle - combat_angle;
        if angle_diff.abs() > std::f32::consts::PI * 0.375 { continue; } // chest not in sector

        chest_hp.current = 0;
    }
    commands.spawn(AudioBundle {
        source: asset_server.load("player-swing.ogg"),
        ..default()
    });
    if is_host.0 {
        local_events.attack = false;
    }
}

// animate the sword swing when the player attacks
pub fn animate_sword(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Visibility, &mut SwordAnimation), With<PlayerWeapon>>,
) {
    for (mut tf, mut vis, mut anim) in query.iter_mut() {
        let attack_radius = 50.0;
        let current_step = anim.current / anim.max;

        let cursor_angle = anim.cursor_vector.y.atan2(anim.cursor_vector.x);
        let sword_translation_angle;
        if anim.cursor_vector.x > 0.0 {
            sword_translation_angle = current_step * std::f32::consts::PI * 0.75 - std::f32::consts::PI * 0.375 - cursor_angle;
        } else {
            sword_translation_angle = current_step * std::f32::consts::PI * 0.75 - std::f32::consts::PI * 0.375 + cursor_angle;
        }
        let sword_rotation_vector = Vec3::new(sword_translation_angle.cos(), sword_translation_angle.sin(), 0.0);
        let sword_rotation_angle = sword_rotation_vector.y.atan2(sword_rotation_vector.x);

        tf.translation.x = sword_translation_angle.cos() * attack_radius;
        if anim.cursor_vector.x > 0.0 {
            tf.rotation = Quat::from_rotation_z(-1.0 * sword_rotation_angle);
            tf.translation.y = -1.0 * sword_translation_angle.sin() * attack_radius;
        } else {
            tf.rotation = Quat::from_rotation_z(sword_rotation_angle);
            tf.translation.y = sword_translation_angle.sin() * attack_radius;
            tf.scale.y = -1.0;
        }
        if anim.current == 0.0 {
            *vis = Visibility::Visible;
        }
        anim.current += time.delta_seconds();
        if anim.current >= anim.max {
            *vis = Visibility::Hidden;
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
            commands.spawn(AudioBundle {
                source: asset_server.load("shield.ogg"),
                ..default()
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
    mut player_query: Query<(&Player, &mut PosBuffer, &mut Health)>,
) {
    for ev in player_reader.iter() {
        for (pl, mut pb, mut hp) in &mut player_query {
            if pl.0 == ev.tick.id {
                //println!("player {:?} at {:?} during tick {:?}, dead={:?}", pl.0, ev.tick.pos, ev.seq_num, hp.dead);
                pb.0.set(ev.seq_num, ev.tick.pos);
                hp.current = ev.tick.hp;
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
    mut player_query: Query<(&Player, &mut PosBuffer, &mut Health)>,
) {
    for ev in usercmd_reader.iter() {
        for (pl, mut pb, mut hp) in &mut player_query {
            if pl.0 == ev.id {
                if (ev.tick.events & SPAWN_BITFLAG) != 0 {
                    hp.current = PLAYER_DEFAULT_HP;
                }
                pb.0.set(ev.seq_num, ev.tick.pos);
            }
        }
    }
}

// RUN CONDITIONS

pub fn local_player_dead(health: Query<&Health, With<LocalPlayer>>
) -> bool {
    let health = health.get_single();
    if health.is_err() { return false; }
    let health = health.unwrap();
    return health.dead;
}