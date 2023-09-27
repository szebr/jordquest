use bevy::prelude::*;

mod game;
mod net;
mod functions;
mod main_menu;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use net::NetPlugin;

use crate::game::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState{
    #[default]
    MainMenu,
    Game,
    GameOver
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins((
            GamePlugin,
            MainMenuPlugin,
            NetPlugin,
        ))
        .run();
}


