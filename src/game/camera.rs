use bevy::prelude::*;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::render::camera::Viewport;
use crate::game::player::LocalPlayer;
use crate::player;
use crate::map;
use crate::AppState;

pub const GAME_PROJ_SCALE: f32 = 0.5;
pub const MINIMAP_PROJ_SCALE: f32 = 8.;

const MINIMAP_POSITION: UVec2 = UVec2::new(992, 32);
const MINIMAP_DIMENSIONS: UVec2 = UVec2::new(256, 256);
const MINIMAP_BORDER_TRANSLATION: Vec3 = Vec3::new(240., 100., 5.);

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct MinimapCamera;

#[derive(Component)]
pub struct MinimapBorder;

#[derive(Component)]
pub struct SpatialCameraBundle;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, update.after(player::move_player))
            .add_systems(OnEnter(AppState::Game), spawn_minimap_cam.after(startup));
    }
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        SpatialBundle {
            ..Default::default()
        },
        SpatialCameraBundle,
    )).with_children(|parent|{
            parent.spawn((
                Camera2dBundle {
                    camera: Camera {
                        viewport: Some(Viewport {
                            physical_position: UVec2::new(0, 0),
                            physical_size: UVec2::new(super::WIN_W as u32, super::WIN_H as u32),
                            ..default()
                        }),
                        ..default()
                    },
                    projection: OrthographicProjection{near: -1000., scale: GAME_PROJ_SCALE, ..default()},
                    ..default()
                },
                GameCamera,
            ));
            parent.spawn((
                SpriteBundle {
                    texture: asset_server.load("minimap_border.png"),
                    transform: Transform {
                        translation: MINIMAP_BORDER_TRANSLATION,
                        scale: Vec3::new(0.5, 0.5, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                MinimapBorder,
            ));
        },
            
    );
}

fn spawn_minimap_cam(mut commands: Commands) {
    commands.spawn((
        SpatialBundle {
            ..Default::default()
        },
        SpatialCameraBundle,
    )).with_children(|parent|{
        parent.spawn((
            Camera2dBundle {
                camera: Camera {
                    order: 1,
                    viewport: Some(Viewport {
                        physical_position: MINIMAP_POSITION,
                        physical_size: MINIMAP_DIMENSIONS,
                        ..default()
                    }),
                    ..default()
                },
                camera_2d: Camera2d { 
                    clear_color: ClearColorConfig::None
                },
                projection: OrthographicProjection{near: -1000., scale: MINIMAP_PROJ_SCALE, ..default()},
                ..default()
            },
            MinimapCamera,
        ));
    });
}

fn update(
    players: Query<(&Transform, &player::Player), (With<LocalPlayer>, Without<Camera>)>,
    window: Query<&Window>,
    mut orthocam: Query<(&Parent, &OrthographicProjection, &Camera), (With<Camera>, Without<player::Player>)>,
    mut spatial_bundle_tf: Query<&mut Transform, (With<SpatialCameraBundle>, Without<player::Player>)>
) {
    // initalize resolution with expected defaults
    let mut win_x = super::WIN_W;
    let mut win_y = super::WIN_H;

    // should only have one window? not entirely sure how to unwrap this otherwise
    for w in &window {
        win_x = w.resolution.width();
        win_y = w.resolution.height();
    }

    for (parent, ortho_proj, cam) in &mut orthocam {
        for (ptf, pl) in &players {
            let sbtf = spatial_bundle_tf.get_mut(parent.get());

            match sbtf {
                Ok(mut sb) => {
                    // SpatialCameraBundle is valid
                    sb.translation.x = ptf.translation.x;
                    sb.translation.y = ptf.translation.y;

                    let mut cam_size_x: u32 = 0;
                    let mut cam_size_y: u32 = 0;

                    match cam.physical_viewport_size() {
                        Some(uvec2) => {
                            cam_size_x = uvec2.x;
                            cam_size_y = uvec2.y;
                        }
                        None => {

                        }
                    }

                    let clamp_neg_x: f32 = ((-((map::MAPSIZE * map::TILESIZE) as isize)/2) + (((cam_size_x as f32 * ortho_proj.scale) / 2.) as isize)) as f32;
                    let clamp_pos_x: f32 = ((((map::MAPSIZE * map::TILESIZE) as isize)/2) - (((cam_size_x as f32 * ortho_proj.scale) / 2.) as isize)) as f32;

                    let clamp_neg_y: f32 = ((-((map::MAPSIZE * map::TILESIZE) as isize)/2) + (((cam_size_y as f32 * ortho_proj.scale) / 2.) as isize)) as f32;
                    let clamp_pos_y: f32 = ((((map::MAPSIZE * map::TILESIZE) as isize)/2) - (((cam_size_y as f32 * ortho_proj.scale) / 2.) as isize)) as f32;

                    // Clamp camera view to map borders
                    // Center camera in axis if map dimensions < window size
                    if map::MAPSIZE * map::TILESIZE < win_x as usize {
                        sb.translation.x = 0.
                    }
                    else {
                        if sb.translation.x < clamp_neg_x {
                            sb.translation.x = clamp_neg_x
                        }
                        if sb.translation.x > clamp_pos_x {
                            sb.translation.x = clamp_pos_x
                        }
                    }

                    if map::MAPSIZE * map::TILESIZE < win_y as usize {
                        sb.translation.y = 0.
                    }
                    else {
                        if sb.translation.y < clamp_neg_y {
                            sb.translation.y = clamp_neg_y
                        }
                        if sb.translation.y > clamp_pos_y {
                            sb.translation.y = clamp_pos_y
                        }
                    }
                    break
                }
                Err(_) => {

                }
            }
        }
    }
}
