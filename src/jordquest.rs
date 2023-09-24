use bevy::prelude::*;
use crate::{input, player, enemy, net, map};

pub struct JordQuestPlugin;
impl Plugin for JordQuestPlugin {
    fn build(&self, app: &mut App) {
        //TODO would be good to load all assets first and pass their handles to spawning functions
        app.add_systems(Startup, (
                setup,
                player::spawn,
                input::setup,
                net::setup,
                map::setup,
                enemy::spawn,
            ))
            .add_systems(FixedUpdate, (
                net::tick_host.run_if(is_host),
                net::tick_client.run_if(is_client),
                net::increment_tick,
                player::next,
                enemy::next,
            ))
            .add_systems(Update, (
                input::update_movement_vector,
                input::handle_mouse_button_events,
                player::update,
                enemy::update,
            ));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(player::PlayerID {0:0});
}

fn is_host() -> bool {
    true
}

fn is_client() -> bool{
    false
}