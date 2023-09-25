use bevy::prelude::*;

mod game;
mod functions;
mod main_menu;
use game::GamePlugin;
use main_menu::MainMenuPlugin;

use crate::game::*;
use crate::functions::*;

//TODO can these be in one line?
/*mod jordquest;
mod input;
mod enemy;
mod player;
mod net;
mod map;

use bevy::prelude::*;

const TITLE: &str = "JORDQUEST";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;*/

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState{
    #[default]
    MainMenu,
    Game,
    GameOver
}

fn main() {
    App::new()
        // Bevy plugin
        /* 
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: bevy::window::PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))*/
        .add_state::<AppState>()
        //Defined plugins
        .add_plugins((
            //jordquest::JordQuestPlugin,
            GamePlugin,
            MainMenuPlugin,
        ))
        //.add_systems(Update, transition_to_game_state)
        //.add_systems(Update, transition_to_main_menu_state)
        .run();
}


