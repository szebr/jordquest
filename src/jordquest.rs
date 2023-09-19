use bevy::prelude::*;

pub struct JordQuestPlugin;

impl Plugin for JordQuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (update));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn( SpriteBundle {
        texture: asset_server.load("jordan.png"),
        transform: Transform::from_xyz(100., 0., 0.),
        ..default()
    } );
}

fn update() {
    // incredible update function!!!
}
