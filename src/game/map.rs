use bevy::prelude::*;
use csv::ReaderBuilder;
use std::{error::Error, fs::File};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Biome{
    Free,
    Wall,
    Grass,
    Camp,
}

#[derive(Component)]
pub struct WorldMap{
    pub map_size: usize,
    pub tile_size: usize,
    pub biome_map: [[Biome; MAPSIZE]; MAPSIZE],
}

// Set the size of the map in tiles (its a square)
//CHANGE THIS TO CHANGE MAP SIZE
// For test map, may eventually want to make this dependent on map dimensions in csv
pub const MAPSIZE: usize = 64;
pub const TILESIZE: usize = 32;

// //THESE ARE CURRENTLY ONLY HERE SO THAT THE CAMERA WORKS, I HAVEN'T DONE ANYTHING WITH THAT YET
// pub const TILE_SIZE: f32 = 100.;
// pub const LEVEL_W: f32 = 1920.;
// pub const LEVEL_H: f32 = 1080.;

//this wasn't being used
// #[derive(Component)]
// struct Brick;

#[derive(Component)]
struct Background;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        //basic background
        //app.add_systems(Startup, startup);
        //new background
        app.add_systems(Startup, setup);
    }
}

//Replaced RD's code
// fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn(SpriteBundle {
//             texture: asset_server.load("bg.png"),
//             transform: Transform::default(),
//             ..default()
//         })
//         .insert(Background);
// }

fn read_map(map: &mut WorldMap) -> Result<(), Box<dyn Error>> {
    let path = "assets/test_map.csv";
    let file = File::open(path)?;
    //let mut reader = csv::Reader::from_reader(file);
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let mut row = 0;
    let mut col = 0;

    for result in reader.records() {
        let record = result?;
        for field in record.iter() {
            match field {
                "F" => {
                    map.biome_map[row][col] = Biome::Free;
                }
                "W" => {
                    map.biome_map[row][col] = Biome::Wall;
                }
                "G" => {
                    map.biome_map[row][col] = Biome::Grass;
                }
                "C" => {
                    map.biome_map[row][col] = Biome::Camp;
                }
                &_ => {

                }
            };
            col += 1;
        }
        row += 1;
        col = 0;
    }
    Ok(())
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    
    let mut world_map = WorldMap{
        map_size: MAPSIZE,
        tile_size: TILESIZE,
        biome_map: [[Biome::Free; MAPSIZE]; MAPSIZE]
    };

    let _ = read_map(&mut world_map);

    // TODO: Determine what causes the map to be drawn rotated 90 degrees ccw
    // Draw the map tiles
    // Adding 0.5 to x_coord and y_coord will put (0,0) in the actual center of the map,
    // in between tiles, rather than in the center of a tile
    // Create this to center the x-positions of the map
    let mut x_coord: f32 = -((MAPSIZE as f32)/2.) + 0.5;
    for row in 0..MAPSIZE {
        // Create this to center the y-positions of the map
        let mut y_coord: f32 = ((MAPSIZE as f32)/2.) - 0.5;
        for col in 0..MAPSIZE {
            if world_map.biome_map[col][row] == Biome::Wall {
                // Spawn a wall sprite if the current tile is a wall
                commands.spawn(SpriteBundle {
                    texture: asset_server.load("wall.png"),
                    transform: Transform::from_xyz(x_coord*TILESIZE as f32, y_coord*TILESIZE as f32, 0.0),
                    ..default()
                });
                
            }else if world_map.biome_map[col][row] == Biome::Grass {
                // Spawn a grass sprite if the current tile is grass
                commands.spawn(SpriteBundle {
                    texture: asset_server.load("ground.png"),
                    transform: Transform::from_xyz(x_coord*TILESIZE as f32, y_coord*TILESIZE as f32, 0.0),
                    ..default()
                });
            }else if world_map.biome_map[col][row] == Biome::Camp {
                // Spawn a camp sprite if the current tile is a camp
                commands.spawn(SpriteBundle {
                    texture: asset_server.load("camp.png"),
                    transform: Transform::from_xyz(x_coord*TILESIZE as f32, y_coord*TILESIZE as f32, 0.0),
                    ..default()
                });
            }
            y_coord-=1.0;
        }
        x_coord+=1.0;
    }
}

pub fn get_surrounding_tiles(
    player: &Vec2,
    map: &[[Biome; MAPSIZE]; MAPSIZE],
) -> [[Biome; 3]; 3] {
    
    //align the player's x position to be the leftmost pixel of a given tile
    let player_x_whole = player.x.floor() as isize;
    let x_aligned = {
        if player_x_whole % TILESIZE as isize != 0{
            player_x_whole - (player_x_whole % TILESIZE as isize)
        }else{
            player_x_whole 
        }
    };
    //convert aligned x pos of player into a col of map array
    let tile_col:usize = ((x_aligned/TILESIZE as isize) + (MAPSIZE as isize/2)) as usize;
    
    //align the player's y position to be the topmost pixel of a given tile
    let player_y_whole = player.y.floor() as isize;
    let y_aligned = {
        if player_y_whole % TILESIZE as isize != 0{
            player_y_whole + (player_y_whole % TILESIZE as isize)
        }else{
            player_y_whole
        }
    };
    //convert aligned y pos of player into a row of map array
    let tile_row:usize = ((MAPSIZE as isize/2) - (y_aligned/TILESIZE as isize)) as usize;
    
    //return an array of enums for the 9 surrounding tiles
    let tiles: [[Biome; 3]; 3] = [
        [map[tile_col-1][tile_row-1], map[tile_col][tile_row-1], map[tile_col+1][tile_row-1]],
        [map[tile_col-1][tile_row], map[tile_col][tile_row], map[tile_col+1][tile_row]],
        [map[tile_col-1][tile_row+1], map[tile_col][tile_row+1], map[tile_col+1][tile_row+1]]
    ];
    tiles
}