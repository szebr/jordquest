use bevy::prelude::*;
use crate::player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, update.after(player::update));
    }
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update(
    players: Query<(&Transform, &player::Player), Without<Camera>>,
    id: Res<player::PlayerID>,
    mut camera_tfs: Query<&mut Transform, (With<Camera>, Without<player::Player>)>,
) {
    // might have multiple cameras when we get into minimaps
    for mut ctf in &mut camera_tfs {
        for (ptf, pl) in &players {
            if pl.id == id.0 {
                ctf.translation.x = ptf.translation.x;
                ctf.translation.y = ptf.translation.y;
                break
            }
        }
    }
}
