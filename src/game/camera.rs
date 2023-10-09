use bevy::prelude::*;
use crate::game::player::LocalPlayer;
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
    players: Query<(&Transform, &player::Player), (With<LocalPlayer>, Without<Camera>)>,
    window: Query<&Window>,
    mut camera_tfs: Query<&mut Transform, (With<Camera>, Without<player::Player>)>,
) {
    // initalize resolution with expected defaults
    let mut win_x = super::WIN_W;
    let mut win_y = super::WIN_H;

    // should only have one window? not entirely sure how to unwrap this otherwise
    for w in &window {
        win_x = w.resolution.width();
        win_y = w.resolution.height();
    }

    // might have multiple cameras when we get into minimaps?
    for mut ctf in &mut camera_tfs {
        for (ptf, pl) in &players {
            ctf.translation.x = ptf.translation.x;
            ctf.translation.y = ptf.translation.y;

            let clamp_neg_x: f32 = ((-((map::MAPSIZE * map::TILESIZE) as isize)/2) + (win_x/2.) as isize) as f32;
            let clamp_pos_x: f32 = ((((map::MAPSIZE * map::TILESIZE) as isize)/2) - (win_x/2.) as isize) as f32;

            let clamp_neg_y: f32 = ((-((map::MAPSIZE * map::TILESIZE) as isize)/2) + (win_y/2.) as isize) as f32;
            let clamp_pos_y: f32 = ((((map::MAPSIZE * map::TILESIZE) as isize)/2) - (win_y/2.) as isize) as f32;

            // Clamp camera view to map borders
            // Center camera in axis if map dimensions < window size
            if map::MAPSIZE * map::TILESIZE < win_x as usize {
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

            if map::MAPSIZE * map::TILESIZE < win_y as usize {
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
