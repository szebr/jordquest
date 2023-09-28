use bevy::prelude::*;

#[derive(Clone, Copy)]
enum Biome{
    Free,
    Wall,
    Grass,
    Camp,
}

// Set the size of the map in tiles (its a square)
const MAPSIZE: usize = 32;

pub const TILE_SIZE: f32 = 100.;
pub const LEVEL_W: f32 = 1920.;
pub const LEVEL_H: f32 = 1080.;

#[derive(Component)]
struct Brick;

#[derive(Component)]
struct Background;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        //basic background
        app.add_systems(Startup, startup);
        //new background
        //app.add_systems(Startup, setup)
    }
}
fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
            texture: asset_server.load("bg.png"),
            transform: Transform::default(),
            ..default()
        })
        .insert(Background);
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    
    // fill the map with tiles
    let mut biome_map: [[Biome; MAPSIZE]; MAPSIZE] = [[Biome::Free; MAPSIZE]; MAPSIZE];
    for row in 0..MAPSIZE {
        for col in 0..MAPSIZE {
            if row == 0 || row == MAPSIZE - 1 || col == 0 || col == MAPSIZE - 1 {
                // Set the outer border to `Biome::Wall`
                biome_map[row][col] = Biome::Wall;
            }else if is_between(&row, 10, 20) && is_between(&col, 10, 20){
                // create a camp near the center of the map
                biome_map[row][col] = Biome::Camp;
            }else {
                //make every other row basic grass
                biome_map[row][col] = Biome::Grass;
            }
        }
    }
    
    //TODO: draw the map tiles
}

fn is_between(x: &usize, lower: usize, upper: usize) -> bool{
    if *x <= upper && *x >= lower {
        true
    }else {
        false
    }
}