//TODO can these be in one line?
mod jordquest;
mod input;
mod enemy;
mod player;
mod net;
mod map;

use bevy::prelude::*;

const TITLE: &str = "JORDQUEST";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: bevy::window::PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }), jordquest::JordQuestPlugin))
        .run();
}


