use bevy::{prelude::*, utils::HashMap};
use std::{
    error::Error, 
    // fs::File, 
    // thread::spawn
};
use rand::Rng;
use crate::noise::Perlin;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Biome{
    Free,
    Wall,
    Ground,
    Camp,
}

#[derive(PartialEq, Eq, Hash)]
enum SheetTypes{
    Ground,
    Camp,
    Wall,
}

struct SheetData {
    len: usize,
    handle: Handle<TextureAtlas>,
}

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Camp;

#[derive(Component)]
struct Wall;

#[derive(Resource)]
pub struct WorldMap{
    pub map_size: usize,
    pub tile_size: usize,
    pub biome_map: [[Biome; MAPSIZE]; MAPSIZE],
}

// Set the size of the map in tiles (its a square)
//CHANGE THIS TO CHANGE MAP SIZE
pub const MAPSIZE: usize = 512;
pub const TILESIZE: usize = 16;

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

fn read_map(map: &mut WorldMap) -> Result<(), Box<dyn Error>> {
    // new perlin noise generator with random u64 as seed
    let mut rng = rand::thread_rng();
    let random_u64: u64 = rng.gen();
    // seed, amplitude, frequency, octaves
    let perlin = Perlin::new(random_u64, 1.0, 0.08, 3);

    for row in 0..MAPSIZE {
        for col in 0..MAPSIZE {
            let v = perlin.noise(row,col);
            /*let r = (255 as f64 * v);
            let y: u32 = r as u32;
            let t = y % 85 as u32;
            let x = y - t;*/

            if v < 0.4 {
                map.biome_map[row][col] = Biome::Free;
            }
            if v < 0.7 {
                map.biome_map[row][col] = Biome::Ground;
            }
            else {
                map.biome_map[row][col] = Biome::Camp;
            }
            if row % (MAPSIZE-1) == 0 {
                map.biome_map[row][col] = Biome::Wall;
            }
            if col % (MAPSIZE-1) == 0 {
                map.biome_map[row][col] = Biome::Wall;
            }
        }
    }
    Ok(())
}

pub fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    
    //Initialize the WorldMap Component
    let mut world_map = WorldMap{
        map_size: MAPSIZE,
        tile_size: TILESIZE,
        biome_map: [[Biome::Free; MAPSIZE]; MAPSIZE]
    };
    
    //Initialize the tilesheets for ground and camp
    let sheets_data: HashMap<_,_> = [SheetTypes::Camp, SheetTypes::Ground, SheetTypes::Wall]
        .into_iter()
        .map(|s|{
            let (fname, cols, rows) = match s {
                SheetTypes::Camp => ("camptilesheet.png", 50, 1),
                SheetTypes::Ground => ("groundtilesheet.png", 50, 1),
                SheetTypes::Wall => ("wall.png", 2, 2),
            };
            let handle = asset_server.load(fname);
            let atlas = 
                TextureAtlas::from_grid(handle, Vec2::splat(TILESIZE as f32), cols, rows, None, None);
            (
                s,
                SheetData {
                    len: atlas.textures.len(),
                    handle: texture_atlases.add(atlas),
                },
            )
        })
        .collect();

    let _ = read_map(&mut world_map);

    //create an rng to randomly choose a tile from the tilesheet
    let mut rng = rand::thread_rng();
    // Create this to center the x-positions of the map
    let mut x_coord: f32 = -((MAPSIZE as f32)/2.) + 0.5;
    for row in 0..MAPSIZE {
        // Create this to center the y-positions of the map
        let mut y_coord: f32 = ((MAPSIZE as f32)/2.) - 0.5;
        for col in 0..MAPSIZE {
            let sheet_index = rng.gen_range(0..50);

            if world_map.biome_map[col][row] == Biome::Wall {
                // Spawn a wall sprite if the current tile is a wall
                spawn_tile(&mut commands, &sheets_data[&SheetTypes::Wall], sheet_index, Wall, &x_coord, &y_coord);

                
            }else if world_map.biome_map[col][row] == Biome::Ground {
                // Spawn a Ground sprite if the current tile is Ground
                spawn_tile(&mut commands, &sheets_data[&SheetTypes::Ground], sheet_index, Ground, &x_coord, &y_coord);

            }else if world_map.biome_map[col][row] == Biome::Camp {
                // Spawn a camp sprite if the current tile is a camp
                spawn_tile(&mut commands, &sheets_data[&SheetTypes::Camp], sheet_index, Camp, &x_coord, &y_coord);
            }
            y_coord-=1.0;
        }
        x_coord+=1.0;
    }
    commands.insert_resource(world_map);
}

fn spawn_tile<T>(
    commands: &mut Commands,
    data: &SheetData,
    index: usize,
    component: T,
    x: &f32,
    y: &f32,
) where
    T: Component,
{
    commands.spawn(SpriteSheetBundle{
        texture_atlas: data.handle.clone(),
        transform: Transform::from_xyz(x*TILESIZE as f32, y*TILESIZE as f32, 0.),
        sprite: TextureAtlasSprite {
            index: index % data.len,
            ..default()
        },
        ..default()
    })
    .insert(component);
}

pub fn get_surrounding_tiles(
    player: &Vec3,
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
