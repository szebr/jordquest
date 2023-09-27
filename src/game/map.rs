use bevy::prelude::*;

pub const TILE_SIZE: f32 = 100.;
pub const LEVEL_W: f32 = 1920.;
pub const LEVEL_H: f32 = 1080.;

#[derive(Component)]
struct Brick;

#[derive(Component)]
struct Background;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}
fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
            texture: asset_server.load("bg.png"),
            transform: Transform::default(),
            ..default()
        })
        .insert(Background);
}