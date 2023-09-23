use bevy::prelude::*;
use crate::jordquest::*;
use crate::player::Player;

#[derive(Component)]
pub struct Enemy;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 100., 0.),
            texture: asset_server.load("horse.png"),
            ..default()
        },
        Enemy,
        Character {
            health: 100.0,
            speed: 100.,
            abilities: vec![
                Ability {
                    ready_at: 0,
                    duration: 30,
                    ability_type: AbilityType::Bite,
                }]
        })
    );
}

pub fn movement(
    time: Res<Time>,
    mut enemy_position: Query<(&mut Transform, &Character), (With<Enemy>, Without<Player>)>,
    player_position: Query<&Transform, (With<Player>, Without<Enemy>)>) {
    for (mut transform, ch) in &mut enemy_position {
        let closest_player = &player_position.iter().next();
        if !closest_player.is_none() {
            //TODO when there are multiple players, find the closest one
            /*for player in &player_position {
            }*/
            let closest_player = closest_player.unwrap();
            let movement = closest_player.translation - transform.translation;
            let movement = movement.normalize();
            transform.translation.x += movement.x * ch.speed * time.delta_seconds();
            transform.translation.y += movement.y * ch.speed * time.delta_seconds();
        }
    }

}
