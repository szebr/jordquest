use bevy::prelude::*;

pub mod input;
pub mod player;
pub mod enemy;
pub mod camera;
pub mod map;
pub mod noise;

pub const TITLE: &str = "JORDQUEST: SPAWNED INTO A PIXELATED WORLD WITH ENEMIES, CAMPS, AND... ANOTHER PLAYER!? CAN I EARN ENOUGH UPGRADES TO BE VICTORIOUS AND FILL MY DIAPER?";
pub const WIN_W: f32 = 1280.;
pub const WIN_H: f32 = 720.;

#[derive(Resource)]
pub struct Atlas{
    pub handle: Handle<TextureAtlas>
}

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
        .add_systems(Startup, startup)
        .add_plugins((
            player::PlayerPlugin,
            enemy::EnemyPlugin,
            map::MapPlugin,
            camera::CameraPlugin,
            input::InputPlugin,
        ));
    }
}

pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>){
    let entity_handle = asset_server.load("entity_sheet.png");
    let entity_tex_atlas = TextureAtlas::from_grid(entity_handle, Vec2::splat(32.), 2, 6, None, None);
    let entity_atlas_handle = texture_atlases.add(entity_tex_atlas);
    let entity_atlas = Atlas{handle: entity_atlas_handle};
    commands.insert_resource(entity_atlas);
}