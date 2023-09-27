use bevy::prelude::*;

use crate::components::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_player)
        .add_systems(Update, move_player);
        
    }
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("jordan.png"),
            transform: Transform {
                translation: Vec3::new(0., 0., 900.),
                ..default()
            },
            ..default()
        })
        .insert(Player);
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
){
    if let Ok(mut transform) = player_query.get_single_mut(){
        let mut direction = Vec3::ZERO;
        
        if keyboard_input.pressed(KeyCode::A){
            if keyboard_input.pressed(KeyCode::S) {
                direction += Vec3::new(-1.0, -1.0, 0.0);
            }
            else {
                direction += Vec3::new(-1.0, 0.0, 0.0);
            }
        }
        if keyboard_input.pressed(KeyCode::D){
            if keyboard_input.pressed(KeyCode::S) {
                direction += Vec3::new(1.0, -1.0, 0.0);
            }
            else {
                direction += Vec3::new(1.0, 0.0, 0.0);
            }
        }
        if keyboard_input.pressed(KeyCode::W){
            if keyboard_input.pressed(KeyCode::D) {
                direction += Vec3::new(1.0, 1.0, 0.0);
            }
            else if keyboard_input.pressed(KeyCode::A) {
                direction += Vec3::new(-1.0, 1.0, 0.0);
            }
            else {
                direction += Vec3::new(0.0, 1.0, 0.0);
            }
        }
        if keyboard_input.pressed(KeyCode::S){
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        let change = direction * PLAYER_SPEED * time.delta_seconds();

        // Check bounds for X axis
        let new_pos_x = transform.translation + Vec3::new(change.x, 0., 0.);
        if new_pos_x.x >= -(LEVEL_W / 2.) + TILE_SIZE / 2. && new_pos_x.x <= LEVEL_W / 2. - TILE_SIZE / 2. {
            transform.translation.x = new_pos_x.x;
        }

        // Check bounds for Y axis
        let new_pos_y = transform.translation + Vec3::new(0., change.y, 0.);
        if new_pos_y.y >= -(LEVEL_H / 2.) + TILE_SIZE / 2. && new_pos_y.y <= LEVEL_H / 2. - TILE_SIZE / 2. {
            transform.translation.y = new_pos_y.y;
        }
    }
}
