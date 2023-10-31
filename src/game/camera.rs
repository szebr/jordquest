use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use crate::game::player::LocalPlayer;
use crate::{map, map::WorldMap};
use crate::movement;
use crate::AppState;

pub const GAME_PROJ_SCALE: f32 = 0.5;

const MINIMAP_DIMENSIONS: UVec2 = UVec2::new(256, 256);
const MINIMAP_PAD: UVec2 = UVec2::new(32, 32); // How many pixels between top right of window and top right of minimap (not border)
const MINIMAP_TRANSLATION: Vec3 = Vec3::new(
    ((super::WIN_W / 2.) as u32 - MINIMAP_PAD.x - (MINIMAP_DIMENSIONS.x / 2)) as f32 * GAME_PROJ_SCALE,
    ((super::WIN_H / 2.) as u32 - MINIMAP_PAD.y - (MINIMAP_DIMENSIONS.y / 2)) as f32 * GAME_PROJ_SCALE,
    5.
);

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct Marker;

#[derive(Component)]
pub struct Minimap;

#[derive(Component)]
pub struct MinimapBorder;

#[derive(Component)]
pub struct SpatialCameraBundle;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, update.after(movement::handle_move))
            .add_systems(OnEnter(AppState::Game), spawn_minimap);
    }
}

fn startup(
    commands: Commands,
) {
    spawn_camera(commands);
}

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn((
        SpatialBundle {
            ..Default::default()
        },
        SpatialCameraBundle,
    )).with_children(|parent|{
            parent.spawn((
                Camera2dBundle {
                    projection: OrthographicProjection{near: -1000., scale: GAME_PROJ_SCALE, ..default()},
                    ..default()
                },
                GameCamera,
            ));
        },
    );
}

fn spawn_minimap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<Image>>,
    map: Res<WorldMap>,
    mut cam_bundle: Query<Entity, With<SpatialCameraBundle>>
) {
    let border_ent = commands.spawn((
        SpriteBundle {
            texture: asset_server.load("minimap_border.png"),
            transform: Transform {
                translation: MINIMAP_TRANSLATION,
                scale: Vec3::new(GAME_PROJ_SCALE, GAME_PROJ_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        MinimapBorder,
    )).id();

    let minimap: Image = draw_minimap(map);
    let minimap_handle = assets.add(minimap);

    let minimap_ent = commands.spawn((
        SpriteBundle {
            texture: minimap_handle,
            transform: Transform {
                translation: Vec3 {
                    x: MINIMAP_TRANSLATION.x,
                    y: MINIMAP_TRANSLATION.y,
                    z: MINIMAP_TRANSLATION.z + 1.
                },
                scale: Vec3::new(GAME_PROJ_SCALE, GAME_PROJ_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        Minimap,
    )).id();

    let marker_ent = commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player_marker.png"),
            transform: Transform {
                translation: Vec3 {
                    x: MINIMAP_TRANSLATION.x,
                    y: MINIMAP_TRANSLATION.y,
                    z: MINIMAP_TRANSLATION.z + 2.
                },
                scale: Vec3::new(GAME_PROJ_SCALE, GAME_PROJ_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        Marker,
    )).id();

    for parent in &mut cam_bundle {
        commands.entity(parent).add_child(border_ent);
        commands.entity(parent).add_child(minimap_ent);
        commands.entity(parent).add_child(marker_ent);
    }
}

fn draw_minimap(
    map: Res<WorldMap>,
) -> Image 
{
    let mut minimap_data: Vec<u8> = Vec::new();

    // Create data vec with 4 bytes per pixel from map data
    for row in 0..map::MAPSIZE {
        for col in 0..map::MAPSIZE {
            let tile = map.biome_map[row][col];
            let mut rgba: Vec<u8>;

            match tile {
                map::Biome::Free => {
                    rgba = vec![255,255,255,255];
                }
                map::Biome::Wall => {
                    //rgba = vec![55,59,94,255]; // FULL COLOR
                    rgba = vec![87,53,0,255]; // SEPIA
                }
                map::Biome::Ground => {
                    //rgba = vec![62,166,10,255]; // FULL COLOR
                    rgba = vec![182,136,66,255]; // SEPIA
                }
                map::Biome::Camp => {
                    //rgba = vec![71,109,40,255]; // FULL COLOR
                    rgba = vec![131,91,20,255]; // SEPIA
                }
            }
            minimap_data.append(&mut rgba);
        }
    }

    let minimap = Image::new(
        Extent3d{
            width: MINIMAP_DIMENSIONS.x,
            height: MINIMAP_DIMENSIONS.y,
            depth_or_array_layers: 1
        },
        TextureDimension::D2,
        minimap_data,
        TextureFormat::Rgba8UnormSrgb
    );

    return minimap;
}

fn update(
    player: Query<&Transform, (With<LocalPlayer>, Without<Marker>, Without<SpatialCameraBundle>)>,
    mut marker: Query<&mut Transform, (With<Marker>, Without<SpatialCameraBundle>, Without<LocalPlayer>)>,
    mut camera: Query<&mut Transform, (With<SpatialCameraBundle>, Without<Marker>, Without<LocalPlayer>)>
) {
    for player_tf in &player {
        for mut marker_tf in &mut marker {
            if player_tf.translation.x > -(((map::MAPSIZE / 2) * map::TILESIZE) as f32) && player_tf.translation.x < ((map::MAPSIZE / 2) * map::TILESIZE) as f32 {
                marker_tf.translation.x = MINIMAP_TRANSLATION.x + player_tf.translation.x / 32.
            }
            if player_tf.translation.y > -(((map::MAPSIZE / 2) * map::TILESIZE) as f32) && player_tf.translation.y < ((map::MAPSIZE / 2) * map::TILESIZE) as f32 {
                marker_tf.translation.y = MINIMAP_TRANSLATION.y + player_tf.translation.y / 32.
            }
        }
    
        for mut camera_tf in &mut camera {
            camera_tf.translation.x = player_tf.translation.x;
            camera_tf.translation.y = player_tf.translation.y;

            let clamp_neg_x: f32 = ((-((map::MAPSIZE * map::TILESIZE) as isize)/2) + (((super::WIN_W as f32 * GAME_PROJ_SCALE) / 2.) as isize)) as f32;
            let clamp_pos_x: f32 = ((((map::MAPSIZE * map::TILESIZE) as isize)/2) - (((super::WIN_W as f32 * GAME_PROJ_SCALE) / 2.) as isize)) as f32;

            let clamp_neg_y: f32 = ((-((map::MAPSIZE * map::TILESIZE) as isize)/2) + (((super::WIN_H as f32 * GAME_PROJ_SCALE) / 2.) as isize)) as f32;
            let clamp_pos_y: f32 = ((((map::MAPSIZE * map::TILESIZE) as isize)/2) - (((super::WIN_H as f32 * GAME_PROJ_SCALE) / 2.) as isize)) as f32;

            // Clamp camera view to map borders
            // Center camera in axis if map dimensions < window size
            if map::MAPSIZE * map::TILESIZE < super::WIN_W as usize {
                camera_tf.translation.x = 0.
            }
            else {
                if camera_tf.translation.x < clamp_neg_x {
                    camera_tf.translation.x = clamp_neg_x
                }
                if camera_tf.translation.x > clamp_pos_x {
                    camera_tf.translation.x = clamp_pos_x
                }
            }

            if map::MAPSIZE * map::TILESIZE < super::WIN_H as usize {
                camera_tf.translation.y = 0.
            }
            else {
                if camera_tf.translation.y < clamp_neg_y {
                    camera_tf.translation.y = clamp_neg_y
                }
                if camera_tf.translation.y > clamp_pos_y {
                    camera_tf.translation.y = clamp_pos_y
                }
            }
        }
    }
}