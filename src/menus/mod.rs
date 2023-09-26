use bevy::prelude::*;

use crate::AppState;

use self::{systems::{layout::*, interactions::*}, components::{MainMenu, CreditsButton}};


mod components;
mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
        .add_systems(OnEnter(AppState::Credits), spawn_credits_page)
        .add_systems(Update, interact_with_play_button.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, interact_with_credits_button.run_if(in_state(AppState::MainMenu)));
    }
}