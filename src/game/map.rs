use bevy::utils::petgraph::adj::NodeIndex;
use bevy::{prelude::*, utils::HashMap};
use std::{
    error::Error, 
    // fs::File, 
    // thread::spawn
};
// use csv::ReaderBuilder;
use rand::Rng;
use crate::noise::Perlin;
use bevy::utils::petgraph::algo::min_spanning_tree;
use bevy::utils::petgraph::graph::{DiGraph, UnGraph};
use bevy::utils::petgraph::data::FromElements;

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
pub const PATHWIDTH: usize = 6;
pub const CAMPSIZE: usize = 10;

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

// calculate the euclidean distance between two points
fn euclidean_distance(a: Vec2, b: Vec2) -> f32 {
    (a - b).length()
}

// Remove coordinates that are too close to each other
fn simplify_coordinates(coordinates: &mut Vec<Vec2>) {
    let mut simplified_coordinates = Vec::new();

    for &coordinate in coordinates.iter() {
        let is_far_enough = simplified_coordinates.iter().all(|&simplified| {
            euclidean_distance(coordinate, simplified) > 10.0
        });

        if is_far_enough {
            simplified_coordinates.push(coordinate);
        }
    }

    *coordinates = simplified_coordinates;
}

fn create_mst(points: Vec<Vec2>) -> UnGraph<Vec2, f32> {
    // Create an undirected graph
    let mut graph: UnGraph<Vec2, f32> = UnGraph::new_undirected();

    // Add nodes from points to the graph
    for point in points.iter() {
        graph.add_node(*point);
    }

    // Add edges using points and distance between points
    for i in points.iter().enumerate() {
        for j in points.iter().enumerate() {
            if i != j {
                let distance = euclidean_distance(*i.1,*j.1);
                graph.add_edge((i.0 as u32).into(), (j.0 as u32).into(), distance);
            }
        }
    }

    // Find the minimum spanning tree
    let mst = UnGraph::<Vec2, f32>::from_elements(min_spanning_tree(&graph));

    mst
}

// Perlin Noise Generated Map (for post midterm)
fn read_map(
    map: &mut WorldMap,
    camp_nodes: &mut Vec<Vec2>,
) -> Result<(), Box<dyn Error>> {
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

            if v < 0.32 {
                map.biome_map[row][col] = Biome::Ground;
                camp_nodes.push(Vec2::new(row as f32, col as f32));
            }
            else if v > 0.68 {
                map.biome_map[row][col] = Biome::Wall;
            }
            else {
                map.biome_map[row][col] = Biome::Ground;
            }
        }
    }

    // Any camp tiles that are too close to each other are removed from camp_nodes
    // Because we only need the coordinate for the camp, not the coordinates for all the tiles in a camp
    simplify_coordinates(camp_nodes);

    // create a mst from the graph
    let mst = create_mst(camp_nodes.to_vec());
    
    // enumerate over the mst and create paths between each node
    for edge_index in mst.edge_indices() {
        let (source_node_index, target_node_index) = mst.edge_endpoints(edge_index).unwrap();
        let source_node = &mst[source_node_index]; // from
        let target_node = &mst[target_node_index]; // to

        // Calculate the direction vector for the path
        let direction = (*target_node - *source_node).normalize();
        let distance = euclidean_distance(*source_node, *target_node);

        // Number of steps between the two points
        let num_steps = distance as usize;

        // Update the map cells along the path
        for step in 0..=num_steps {
            // Calculate the ratio of the current step to the total number of steps
            let step_ratio = step as f32 / num_steps as f32;
            // randomize the direction vector a bit so the lines aren't completely straight
            let direction = direction + Vec2::new(
                rand::thread_rng().gen_range(-0.2..0.2),
                rand::thread_rng().gen_range(-0.2..0.2),
            );
            // Calculate the position of the current step
            let step_position = *source_node + direction * (step_ratio * distance);

            // Calculate the corresponding row and column in the map for this step
            let row = (step_position.y) as usize; // Adjust as needed
            let col = (step_position.x) as usize; // Adjust as needed

            // Update the map biomes along the path to Biome::Ground (camp for debug)
            if row < map.biome_map.len() && col < map.biome_map[0].len() {
                for row_offset in 0..PATHWIDTH {
                    for col_offset in 0..PATHWIDTH {
                        if row + row_offset <= MAPSIZE - 1 && col + col_offset <= MAPSIZE - 1
                        {
                            map.biome_map[row + row_offset][col + col_offset] = Biome::Ground;
                            // map.biome_map[row + row_offset][col + col_offset] = Biome::Camp; // for debug
                        }
                    }
                }
            }
        }
    }

    // Make the camps bigger by expanding the area around the camp tiles, 
    // but using Perlin Noise to determine which tiles to expand to
    for node in mst.node_indices() {
        let node = &mst[node];
        let row = node.y as usize;
        let col = node.x as usize;
        for row_offset in 0..CAMPSIZE {
            for col_offset in 0..CAMPSIZE {
                if row + row_offset <= MAPSIZE - 1 && col + col_offset <= MAPSIZE - 1 {
                    let v =  perlin.noise(row + row_offset,col + col_offset);
                    if v < 0.52 {
                        map.biome_map[row + row_offset][col + col_offset] = Biome::Camp;
                    }
                }
            }
        }
    }

    // Create the outer walls
    for row in 0..MAPSIZE {
        map.biome_map[row][0] = Biome::Wall;
        map.biome_map[row][MAPSIZE-1] = Biome::Wall;    
    }
    for col in 0..MAPSIZE {
        map.biome_map[0][col] = Biome::Wall;
        map.biome_map[MAPSIZE-1][col] = Biome::Wall;    
    }

    Ok(())
}



// CSV Read Map (for midterm)
// fn read_map(map: &mut WorldMap) -> Result<(), Box<dyn Error>> {
//     let path = "assets/midterm_map.csv";
//     let file = File::open(path)?;
//     //let mut reader = csv::Reader::from_reader(file);
//     let mut reader = ReaderBuilder::new()
//         .has_headers(false)
//         .from_reader(file);
//     let mut row = 0;
//     let mut col = 0;
//     for result in reader.records() {
//         let record = result?;
//         for field in record.iter() {
//             match field {
//                 "w" => {
//                     map.biome_map[row][col] = Biome::Wall;
//                 }
//                 "g" => {
//                     map.biome_map[row][col] = Biome::Ground;
//                 }
//                 "c" => {
//                     map.biome_map[row][col] = Biome::Camp;
//                 }
//                 &_ => {
//                 }
//             };
//             col += 1;
//         }
//         row += 1;
//         col = 0;
//     }
//     Ok(())
// }

// create the map, spawn the tiles, and add the WorldMap resource
pub fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    //Initialize the WorldMap Component and the camp_nodes vector
    let mut world_map = WorldMap{
        map_size: MAPSIZE,
        tile_size: TILESIZE,
        biome_map: [[Biome::Free; MAPSIZE]; MAPSIZE]
    };
    let mut camp_nodes: Vec<Vec2> = Vec::new();

    // Generate the map and read it into the WorldMap Component
    // Also mark the camp tiles into raw_camp_nodes
    let _ = read_map(&mut world_map, &mut camp_nodes);

    

    //Initialize the tilesheets for ground and camp
    let sheets_data: HashMap<_,_> = [SheetTypes::Camp, SheetTypes::Ground, SheetTypes::Wall]
        .into_iter()
        .map(|s|{
            let (fname, cols, rows) = match s {
                SheetTypes::Camp => ("camptilesheet.png", 50, 1),
                SheetTypes::Ground => ("groundtilesheet.png", 50, 1),
                SheetTypes::Wall => ("wall.png", 3, 1),
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

    // Spawn the map
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
    let col = (player_pos.x + (TILESIZE * MAPSIZE / 2) as f32) as isize / TILESIZE as isize;
    let row = (-player_pos.y + (TILESIZE * MAPSIZE / 2) as f32) as isize / TILESIZE as isize;
    let mut ret = [[Biome::Wall; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            ret[i][j] = get_biome_from_map(row + i as isize - 1, col + j as isize - 1, map);
        }
    }
    ret
}

pub fn get_tile_midpoint_position(
    pos: &Vec3,
    map: &[[Biome; MAPSIZE]; MAPSIZE],
) -> Vec3 {
    let offset = (TILESIZE * MAPSIZE) as f32;
    let x = (TILESIZE / 2) as f32 - (pos.x + offset) % TILESIZE as f32;
    let y = (TILESIZE / 2) as f32 - (pos.y + offset) % TILESIZE as f32;
    //println!("player at x: {:2} y: {:2} midpoint at x: {:2} y: {:2}", pos.x, pos.y, pos.x+x, pos.y+y);
    Vec3::new(pos.x + x, pos.y + y, 0.0)
}

pub fn get_biome_from_map(
    row: isize,
    col: isize,
    map: &[[Biome; MAPSIZE]; MAPSIZE],
) -> Biome {
    if row < 0 || col < 0 || row >= MAPSIZE as isize || col >= MAPSIZE as isize {
        Biome::Wall
    } else {
        map[row as usize][col as usize]
    }
}

pub fn get_tile_at_pos(
    player_pos: &Vec3,
    map: &[[Biome; MAPSIZE]; MAPSIZE],
) -> Biome {
    let col = (player_pos.x + (TILESIZE * MAPSIZE / 2) as f32) as usize / TILESIZE;
    let row = (-player_pos.y + (TILESIZE * MAPSIZE / 2) as f32) as usize / TILESIZE;
    map[row][col]
}

pub fn get_pos_in_tile(
    pos: &Vec3,
) -> Vec2 {
    let mut x = ((pos.x % TILESIZE as f32) + TILESIZE as f32) % TILESIZE as f32;
    let mut y = ((pos.y % TILESIZE as f32) + TILESIZE as f32) % TILESIZE as f32;
    Vec2::new(x, y)
}