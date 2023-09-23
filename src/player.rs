use bevy::prelude::*;
use crate::jordquest::*;
use crate::input::InputState;

#[derive(Component)]
pub struct Player;  // empty player tag

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("jordan.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Player,
        Character { health: 100.0, speed: 150., abilities: vec![
            Ability {
                ready_at: 0,
                duration: 30,
                ability_type: AbilityType::Bite,
            }]
        })
    );
}

pub fn movement(
    input_state: Res<InputState>,
    time: Res<Time>,
    mut player_position: Query<(&mut Transform, &Character), With<Player>>) {
    let speed = 150.;
    for (mut transform, ch) in &mut player_position {
        transform.translation.x += input_state.movement.x * ch.speed * time.delta_seconds();
        transform.translation.y += input_state.movement.y * ch.speed * time.delta_seconds();
    }
}

//TODO needs refactoring for
//  charge attacks
//  multiple characters
//  ai
//  multiple abilities
pub fn attack(
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
pub fn update_sprite(tick_num: Res<TickNum>, mut query: Query<(&Character, &mut Sprite), With<Player>>) {
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
