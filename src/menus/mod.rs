use bevy::{prelude::*, input::common_conditions::input_pressed};
mod layout;
mod interactions;
mod components;

use crate::AppState;
use layout::*;
use interactions::*;


pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
        .add_systems(OnEnter(AppState::Credits), spawn_credits_page)
        .add_systems(Update, show_popup)
        .add_systems(OnExit(AppState::Credits), despawn_credits_page)
        .add_systems(OnEnter(AppState::Hosting), spawn_host_page)
        .add_systems(OnExit(AppState::Hosting), despawn_host_page)
        .add_systems(OnEnter(AppState::Joining), spawn_join_page)
        .add_systems(OnExit(AppState::Joining), despawn_join_page)
        .add_systems(Update, interact_with_host_button.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, interact_with_join_button.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, interact_with_credits_button.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, interact_with_back_button.run_if(in_state(AppState::Hosting)))
        .add_systems(Update, interact_with_back_button.run_if(in_state(AppState::Joining)))
        .add_systems(Update, update_host_input)
        .add_systems(Update, save_host_input)
        .add_systems(Update, update_join_port_input)
        .add_systems(Update, switch_input_joinpage);
}}