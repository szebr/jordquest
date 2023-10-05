use bevy::prelude::*;

pub mod input;
pub mod player;
pub mod enemy;
pub mod camera;
pub mod map;

pub const TITLE: &str = "JORDQUEST: SPAWNED INTO A PIXELATED WORLD WITH ENEMIES, CAMPS, AND... ANOTHER PLAYER!? CAN I EARN ENOUGH UPGRADES TO BE VICTORIOUS AND FILL MY DIAPER?";
pub const WIN_W: f32 = 1280.;
pub const WIN_H: f32 = 720.;

pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: bevy::window::PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            player::PlayerPlugin,
            enemy::EnemyPlugin,
            map::MapPlugin,
            camera::CameraPlugin,
            input::InputPlugin,
        ));
    }
}
