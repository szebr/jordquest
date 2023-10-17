use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::{enemy, net};
use crate::game::movement::*;
use crate::{Atlas, AppState};
use serde::{Deserialize, Serialize};
use crate::buffers::*;
use crate::net::IsHost;


pub const PLAYER_SPEED: f32 = 250.;
const PLAYER_DEFAULT_HP: f32 = 100.;
pub const PLAYER_SIZE: Vec2 = Vec2 { x: 32., y: 32. };
pub const MAX_PLAYERS: usize = 4;

//TODO public struct resource holding player count

/// sent by network module to disperse information from the host
#[derive(Event, Debug)]
pub struct PlayerTickEvent {
    pub seq_num: u16,
    pub id: u8,
    pub tick: PlayerTick
}

/// the information that the host needs to produce on each tick
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct PlayerTick {
    pub pos: Vec2,
    pub hp: f32
}

#[derive(Event, Debug)]
pub struct UserCmdEvent {
    pub seq_num: u16,
    pub id: u8,
    pub tick: UserCmd
}

/// the information that the client needs to produce on each tick
// TODO this should just have inputs rather than a pos
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct UserCmd {
    pub pos: Vec2,
    pub dir: f32,
}

#[derive(Component)]
pub struct LocalPlayer;  // marks the player controlled by the local computer

#[derive(Component)]
pub struct Player(pub u8);  // holds id

#[derive(Component)]
pub struct Hp(pub f32);

#[derive(Component)]
pub struct Weapon;

#[derive(Component)]
struct DespawnWeaponTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(FixedUpdate, fixed.before(enemy::fixed))
            .add_systems(Update,
                (spawn_weapon_on_click,
                despawn_after_timer,
                move_player.run_if(in_state(AppState::Game)),
                packet, usercmd))
            .add_systems(OnEnter(AppState::Game), spawn_players)
            .add_event::<PlayerTickEvent>()
            .add_event::<UserCmdEvent>();
    }
}


pub fn spawn_players(
    mut commands: Commands,
    entity_atlas: Res<Atlas>,
    asset_server: Res<AssetServer>,
    is_host: Res<IsHost>
) {
    for i in 0..MAX_PLAYERS {

        let pb = PosBuffer(CircularBuffer::new_from(Vec2::new(i as f32 * 100., i as f32 * 100.)));
        let pl = commands.spawn((
            Player(i as u8),
            pb,
            Hp(PLAYER_DEFAULT_HP),
            SpriteSheetBundle {
                texture_atlas: entity_atlas.handle.clone(),
                sprite: TextureAtlasSprite { index: entity_atlas.coord_to_index(i as i32, 0), ..default()},
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            },
            Collider(PLAYER_SIZE),
        )).id();

        let health_bar = commands.spawn(SpriteBundle {
            texture: asset_server.load("healthbar.png").into(),
            transform: Transform {
                translation: Vec3::new(0., 24., 2.),
                ..Default::default()
            },
            ..Default::default()
        }).id();

        commands.entity(pl).push_children(&[health_bar]);

        if i == 0 && is_host.0 {
            commands.entity(pl).insert(LocalPlayer);
        }
        if i == 1 && !is_host.0 {
            commands.entity(pl).insert(LocalPlayer);
        }
    }
}

pub fn spawn_weapon_on_click(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    query: Query<(Entity, &Transform), With<LocalPlayer>>,
) {

    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }
    let window = window_query.get_single().unwrap();
    for (player_entity, player_transform) in query.iter() {
        let window_size = Vec2::new(window.width(), window.height());
        let cursor_position = window.cursor_position().unwrap();
        let cursor_position_in_world = Vec2::new(cursor_position.x, window_size.y - cursor_position.y) - window_size * 0.5;
    
        let direction_vector = cursor_position_in_world.normalize();
        let weapon_direction = direction_vector.y.atan2(direction_vector.x);

        let circle_radius = 50.0;// position spawning the sword, make it variable later
        let offset_x = circle_radius * weapon_direction.cos();
        let offset_y = circle_radius * weapon_direction.sin();
        let offset = Vec2::new(offset_x, offset_y);
    
        commands.entity(player_entity).with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture: asset_server.load("sword01.png").into(),
                transform: Transform {
                    translation: Vec3::new(offset.x, offset.y, 1.0),
                    rotation: Quat::from_rotation_z(weapon_direction),
                    ..Default::default()
                },
                ..Default::default()
            }).insert(Weapon {}).insert(DespawnWeaponTimer(Timer::from_seconds(1.0, TimerMode::Once)));
        });
    }
}

fn despawn_after_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DespawnWeaponTimer)>,
) {
    for (entity, mut despawn_timer) in query.iter_mut() {
        despawn_timer.0.tick(time.delta());
        if despawn_timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn fixed(
        tick: Res<net::TickNum>,
        mut players: Query<(&mut PosBuffer, &Transform), With<LocalPlayer>>,
    ) {
    for ( mut player_pos_buffer, current_pos) in &mut players {
        // pull current position into PositionBuffer
        player_pos_buffer.0.set(tick.0, Vec2::new(current_pos.translation.x, current_pos.translation.y));
    }
}

pub fn packet(
    mut player_reader: EventReader<PlayerTickEvent>,
    mut player_query: Query<(&Player, &mut PosBuffer)>
) {
    //TODO if you receive info that your predicted local position is wrong, it needs to be corrected
    for ev in player_reader.iter() {
        // TODO this is slow but i have no idea how to make the borrow checker okay
        //   with the idea of an array of player PosBuffer references
        for (pl, mut pb) in &mut player_query {
            if pl.0 == ev.id {
                pb.0.set(ev.seq_num, ev.tick.pos);
            }
        }
    }
}

pub fn usercmd(
    mut usercmd_reader: EventReader<UserCmdEvent>,
    mut player_query: Query<(&Player, &mut PosBuffer)>
) {
    // TODO in the future usercmds are just inputs, so here is where movement would be calculated
    //   ideally using the same function that clients use for local prediction
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