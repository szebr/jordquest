use bevy::prelude::*;
use crate::input::*;

pub struct JordQuestPlugin;

#[derive(Resource)]
struct TickNum(u16);

enum AbilityType {
    Bite
}

//TODO tick rollover is not even REMOTELY addressed
struct Ability {
    ready_at: u16,
    duration: u16,
    ability_type: AbilityType
}

#[derive(Component)]
struct Character {
    health: f32,  // TODO what type should this be?
    abilities: Vec<Ability>
}

#[derive(Component)]
struct Enemy {
    // decision tree
    // tree state
}


#[derive(Component)]
struct Player;  // empty player tag

const TICKRATE: u8 = 30;

impl Plugin for JordQuestPlugin {
    fn build(&self, app: &mut App) {
        let key_binds = KeyBinds::new();
        let mouse_binds = MouseBinds::new();
        let input_state = InputState::new_with_bindings(key_binds, mouse_binds);
        app.insert_resource(input_state)
            .insert_resource(FixedTime::new_from_secs(1. / (TICKRATE as f32)))
            .insert_resource(TickNum { 0: 0 })
            .add_systems(Startup, setup)
            // FixedUpdate runs every simulation tick
            .add_systems(FixedUpdate, increment_tick)
            // Update runs every drawing frame
            .add_systems(Update, (
                update_key_state,
                update_mouse_state,
                player_attack.after(update_mouse_state),
                player_movement.after(update_key_state),
                update_sprite));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("jordan.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Player,
        Character { health: 100.0, abilities: vec![
            Ability {
                ready_at: 0,
                duration: 30,
                ability_type: AbilityType::Bite,
            }
        ] } ));
}

fn player_movement(
    input_state: Res<InputState>,
    time: Res<Time>,
    mut sprite_position: Query<&mut Transform, With<Player>>) {
    let speed = 150.;
    for mut transform in &mut sprite_position {
        transform.translation.x += input_state.movement.x * speed * time.delta_seconds();
        transform.translation.y += input_state.movement.y * speed * time.delta_seconds();
    }
}

//TODO needs refactoring for
//  charge attacks
//  multiple characters
//  ai
//  multiple abilities
fn player_attack(
    input_state: Res<InputState>,
    tick_num: Res<TickNum>,
    mut characters: Query<&mut Character, With<Player>>) {
    for mut character in &mut characters {
        if input_state.attack &&
            (character.abilities[0].ready_at <= tick_num.0) {
            character.abilities[0].ready_at += character.abilities[0].duration;
            // TODO write event stating that this character has used this ability.
        }
    }
}
fn increment_tick(
    //mut last_time: Local<f32>,
    //time: Res<Time>,
    //fixed_time: Res<FixedTime>,
    mut tick_num: ResMut<TickNum>) {
    tick_num.0 += 1;
}

fn update_sprite(tick_num: Res<TickNum>, mut query: Query<(&Character, &mut Sprite), With<Player>>) {
    //TODO in the future this will be where spritesheet animation happens
    for (ch, mut sp) in &mut query {
        if tick_num.0 >= ch.abilities[0].ready_at {
            sp.color = Color::rgb(1., 1., 1.);
        }
        else {
            sp.color = Color::rgb(1., 0., 0.);
        }
    }

}
