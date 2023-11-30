use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::utils::petgraph::{algo::min_spanning_tree, visit::EdgeRef, graph::UnGraph, data::FromElements};
use std::error::Error;
use rand::{Rng,seq::SliceRandom,RngCore};
use rand_chacha::{rand_core::SeedableRng,ChaChaRng};
use crate::noise::Perlin;
use crate::AppState;
use crate::menus::components::{NumCampsInput, MapSeedInput};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Biome{
    Free,
    Wall,
    Ground,
    Camp,
    Path,
}

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Camp;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Path;

#[derive(Resource)]
pub struct WorldMap{
    pub map_size: usize,
    pub tile_size: usize,
    pub biome_map: [[Biome; MAPSIZE]; MAPSIZE],
}

#[derive(Resource)]
pub struct CampNodes(pub Vec<Vec2>);

#[derive(Resource)]
pub struct MapSeed(pub u64);

#[derive(Resource)]
pub struct NumCamps(pub u8);

#[derive(Resource)]
pub struct ChestCoords(pub Vec<Vec2>);

// Set the size of the map in tiles (its a square)
// CHANGE THIS TO CHANGE MAP SIZE
pub const MAPSIZE: usize = 256;
pub const TILESIZE: usize = 16;
pub const PATHWIDTH: usize = 5; // Width of the paths in tiles
pub const CAMPSIZE: usize = 17; // Diameter of camp size in tiles
pub const MAXEGGS: usize = 5;
pub const EXTRANODES: usize = 20; // Number of extra nodes to add to the graph
pub const EXTRAPATHS: usize = 2; // Number of extra paths to add to the graph
pub const MAXCHESTS: usize = 10; // Maximum number of possible chests to spawn
pub const CHESTDIST: f32 = 50.;

// Base colors for navigable tiles
pub const BASECOLOR_GROUND: Color = Color::Rgba{red: 0.243, green: 0.621, blue: 0.039, alpha: 1.0};
pub const BASECOLOR_CAMP: Color = Color::Rgba{red: 0.278, green: 0.427, blue: 0.157, alpha: 1.0};
pub const BASECOLOR_PATH: Color = Color::Rgba{red: 0.941, green: 0.663, blue: 0.325, alpha: 1.0};
pub const BASECOLOR_WALL: Color = Color::Rgba{red: 0.216, green: 0.231, blue: 0.369, alpha: 1.0};

#[derive(Component)]
struct Background;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_map_resources);
        app.add_systems(OnEnter(AppState::Hosting), set_seed);
        app.add_systems(OnEnter(AppState::Hosting), set_num_camps);
        app.add_systems(OnExit(AppState::Hosting), set_seed);
        app.add_systems(OnExit(AppState::Hosting), set_num_camps);
        app.add_systems(OnEnter(AppState::Game), setup_map);
    }
}

// calculate the euclidean distance between two points
fn euclidean_distance(a: Vec2, b: Vec2) -> f32 {
    (a - b).length()
}

// Remove coordinates that are too close to each other or a wall
fn refine_coordinates(coords: &mut Vec<Vec2>) {
    let mut new_coords = Vec::new();
    for &coord in coords.iter() {
        let is_far_enough = new_coords.iter().all(|&new_coord| {
            euclidean_distance(coord, new_coord) > 50.0
        });

        if is_far_enough {
            if coord.x > 30.0 && coord.x < (MAPSIZE-30) as f32 && coord.y > 30.0 && coord.y < (MAPSIZE-30) as f32 
            {
                new_coords.push(coord);
            }
        }
    }
    *coords = new_coords;
}

// Create a minimum spanning tree from a vector of points
fn create_mst(points: Vec<Vec2>) -> UnGraph<Vec2, f32> {
    let mut graph: UnGraph<Vec2, f32> = UnGraph::new_undirected();
    for point in points.iter() {
        graph.add_node(*point);
    }
    for i in points.iter().enumerate() {
        for j in points.iter().enumerate() {
            if i != j {
                let distance = euclidean_distance(*i.1,*j.1);
                graph.add_edge((i.0 as u32).into(), (j.0 as u32).into(), distance);
            }
        }
    }
    return UnGraph::<Vec2, f32>::from_elements(min_spanning_tree(&graph));
}

// Initialize the WorldMap, CampNodes, MapSeed, and NumCamps resources
fn initialize_map_resources(mut commands: Commands) {
    let world_map = WorldMap{
        map_size: MAPSIZE,
        tile_size: TILESIZE,
        biome_map: [[Biome::Free; MAPSIZE]; MAPSIZE]
    };
    let camp_nodes = CampNodes(Vec::new());
    let map_seed = MapSeed(0);
    let num_camps = NumCamps(10);
    let chest_coords = ChestCoords(Vec::new());
    commands.insert_resource(world_map);
    commands.insert_resource(camp_nodes);
    commands.insert_resource(map_seed);
    commands.insert_resource(num_camps);
    commands.insert_resource(chest_coords);
}

// Set the map seed based on the MapSeedInput resource (default 0)
fn set_seed(
    map_seed_input_query: Query<&MapSeedInput>,
    mut map_seed: ResMut<MapSeed>,
) {
    let mut seed: u64 = 0;
    for input in map_seed_input_query.iter() {
        if let Ok(parsed_num) = input.value.parse::<u64>() {
            seed = parsed_num;
        }
    }
    map_seed.0 = seed;
}

// Set the number of camps based on the NumCampsInput resource (default 10)
fn set_num_camps(
    num_camps_input_query: Query<&NumCampsInput>,
    mut num_camps: ResMut<NumCamps>,
) {
    let mut num: u8 = 10;
    for input in num_camps_input_query.iter() {
        if let Ok(parsed_num) = input.value.parse::<u8>() {
            num = parsed_num;
        }
    }
    num_camps.0 = num;
}

// Generate the map using Perlin noise
fn read_map(
    map: &mut WorldMap,
    camp_nodes: &mut Vec<Vec2>,
    num_camps: &Res<NumCamps>,
    mut rng: &mut ChaChaRng,
    chest_coords: &mut Vec<Vec2>,
) -> Result<(), Box<dyn Error>> {
    // seed, amplitude, frequency, octaves
    let perlin = Perlin::new(rng.next_u64(), 1.0, 0.08, 3);

    for row in 0..MAPSIZE {
        for col in 0..MAPSIZE {
            let v = perlin.noise(row,col);
            if v < 0.32 {
                map.biome_map[row][col] = Biome::Ground;
                camp_nodes.push(Vec2::new(row as f32, col as f32));
            }
            else if v > 0.72 {
                map.biome_map[row][col] = Biome::Wall;
            }
            else {
                map.biome_map[row][col] = Biome::Ground;
            }
        }
    }

    // Refine the camp nodes so that they are not too close to each other or a wall, 
    // and shuffle them, then truncate the vector to the number of camps
    refine_coordinates(camp_nodes);
    camp_nodes.shuffle(&mut rng);
    if camp_nodes.len() > num_camps.0 as usize {
        camp_nodes.truncate(num_camps.0 as usize);
    }

    // Create a vector of coordinates for extra nodes for the graph equal to EXTRANODES variable
    let mut extra_nodes: Vec<Vec2> = Vec::new();
    for _ in 0..EXTRANODES*2 {
        if extra_nodes.len() >= EXTRANODES {
            break;
        }
        let row = rng.gen_range(0..MAPSIZE) as f32;
        let col = rng.gen_range(0..MAPSIZE) as f32;
        extra_nodes.push(Vec2::new(row, col));
    }
    refine_coordinates(&mut extra_nodes);

    // Combine the camp nodes and extra nodes into one vector
    let mut all_nodes: Vec<Vec2> = Vec::new();
    all_nodes.extend(camp_nodes.iter().cloned());
    all_nodes.extend(extra_nodes.iter().cloned());
    
    // create a mst from all nodes
    let mut all_nodes_graph = create_mst(all_nodes.to_vec());
    
    // enumerate over the graph and create paths between each node
    for edge_index in all_nodes_graph.edge_indices() {
        let (source_node_index, target_node_index) = all_nodes_graph.edge_endpoints(edge_index).unwrap();
        let source_node = &all_nodes_graph[source_node_index]; // from
        let target_node = &all_nodes_graph[target_node_index]; // to

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
            let noise_value = perlin.noise(step, 0); // Adjust dimension as needed
            let direction = direction + Vec2::new(noise_value as f32 * 0.15, noise_value as f32 * 0.15); // Adjust the scaling factor

            // Calculate the position of the current step
            let step_position = *source_node + direction * (step_ratio * distance);

            // Calculate the corresponding row and column in the map for this step
            let row = (step_position.y) as i32; // Adjust as needed
            let col = (step_position.x) as i32; // Adjust as needed

            // Update the map biomes along the path to Biome::Path
            if row < map.biome_map.len() as i32 && col < map.biome_map[0].len() as i32 {
                for row_offset in -(PATHWIDTH as i32/2)..PATHWIDTH as i32/2 {
                    for col_offset in -(PATHWIDTH as i32/2)..PATHWIDTH as i32/2 {
                        if row + row_offset <= MAPSIZE as i32 - 1 && col + col_offset <= MAPSIZE as i32 - 1
                        {
                            let v = perlin.noise((row + row_offset) as usize,(col + col_offset) as usize);
                            if v > 0.64 || v < 0.60 {
                                map.biome_map[(row + row_offset) as usize][(col + col_offset) as usize] = Biome::Path;
                            }
                        }
                    }
                }
            }
        }
    }

    // Add extra paths to the mst, making it a regular graph
    let num_nodes = all_nodes_graph.node_count();
    for _ in 0..EXTRAPATHS 
    {
        let source_node = all_nodes_graph.node_indices().nth(rng.gen_range(0..num_nodes)).unwrap();
        let target_node = all_nodes_graph.node_indices().nth(rng.gen_range(0..num_nodes)).unwrap();
        // Check if the edge already exists
        if !(all_nodes_graph.edges(source_node).any(|edge| edge.target() == target_node))
        {
            let distance = euclidean_distance(all_nodes_graph[source_node], all_nodes_graph[target_node]);
            all_nodes_graph.add_edge(source_node, target_node, distance);
        }
    }

    // Create a mst from only the camp nodes
    let camp_nodes_mst = create_mst(camp_nodes.to_vec());
    // Make the camps bigger by expanding the area around the camp tiles, 
    // but using Perlin Noise to determine which tiles to expand to

    // Define the radius of the camp circle
    let camp_radius = CAMPSIZE / 2;

    for node in camp_nodes_mst.node_indices() {
        let node = &camp_nodes_mst[node];
        let center_row = node.y as usize;
        let center_col = node.x as usize;

        // create a guaranteed clear circle for each camp
        for row_offset in -(camp_radius as i32)..=camp_radius as i32 {
            for col_offset in -(camp_radius as i32)..=camp_radius as i32 {
                let row = center_row as i32 + row_offset;
                let col = center_col as i32 + col_offset;

                // Check if the current position is within the camp circle
                let distance_squared = (row - center_row as i32).pow(2) + (col - center_col as i32).pow(2);
                let camp_radius_squared = (camp_radius as i32).pow(2);

                if row >= 0 && row < MAPSIZE as i32 && col >= 0 && col < MAPSIZE as i32 && distance_squared <= camp_radius_squared {

                    //let v = perlin.noise(row as usize, col as usize);
                    // if distance_squared <= camp_radius_squared 
                    //&& v < 0.99 
                    {
                        map.biome_map[row as usize][col as usize] = Biome::Camp;
                    }
                }
            }
        }
        
        // create a few eggs to make it look a lil crazy
        for _n in 1..rng.gen_range(2..MAXEGGS){
            // randomly choose the position of the egg in the camp
            let egg_center_row = center_row as i32 + rng.gen_range(-(camp_radius as i32)..camp_radius as i32);
            let egg_center_col = center_col as i32 + rng.gen_range(-(camp_radius as i32)..camp_radius as i32);

            // Randomize the egg width and height for less uniformity

            // CHANGE THESE VARIABLES TO ADJUST THE SIZE OF THE EGGS
            let egg_min_w: f32 = camp_radius as f32 * 2.0;
            let egg_max_w: f32 = camp_radius as f32 * 6.0;
            let egg_min_h: f32 = camp_radius as f32 * 1.0;
            let egg_max_h: f32 = camp_radius as f32 * 4.0;

            // determine the width and height of the egg using the above variables
            let egg_width = rng.gen_range(egg_min_w..egg_max_w);
            let egg_height = rng.gen_range(egg_min_h..egg_max_h);

            // Draw the egg around the randomly selected center
            for row_offset in -(egg_height as i32 / 2)..=(egg_height as i32 / 2) {
                for col_offset in -(egg_width as i32 / 2)..=(egg_width as i32 / 2) {
                    let row = egg_center_row + row_offset;
                    let col = egg_center_col + col_offset;

                    let distance_squared = ((row - egg_center_row) as f32 / (egg_height / 2.0)).powi(2)
                        + ((col - egg_center_col) as f32 / (egg_width / 2.0)).powi(2);

                    // Check if the current position is within the egg
                    if row >= 0 && row < MAPSIZE as i32 && col >= 0 && col < MAPSIZE as i32 && distance_squared <= 1.0{
                        //skip over walls
                        if map.biome_map[row as usize][col as usize] != Biome::Wall {
                            map.biome_map[row as usize][col as usize] = Biome::Camp;
                        }
                    }
                }
            }
        }
    }

    // Generate a random low number of high-tier item chests in the map
    let numchests = rng.gen_range(2..=MAXCHESTS);

    for _ in 0..numchests {
        loop{
            let cur_chest = Vec2 {x: rng.gen_range(5..MAPSIZE - 5) as f32, y: rng.gen_range(5..MAPSIZE - 5) as f32};

            let mut valid = true;

            // check that the chest is far enough away from any camp
            for node in camp_nodes_mst.node_indices() {
                let node = &camp_nodes_mst[node];
                if euclidean_distance(cur_chest, *node) < CHESTDIST{
                    valid = false;
                }
            }
            // check that chest is not surrounded by a wall
            if map.biome_map[cur_chest.y as usize - 3][cur_chest.x as usize - 3] == Biome::Ground
            && map.biome_map[cur_chest.y as usize - 3][cur_chest.x as usize] == Biome::Ground
            && map.biome_map[cur_chest.y as usize - 3][cur_chest.x as usize + 3] == Biome::Ground
            && map.biome_map[cur_chest.y as usize][cur_chest.x as usize - 3] == Biome::Ground 
            && map.biome_map[cur_chest.y as usize][cur_chest.x as usize] == Biome::Ground 
            && map.biome_map[cur_chest.y as usize][cur_chest.x as usize + 3] == Biome::Ground 
            && map.biome_map[cur_chest.y as usize + 3][cur_chest.x as usize - 3] == Biome::Ground 
            && map.biome_map[cur_chest.y as usize + 3][cur_chest.x as usize] == Biome::Ground 
            && map.biome_map[cur_chest.y as usize + 3][cur_chest.x as usize + 3] == Biome::Ground 
            && valid{
                println!("Chest coords at x: {} y: {}", cur_chest.x, cur_chest.y);
                chest_coords.push(cur_chest);
                break;
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

// create the map, spawn the tiles, and add the WorldMap resource
pub fn setup_map(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    map_seed: Res<MapSeed>,
    num_camps: Res<NumCamps>,
    mut camp_nodes: ResMut<CampNodes>,
    mut world_map: ResMut<WorldMap>,
    mut chest_coords: ResMut<ChestCoords>,
) {
    //create an rng to randomly choose a goober in the near future
    let mut rng = rand_chacha::ChaChaRng::seed_from_u64(map_seed.0);

    // Generate the map, camp nodes, and item nodes
    let _ = read_map(&mut world_map, &mut camp_nodes.0, &num_camps, &mut rng, &mut chest_coords.0);

    // Get a handle for a pure white TILESIZE x TILESIZE image to be colored based on tile type later
    let tile_handle = assets.add(create_tile_image());

    // Load in goobers (tile overlays) and turn them into a TextureAtlas so they can be selected later
    let goober_handle = asset_server.load("goobers.png");
    let goober_dims = vec![8, 4]; // 8 cols, 4 rows
    let goober_atlas = TextureAtlas::from_grid(
        goober_handle,
        Vec2::splat(TILESIZE as f32),
        goober_dims[0],
        goober_dims[1],
        None,
        None
    );
    let goober_atlas_handle = texture_atlases.add(goober_atlas);

    // Create this to center the x-positions of the map
    let mut x_coord: f32 = -((MAPSIZE as f32)/2.) + 0.5;
    for row in 0..MAPSIZE {
        // Create this to center the y-positions of the map
        let mut y_coord: f32 = ((MAPSIZE as f32)/2.) - 0.5;
        for col in 0..MAPSIZE {
            let goober_index; // -1 means NO GOOBER!!!!!!!
            let goober_chance = vec![0.5, 0.18, 0.18, 0.18]; // Wall, Ground, Camp, Path

            if world_map.biome_map[col][row] == Biome::Wall {
                // Spawn a wall sprite if the current tile is a wall
                // If goober roll succeeds, make goober_index a random goober for that tile type, adding sheet width to wrap around and reach the correct row
                // The same logic applies to each instance of this line, just with different values for each tile
                goober_index = if rng.gen_range(0.00..1.00) < goober_chance[0] { rng.gen_range(0..2) + 3 * goober_dims[0] as i32 } else { -1 };
                spawn_tile(&mut commands, &tile_handle, &goober_atlas_handle, goober_index, Wall, &x_coord, &y_coord, BASECOLOR_WALL);
            }else if world_map.biome_map[col][row] == Biome::Ground {
                // Spawn a ground sprite if the current tile is Ground
                // Since we're blending grass tile color, hue must needs be calculated based on the identity of edge-sharing tiles
                let hue = tile_blend_color(&col, &row, &world_map);
                goober_index = if rng.gen_range(0.00..=1.00) < goober_chance[1] { rng.gen_range(0..8) } else { -1 };
                spawn_tile(&mut commands, &tile_handle, &goober_atlas_handle, goober_index, Ground, &x_coord, &y_coord, hue);
            }else if world_map.biome_map[col][row] == Biome::Camp {
                // Spawn a camp sprite if the current tile is a camp
                goober_index = if rng.gen_range(0.00..=1.00) < goober_chance[2] { rng.gen_range(0..8) + 2 * goober_dims[0] as i32 } else { -1 };
                spawn_tile(&mut commands, &tile_handle, &goober_atlas_handle, goober_index, Camp, &x_coord, &y_coord, BASECOLOR_CAMP);
            }else if world_map.biome_map[col][row] == Biome::Path {
                // Spawn a path sprite if the current tile is a path
                goober_index = if rng.gen_range(0.00..=1.00) < goober_chance[3] { rng.gen_range(0..8) + 1 * goober_dims[0] as i32 } else { -1 };
                spawn_tile(&mut commands, &tile_handle, &goober_atlas_handle, goober_index, Path, &x_coord, &y_coord, BASECOLOR_PATH);
            }
            y_coord-=1.0;
        }
        x_coord+=1.0;
    }
}

fn spawn_tile<T>(
    commands: &mut Commands,
    data: &Handle<Image>,
    goober_handle: &Handle<TextureAtlas>,
    goober_index: i32,
    component: T,
    x: &f32,
    y: &f32,
    hue: Color,
) where
    T: Component,
{
    let tile = commands.spawn(SpriteBundle{
        sprite: Sprite {
            color: hue,
            ..default()
        },
        transform: Transform::from_xyz(x*TILESIZE as f32, y*TILESIZE as f32, 0.),
        texture: data.clone(),
        ..default()
    })
    .insert(component)
    .id();

    if goober_index != -1 {
        // Goober is allowed
        commands.entity(tile).insert(SpriteSheetBundle{
            texture_atlas: goober_handle.clone(),
            transform: Transform::from_xyz(x*TILESIZE as f32, y*TILESIZE as f32, 0.),
            sprite: TextureAtlasSprite {
                index: goober_index as usize,
                ..default()
            },
            ..default()
        });
    }
}

fn tile_blend_color(
    x: &usize,
    y: &usize,
    world_map: &WorldMap,
) -> Color {
    // Iterate through each edge-sharing tile of the tile at (x, y)
    // If a path tile is found, return a Color that averages the colors of a ground and path tile together
    for (tile_x, tile_y) in [(*x, y - 1), (*x, y + 1), (x - 1, *y), (x + 1, *y)].iter() {
        if world_map.biome_map[*tile_x][*tile_y] == Biome::Path {
            return Color::Rgba{
                red: (BASECOLOR_GROUND.r() + BASECOLOR_PATH.r()) / 2.,
                green: (BASECOLOR_GROUND.g() + BASECOLOR_PATH.g()) / 2.,
                blue: (BASECOLOR_GROUND.b() + BASECOLOR_PATH.b()) / 2.,
                alpha: 1.
            };
        }
        else if world_map.biome_map[*tile_x][*tile_y] == Biome::Camp {
            // Uncomment to have grass bordering camp biome blend color
            /*
            return Color::Rgba{
                red: (BASECOLOR_GROUND.r() + BASECOLOR_CAMP.r()) / 2.,
                green: (BASECOLOR_GROUND.g() + BASECOLOR_CAMP.g()) / 2.,
                blue: (BASECOLOR_GROUND.b() + BASECOLOR_CAMP.b()) / 2.,
                alpha: 1.
            };
            */
        }

    }

    return BASECOLOR_GROUND;
}

// Returns a pure white image of size (TILESIZE, TILESIZE) for use with spawn_tile()
fn create_tile_image() -> Image {
    let mut pixel_data: Vec<u8> = Vec::new();

    for _ in 0..TILESIZE {
        for _ in 0..TILESIZE {
            pixel_data.append(&mut vec![255,255,255,255]);
        }
    }

    return Image::new(
        Extent3d{
            width: TILESIZE as u32,
            height: TILESIZE as u32,
            depth_or_array_layers: 1
        },
        TextureDimension::D2,
        pixel_data,
        TextureFormat::Rgba8UnormSrgb
    );
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
    map[row.clamp(0, MAPSIZE - 1)][col.clamp(0, MAPSIZE - 1)]
}

pub fn get_pos_in_tile(
    pos: &Vec3,
) -> Vec2 {
    let x = ((pos.x % TILESIZE as f32) + TILESIZE as f32) % TILESIZE as f32;
    let y = ((pos.y % TILESIZE as f32) + TILESIZE as f32) % TILESIZE as f32;
    Vec2::new(x, y)
}