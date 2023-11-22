use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::PrimaryWindow;
use crate::game::player::{LocalPlayer, LocalPlayerDeathEvent, LocalPlayerSpawnEvent, PLAYER_DEFAULT_HP};
use crate::{map, map::WorldMap, map::TILESIZE};
use crate::movement;
use crate::AppState;
use crate::game::components::Health;
use crate::game::player;
use crate::game::camp::setup_camps;
use crate::game::components::Camp;

pub const GAME_PROJ_SCALE: f32 = 0.5;

const MINIMAP_DIMENSIONS: UVec2 = UVec2::new(map::MAPSIZE as u32, map::MAPSIZE as u32);
const MINIMAP_PAD: UVec2 = UVec2::new(32, 32); // How many pixels between top right of window and top right of minimap (not border)
const MINIMAP_TRANSLATION: Vec3 = Vec3::new(
    ((super::WIN_W / 2.) as u32 - MINIMAP_PAD.x - (MINIMAP_DIMENSIONS.x / 2)) as f32 * GAME_PROJ_SCALE,
    ((super::WIN_H / 2.) as u32 - MINIMAP_PAD.y - (MINIMAP_DIMENSIONS.y / 2)) as f32 * GAME_PROJ_SCALE,
    5.
);

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct LocalPlayerMarker;

#[derive(Component)]
pub struct CampMarker(pub u8);

#[derive(Component)]
pub struct Minimap;

#[derive(Component)]
pub struct MinimapBorder;

#[derive(Component)]
pub struct RespawnMap;

#[derive(Component)]
pub struct SpatialCameraBundle;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, game_update.after(movement::handle_move).run_if(in_state(AppState::Game)))
            .add_systems(Update, respawn_update.run_if(player::local_player_dead))
            .add_systems(Update, marker_follow.run_if(not(player::local_player_dead)))
            .add_systems(OnEnter(AppState::Game), spawn_minimap.after(setup_camps))
            .add_systems(Update, configure_map_on_event);
    }
}

fn startup(
    commands: Commands,
) {
    spawn_camera(commands);
}

// Spawns the main game camera as a child of a SpatialBundle that will follow the player in Game state
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

// Spawns the minimap, its border, and the player marker and parents the SpatialBundle to them
pub fn spawn_minimap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<Image>>,
    map: Res<WorldMap>,
    mut cam_bundle: Query<Entity, With<SpatialCameraBundle>>,
    camps: Query<(&Camp), With<Camp>>
) {
    let border_ent = commands.spawn((
        SpriteBundle {
            texture: asset_server.load("minimap_border.png"),
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: MINIMAP_TRANSLATION.z
                },
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
                    x: 0.,
                    y: 0.,
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
                    x: 0.,
                    y: 0.,
                    z: MINIMAP_TRANSLATION.z + 2.
                },
                scale: Vec3::new(GAME_PROJ_SCALE, GAME_PROJ_SCALE, 1.),
                ..Default::default()
            },
            visibility: Visibility::Hidden, // Hide the marker initially and make it visible after first spawn
            ..Default::default()
        },
        LocalPlayerMarker,
    )).id();

    // The minimap-related bundles are made children of SpatialCameraBundle because
    // they need to remain in the same screen position and Bevy UI stuff is icky
    for parent in &mut cam_bundle {
        commands.entity(parent).add_child(border_ent);
        commands.entity(parent).add_child(minimap_ent);
        commands.entity(parent).add_child(marker_ent);

        for (camp_num) in camps.iter() {
            let camp_marker_ent = commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("player_marker.png"),
                    transform: Transform {
                        translation: Vec3 {
                            x: 0.,
                            //x: camp_pos.0.get(0).x - (MINIMAP_DIMENSIONS.x as f32 * GAME_PROJ_SCALE),
                            y: 0.,
                            //y: -(camp_pos.0.get(0).y - (MINIMAP_DIMENSIONS.y as f32 * GAME_PROJ_SCALE)),
                            z: MINIMAP_TRANSLATION.z + 1.99
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                },
                CampMarker(camp_num.0),
            )).id();

            println!("putting camp marker");

            commands.entity(parent).add_child(camp_marker_ent);
        }
    }
}

// Creates and returns the Image of the minimap from the map data
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
                map::Biome::Path => {
                    //rgba = vec![240,169,83,255]; // FULL COLOR
                    rgba = vec![241,213,166,255]; // SEPIA
                
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

// Adjusts minimap/border/marker position and size based on being in Game or Respawn state
fn configure_map_on_event(
    mut minimap: Query<&mut Transform, (With<Minimap>, Without<MinimapBorder>, Without<LocalPlayerMarker>, Without<CampMarker>, Without<SpatialCameraBundle>, Without<LocalPlayer>)>,
    mut border: Query<&mut Transform, (With<MinimapBorder>, Without<Minimap>, Without<LocalPlayerMarker>, Without<CampMarker>, Without<SpatialCameraBundle>, Without<LocalPlayer>)>,
    mut local_marker: Query<&mut Transform, (With<LocalPlayerMarker>, Without<Minimap>, Without<MinimapBorder>, Without<CampMarker>, Without<SpatialCameraBundle>, Without<LocalPlayer>)>,
    mut camp_markers: Query<&mut Transform, (With<CampMarker>, Without<Minimap>, Without<MinimapBorder>, Without<LocalPlayerMarker>, Without<SpatialCameraBundle>, Without<LocalPlayer>)>,
    camera: Query<&Transform, (With<SpatialCameraBundle>, Without<Minimap>, Without<MinimapBorder>, Without<LocalPlayerMarker>, Without<LocalPlayer>)>,
    mut death_reader: EventReader<LocalPlayerDeathEvent>,
    mut spawn_reader: EventReader<LocalPlayerSpawnEvent>
) {
    let mut spawn_mode: Option<bool> = None;
    for _ in death_reader.iter() {
        spawn_mode = Some(true);
    }
    if spawn_mode.is_none() {
        for _ in spawn_reader.iter() {
            spawn_mode = Some(false);
        }
    }
    if spawn_mode.is_none() {
        return;
    }
    // minimap mode
    let mut new_translation: Vec2 = Vec2::new(MINIMAP_TRANSLATION.x, MINIMAP_TRANSLATION.y);
    let mut new_scale: f32 = GAME_PROJ_SCALE;

    if spawn_mode.unwrap() {
        new_translation = Vec2::new(0., 0.);
        new_scale = 1.;
    }

    // Set minimap/border/marker translation/scale with aforementioned parameters
    for mut minimap_tf in &mut minimap {
        minimap_tf.translation.x = new_translation.x;
        minimap_tf.translation.y = new_translation.y;
        minimap_tf.scale.x = new_scale;
        minimap_tf.scale.y = new_scale;
    }

    for mut border_tf in &mut border {
        border_tf.translation.x = new_translation.x;
        border_tf.translation.y = new_translation.y;
        border_tf.scale.x = new_scale;
        border_tf.scale.y = new_scale;
    }

    for mut local_marker_tf in &mut local_marker {
        for camera_tf in &camera {
            local_marker_tf.translation.x = camera_tf.translation.x / 16.;
            local_marker_tf.translation.y = camera_tf.translation.y / 16.;
            local_marker_tf.scale.x = new_scale;
            local_marker_tf.scale.y = new_scale;
        }
    }

    for mut camp_marker_tf in &mut camp_markers {
        camp_marker_tf.scale.x = new_scale;
        camp_marker_tf.scale.y = new_scale;

        if spawn_mode.unwrap() {
            //camp_marker_tf.translation.x = (camp_marker_tf.translation.x * (TILESIZE as f32 / GAME_PROJ_SCALE)) - MINIMAP_TRANSLATION.x;
            //camp_marker_tf.translation.y = (camp_marker_tf.translation.y * (TILESIZE as f32 / GAME_PROJ_SCALE)) - MINIMAP_TRANSLATION.y;
        } else {
            //camp_marker_tf.translation.x = (camp_marker_tf.translation.x / (TILESIZE as f32 / GAME_PROJ_SCALE)) + MINIMAP_TRANSLATION.x;
            //camp_marker_tf.translation.y = (camp_marker_tf.translation.y / (TILESIZE as f32 / GAME_PROJ_SCALE)) + MINIMAP_TRANSLATION.y;
        }
    }
}

// Make marker reflect player position while player is alive
fn marker_follow(
    player: Query<&Transform, (With<LocalPlayer>, Without<LocalPlayerMarker>, Without<SpatialCameraBundle>)>,
    mut marker: Query<&mut Transform, (With<LocalPlayerMarker>, Without<SpatialCameraBundle>, Without<LocalPlayer>)>,
) {
    for player_tf in &player {
        // Set marker position on minimap to reflect the player's current position in the game world
        for mut marker_tf in &mut marker {
            if player_tf.translation.x > -(((map::MAPSIZE / 2) * map::TILESIZE) as f32) && player_tf.translation.x < ((map::MAPSIZE / 2) * map::TILESIZE) as f32 {
                marker_tf.translation.x = MINIMAP_TRANSLATION.x + player_tf.translation.x / 32.
            }
            if player_tf.translation.y > -(((map::MAPSIZE / 2) * map::TILESIZE) as f32) && player_tf.translation.y < ((map::MAPSIZE / 2) * map::TILESIZE) as f32 {
                marker_tf.translation.y = MINIMAP_TRANSLATION.y + player_tf.translation.y / 32.
            }
        }
    }
}

// Runs in Respawn state, waits for mouse click to get player's desired (re)spawn position
fn respawn_update(
    mouse_button_inputs: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut player: Query<(&mut Transform, &mut Health, &mut Visibility), With<LocalPlayer>>,
    mut marker_vis: Query<&mut Visibility, (With<LocalPlayerMarker>, Without<LocalPlayer>)>,
    map: Res<WorldMap>,
    mut spawn_writer: EventWriter<LocalPlayerSpawnEvent>
) {
    // Get mouse position upon click
    if mouse_button_inputs.just_pressed(MouseButton::Left) {
        let window = window_query.get_single().unwrap();
        let cursor_position = window.cursor_position().unwrap();
        println!("{}", cursor_position);

        // Validate mouse position
        let mut cursor_to_map: UVec2 = UVec2::new(0, 0);

        // Ensure cursor within respawn map bounds
        if (cursor_position.x < ((super::WIN_W / 2.) - MINIMAP_DIMENSIONS.x as f32)) ||
            (cursor_position.x > ((super::WIN_W / 2.) + MINIMAP_DIMENSIONS.x as f32)) ||
            (cursor_position.y < ((super::WIN_H / 2.) - MINIMAP_DIMENSIONS.y as f32)) ||
            (cursor_position.y > ((super::WIN_H / 2.) + MINIMAP_DIMENSIONS.y as f32))
        {
            // Outside map bounds, invalid position
        } else {
            // Within bounds, convert to map tile coordinate
            cursor_to_map.x = ((cursor_position.x as u32 - ((super::WIN_W / 2.) as u32 - MINIMAP_DIMENSIONS.x)) / 2).clamp(0, (map::MAPSIZE - 1) as u32);
            cursor_to_map.y = ((cursor_position.y as u32 - ((super::WIN_H / 2.) as u32 - MINIMAP_DIMENSIONS.y)) / 2).clamp(0, (map::MAPSIZE - 1) as u32);

            println!("{}", cursor_to_map);

            // Check if coordinate is wall
            let tile = map.biome_map[cursor_to_map.y as usize][cursor_to_map.x as usize];

            match tile {
                map::Biome::Wall => {
                    // In wall, invalid position
                }
                _ => {
                    // Valid spawn tile
                    app_state_next_state.set(AppState::Game);

                    let (mut tf, mut hp, mut vis) = player.single_mut();

                    hp.current = PLAYER_DEFAULT_HP;
                    spawn_writer.send(LocalPlayerSpawnEvent);
                    tf.translation.x = (cursor_to_map.x as f32 - 128.) * 16.;
                    tf.translation.y = -(cursor_to_map.y as f32 - 128.) * 16.;

                    // Show marker
                    for mut vis in &mut marker_vis {
                        *vis = Visibility::Visible;
                    }
                }
            }
        }
    }
}

// Runs in Game state, makes SpatialCameraBundle follow player
fn game_update(
    player: Query<&Transform, (With<LocalPlayer>, Without<LocalPlayerMarker>, Without<SpatialCameraBundle>)>,
    mut camera: Query<&mut Transform, (With<SpatialCameraBundle>, Without<LocalPlayerMarker>, Without<LocalPlayer>)>
) {
    for player_tf in &player {
        // Make SpatialCameraBundle follow player
        for mut camera_tf in &mut camera {
            camera_tf.translation.x = player_tf.translation.x;
            camera_tf.translation.y = player_tf.translation.y;

            let clamp_pos_x: f32 = ((((map::MAPSIZE * map::TILESIZE) as isize)/2) - (((super::WIN_W * GAME_PROJ_SCALE) / 2.) as isize)) as f32;
            let clamp_pos_y: f32 = ((((map::MAPSIZE * map::TILESIZE) as isize)/2) - (((super::WIN_H * GAME_PROJ_SCALE) / 2.) as isize)) as f32;

            // Clamp camera view to map borders
            // Center camera in axis if map dimensions < window size
            if map::MAPSIZE * map::TILESIZE < super::WIN_W as usize {
                camera_tf.translation.x = 0.
            }
            else {
                if camera_tf.translation.x > clamp_pos_x {
                    camera_tf.translation.x = clamp_pos_x
                }
                if camera_tf.translation.x < -clamp_pos_x {
                    camera_tf.translation.x = -clamp_pos_x;
                }
            }

            if map::MAPSIZE * map::TILESIZE < super::WIN_H as usize {
                camera_tf.translation.y = 0.
            }
            else {
                if camera_tf.translation.y > clamp_pos_y {
                    camera_tf.translation.y = clamp_pos_y
                }
                if camera_tf.translation.y < -clamp_pos_y {
                    camera_tf.translation.y = -clamp_pos_y;
                }
            }
        }
    }
}