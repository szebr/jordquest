use bevy::audio::PlaybackMode;
use bevy::prelude::*;
pub(crate) mod layout;
mod interactions;
pub(crate) mod components;

use crate::AppState;
use layout::*;
use interactions::*;
use crate::menus::components::*;
use crate::game::player::{spawn_players, remove_players};

#[derive(Component)]
struct InGameAmbientAudio;


#[derive(Resource)]
pub struct NetworkAddresses {
    pub host_port: String, //host port
    pub client_port: String,
    pub ip: String,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
        .add_systems(Update, show_popup)
        .add_systems(OnEnter(AppState::Credits), spawn_credits_page)
        .add_systems(OnExit(AppState::Credits), despawn_credits_page)
        .add_systems(OnEnter(AppState::Connecting), spawn_connecting_page)
        .add_systems(OnExit(AppState::Connecting), despawn_connecting_page)
        .add_systems(OnEnter(AppState::Hosting), spawn_host_page)
        .add_systems(OnExit(AppState::Hosting), despawn_host_page)
        .add_systems(OnEnter(AppState::Joining), spawn_join_page)
        .add_systems(OnExit(AppState::Joining), despawn_join_page)
        .add_systems(OnEnter(AppState::Controls), spawn_controls_page)
        .add_systems(OnExit(AppState::Controls), despawn_controls_page)
        .add_systems(OnEnter(AppState::Game), spawn_in_game_ui)
        .add_systems(OnExit(AppState::Game), despawn_in_game_ui)
        .add_systems(OnEnter(AppState::Game), spawn_leaderboard_ui.after(spawn_players))
        .add_systems(OnEnter(AppState::GameOver), update_leaderboard.before(remove_players))
        .add_systems(OnEnter(AppState::GameOver), toggle_leaderboard.before(remove_players))
        .add_systems(OnExit(AppState::GameOver), despawn_leaderboard_ui)
        .add_systems(OnEnter(AppState::Quitting), exit_system)
        .add_systems(OnEnter(AppState::Game), play_ambient)
        .add_systems(OnExit(AppState::Game), despawn_ambient_audio)
        .add_systems(Update, interact_with_button::<HostButtonType>.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, interact_with_button::<JoinButtonType>.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, interact_with_button::<ControlsButtonType>.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, interact_with_button::<CreditsButtonType>.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, toggle_leaderboard.run_if(in_state(AppState::Game)))
        .add_systems(Update, update_leaderboard.run_if(in_state(AppState::Game)))
        .add_systems(Update, interact_with_button::<CreditsButtonType>.run_if(in_state(AppState::GameOver)))
        .add_systems(Update, interact_with_button::<BackButtonType>)
        .add_systems(Update, interact_with_button::<QuitButtonType>.run_if(in_state(AppState::Credits)))
        .add_systems(Update, update_host_input)
        .add_systems(Update, update_num_camps_input)
        .add_systems(Update, update_num_chests_input)
        .add_systems(Update, update_enemies_per_camp_input)
        .add_systems(Update, update_map_seed_input)
        .add_systems(Update, update_eid_percentage_input)
        .add_systems(Update, update_time_remaining_system.run_if(in_state(AppState::Game)))
        .add_systems(Update, save_host_input)
        .add_systems(Update, update_join_port_input)
        .add_systems(Update, update_join_host_port_input)
        .add_systems(Update, join_host_port_but)
        .add_systems(Update, num_camps_but)
        .add_systems(Update, num_chests_but)
        .add_systems(Update, enemy_per_camp_but)
        .add_systems(Update, map_seed_but)
        .add_systems(Update, eid_percentage_but)
        .add_systems(Update, update_join_ip_input)
        .add_systems(Update, join_port_but)
        .add_systems(Update, host_port_but)
        .add_systems(Update, join_ip_but)
        .add_systems(Update, save_join_input)
        .add_systems(Update, init_host_port_input_system)
        .add_systems(Update, init_join_host_port_input_system)
        .add_systems(Update, init_join_port_input_system)
        .add_systems(Update, init_join_ip_input_system)
        .add_systems(Update, init_num_camps_input_system)
        .add_systems(Update, init_num_chests_input_system)
        .add_systems(Update, init_enemies_per_camp_input_system)
        .add_systems(Update, init_map_seed_input_system)
        .add_systems(Update, init_eid_percentage_input_system)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, animate.run_if(in_state(AppState::MainMenu)))
        .add_systems(Startup, startup);
}}

pub fn startup(mut commands: Commands) {
    commands.insert_resource( NetworkAddresses {
        host_port: String::new(), client_port: String::new(), ip: String::new(),
    });
}

fn play_ambient(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn(AudioBundle {
        source: asset_server.load("InGameAmbient.ogg"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            ..Default::default()
        },
    })
    .insert(InGameAmbientAudio);
}

fn despawn_ambient_audio(
    mut commands: Commands,
    query: Query<Entity, With<InGameAmbientAudio>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}