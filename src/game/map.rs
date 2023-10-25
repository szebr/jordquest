use bevy::{prelude::*, utils::HashMap};
use std::{
    error::Error, 
    fs::File, 
    // thread::spawn
};
use csv::ReaderBuilder;
use rand::Rng;
use serde::de::Unexpected::Map;
//use crate::noise::Perlin;

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
pub const MAPSIZE: usize = 256;
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

// Perlin Noise Generated Map (for post midterm)
// fn read_map(map: &mut WorldMap) -> Result<(), Box<dyn Error>> {
//     // new perlin noise generator with random u64 as seed
//     let mut rng = rand::thread_rng();
//     let random_u64: u64 = rng.gen();
//     // seed, amplitude, frequency, octaves
//     let perlin = Perlin::new(random_u64, 1.0, 0.08, 3);

//     for row in 0..MAPSIZE {
//         for col in 0..MAPSIZE {
//             let v = perlin.noise(row,col);
//             /*let r = (255 as f64 * v);
//             let y: u32 = r as u32;
//             let t = y % 85 as u32;
//             let x = y - t;*/

//             if v < 0.4 {
//                 map.biome_map[row][col] = Biome::Free;
//             }
//             if v < 0.7 {
//                 map.biome_map[row][col] = Biome::Ground;
//             }
//             else {
//                 map.biome_map[row][col] = Biome::Camp;
//             }
//             if row % (MAPSIZE-1) == 0 {
//                 map.biome_map[row][col] = Biome::Wall;
//             }
//             if col % (MAPSIZE-1) == 0 {
//                 map.biome_map[row][col] = Biome::Wall;
//             }
//         }
//     }
//     Ok(())
// }

// CSV Read Map (for midterm)
fn read_map(map: &mut WorldMap) -> Result<(), Box<dyn Error>> {
    let path = "assets/midterm_map.csv";
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
                "w" => {
                    map.biome_map[row][col] = Biome::Wall;
                }
                "g" => {
                    map.biome_map[row][col] = Biome::Ground;
                }
                "c" => {
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
    player_pos: &Vec3,
    map: &[[Biome; MAPSIZE]; MAPSIZE],
) -> [[Biome; 3]; 3] {
    let col = (player_pos.x + (TILESIZE * MAPSIZE / 2) as f32).round() as usize / TILESIZE;
    let row = (-player_pos.y + (TILESIZE * MAPSIZE / 2) as f32).round() as usize / TILESIZE;
    if row == 0 || row == MAPSIZE-1 || col == 0 || col == MAPSIZE-1 {
        [[Biome::Wall, Biome::Wall, Biome::Wall],
         [Biome::Wall, Biome::Wall, Biome::Wall],
         [Biome::Wall, Biome::Wall, Biome::Wall]]
    } else {
        [[map[row - 1][col - 1], map[row - 1][col], map[row - 1][col + 1]],
         [map[row][col - 1], map[row][col], map[row][col + 1]],
         [map[row + 1][col - 1], map[row + 1][col], map[row + 1][col + 1]]]
    }
}