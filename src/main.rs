use bevy::prelude::*;

mod game;
mod net;
mod menus;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use net::NetPlugin;

use crate::game::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
    Credits,
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


