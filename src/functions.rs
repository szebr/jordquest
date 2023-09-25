use bevy::prelude::*;

use crate::AppState;

/*pub fn transition_to_game_state(
    mut commands: Commands,
    keyboard_input:Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
){
    if keyboard_input.just_pressed(KeyCode::G){
        if app_state.0 != AppState::Game{
            commands.insert_resource(NextState(Some(AppState::Game)));
            println!("Entered AppState::Game");
        }
    }
}

pub fn transition_to_main_menu_state(
    mut commands: Commands,
    keyboard_input:Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
){
    if keyboard_input.just_pressed(KeyCode::N){
        if app_state.0 != AppState::MainMenu{
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
            println!("Entered AppState::MainMenu");
        }
    }
}*/

