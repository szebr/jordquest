use bevy::prelude::*;
use crate::player;
use crate::map;

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

                // TODO: Replace 1280 and 720 with window size instead of hardcoding
                let clamp_neg_x: f32 = ((-((map::MAPSIZE * map::TILESIZE) as isize)/2) + (1280/2) as isize) as f32;
                let clamp_pos_x: f32 = ((((map::MAPSIZE * map::TILESIZE) as isize)/2) - (1280/2) as isize) as f32;

                let clamp_neg_y: f32 = ((-((map::MAPSIZE * map::TILESIZE) as isize)/2) + (720/2) as isize) as f32;
                let clamp_pos_y: f32 = ((((map::MAPSIZE * map::TILESIZE) as isize)/2) - (720/2) as isize) as f32;

                // Clamp camera view to map borders
                // Center camera in axis if map dimensions < window size
                if map::MAPSIZE * map::TILESIZE < 1280 {
                    ctf.translation.x = 0.
                }
                else {
                    if ctf.translation.x < clamp_neg_x {
                        ctf.translation.x = clamp_neg_x
                    }
                    if ctf.translation.x > clamp_pos_x {
                        ctf.translation.x = clamp_pos_x
                    }
                }

                if map::MAPSIZE * map::TILESIZE < 720 {
                    ctf.translation.y = 0.
                }
                else {
                    if ctf.translation.y < clamp_neg_y {
                        ctf.translation.y = clamp_neg_y
                    }
                    if ctf.translation.y > clamp_pos_y {
                        ctf.translation.y = clamp_pos_y
                    }
                }
                break
            }
        }
    }
}
