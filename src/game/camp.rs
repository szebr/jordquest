use std::collections::VecDeque;

use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use crate::AppState;
use crate::game::enemy;
use crate::Atlas;
use crate::map::{MAPSIZE, TILESIZE, CampNodes};
use crate::components::PowerUpType;
use crate::components::*;
use crate::Decorations;
use crate::game::map::setup_map;
use crate::map::MapSeed;


const CAMP_ENEMIES: u8 = 5;
const NUM_GRADES: u8 = 5;
const DEC_SIZE: Vec2 = Vec2 {x: 32., y: 32.};

pub struct CampPlugin;

impl Plugin for CampPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(OnEnter(AppState::Game), setup_camps
            .after(setup_map));
        //app.add_systems(OnEnter(AppState::Game), spawn_camp_enemy);
        app.add_systems(Update,(
            handle_camp_clear,
        ));
    }
}

pub fn setup_camps(
    mut commands: Commands,
    entity_atlas:Res<Atlas>,
    camp_nodes: Res<CampNodes>,
    decoration_atlas: Res<Decorations>,
    map_seed: Res<MapSeed>,
    asset_server: Res<AssetServer>,
    //TODO: USE THIS FOR SEED UPDATES
    //rng: SeedableRng
) {
    //TODO: make this based off the generated seed when that gets implemented
    let mut rng = ChaChaRng::seed_from_u64(map_seed.0);
    const POWERUP_DROP_CHANCE: u32 = 50;
    // spawn a camp at a specified position

    //TODO: respawn enemies in a camp after a certain amount of time

    // Iterate through the MST of camps generated by perlin noise and spawn a camp at each node
    let mut campid: u8 = 0; 
    let mut id: u8 = 0;
    for camps in camp_nodes.0.iter(){
        // x-y position of the camp
        let camp_pos: Vec2 = get_spawn_vec(camps.x, camps.y);
        // determines camp/enemy type
        let camp_grade: u8 = rng.gen_range(1..=NUM_GRADES);
        //get the prefab data for the given grade
        let mut prefab_data = get_prefab_data(camp_grade);

        let special_enemy_index = rng.gen_range(0..CAMP_ENEMIES);

        commands.spawn((
            Camp(campid),
            SpatialBundle {
                transform: Transform::from_xyz(camp_pos.x, camp_pos.y, 0.),
                ..default()
            },
            Grade(camp_grade),
            CampEnemies{
                max_enemies: CAMP_ENEMIES,
                current_enemies: CAMP_ENEMIES,
            },
            CampStatus{
                status: true,
            },
            
        ));

        // DECORATIONS NEED TO SPAWN BEFORE ENEMIES SO THAT THE VECDEQUE IS IN THE CORRECT ORDER
        //spawn decorations here
        for n in 0..3 {
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: decoration_atlas.handle.clone(),
                    sprite: TextureAtlasSprite {index: decoration_atlas.coord_to_index(n, camp_grade as i32), ..default()},
                    transform: Transform{
                        translation: Vec3::new(
                            camp_pos.x + (prefab_data.pop_front().unwrap() * 16) as f32, 
                            camp_pos.y + (prefab_data.pop_front().unwrap() * 16) as f32, 
                            2.
                        ),
                        scale: Vec3::new(2., 2., 0.),
                        ..default()
                    },
                    ..default()
                },
                Collider(DEC_SIZE),
            ));
        }

        //spawn enemies for this camp
        for n in 0..CAMP_ENEMIES{
            let is_special = n == special_enemy_index;
            //generate a random powerup to drop from each enemy
            let powerups: [PowerUpType; 5] = [PowerUpType::MaxHPUp, PowerUpType::DamageDealtUp, PowerUpType::DamageReductionUp, PowerUpType::AttackSpeedUp, PowerUpType::MovementSpeedUp];
            //TODO: make this a random percentage based on the mapconfig resource
            let power_up_to_drop = powerups[camp_grade as usize - 1];
            let mut chance_drop_powerup = rng.gen_range(0..100) < POWERUP_DROP_CHANCE;

            if is_special{
                chance_drop_powerup = true;
            }

            enemy::spawn_enemy(
                &mut commands, 
                &asset_server,
                &entity_atlas, 
                id,
                campid, 
                Vec2::new(
                    camp_pos.x + (prefab_data.pop_front().unwrap() * 16) as f32, 
                    camp_pos.y + (prefab_data.pop_front().unwrap() * 16) as f32), 
                camp_grade as i32, 
                power_up_to_drop,
                chance_drop_powerup,
                is_special,
            );
            id += 1;
        }
        campid += 1;
    }

}

pub fn handle_camp_clear(
    mut camp_query: Query<(&CampEnemies, &mut CampStatus), With<Camp>>,
){
    for (enemies_in_camp, mut camp_status) in camp_query.iter_mut(){
        
        // only let this happen for camps that are currently active
        if camp_status.status {
            //set the camp as cleared if all enemies are gone
            if enemies_in_camp.current_enemies == 0 {
                camp_status.status = false;
            }
            
        }
    }
}

// convert given row and col into x and y coordinates. Returns a vec2 of these coordinates
fn get_spawn_vec(row: f32, col:f32) -> Vec2{
    let x_coord = TILESIZE as f32 * (row - (MAPSIZE as f32/2. + 0.5));
    let y_coord = TILESIZE as f32 * ((MAPSIZE as f32/2. - 0.5) - col);

    Vec2::new(x_coord, y_coord)
}

// given a grade, return a list of the attributes of that prefab
// LIST CONTENTS ARE:
/*
* dec 1 x offset = [0]
* dec 1 y offset = [1]
* dec 2 x offset = [2]
* dec 2 y offset = [3]
* dec 3 x offset = [4]
* dec 3 y offset = [5]
* en 1 x offset = [6]
* en 1 y offset = [7]
* en 2 x offset = [8]
* en 2 y offset = [9]
* en 3 x offset = [10]
* en 3 y offset = [11]
* en 4 x offset = [12]
* en 4 y offset = [13]
* en 5 x offset = [14]
* en 5 y offset = [15]
*/

fn get_prefab_data(grade: u8) -> VecDeque<i8>{

    let mut pd: VecDeque<i8> = VecDeque::new();
    match grade {
        1 => {
            // add decoration 1 position offset
            pd.push_back(0);
            pd.push_back(7);
            // add decoration 2 position offset
            pd.push_back(-5);
            pd.push_back(-3);
            // add decoration 3 position offset
            pd.push_back(3);
            pd.push_back(-2);
            // add enemy 1 position offset
            pd.push_back(-3);
            pd.push_back(4);
            // add enemy 2 position offset
            pd.push_back(-2);
            pd.push_back(-1);
            // add enemy 3 position offset
            pd.push_back(3);
            pd.push_back(2);
            // add enemy 4 position offset
            pd.push_back(6);
            pd.push_back(1);
            // add enemy 5 position offset
            pd.push_back(0);
            pd.push_back(-5);
        },
        2 => {
            // add decoration 1 position offset
            pd.push_back(-5);
            pd.push_back(4);
            // add decoration 2 position offset
            pd.push_back(0);
            pd.push_back(1);
            // add decoration 3 position offset
            pd.push_back(3);
            pd.push_back(-4);
            // add enemy 1 position offset
            pd.push_back(-2);
            pd.push_back(7);
            // add enemy 2 position offset
            pd.push_back(-6);
            pd.push_back(-3);
            // add enemy 3 position offset
            pd.push_back(-3);
            pd.push_back(-6);
            // add enemy 4 position offset
            pd.push_back(2);
            pd.push_back(-3);
            // add enemy 5 position offset
            pd.push_back(4);
            pd.push_back(3);
        },
        3 => {
            // add decoration 1 position offset
            pd.push_back(-1);
            pd.push_back(5);
            // add decoration 2 position offset
            pd.push_back(-3);
            pd.push_back(-2);
            // add decoration 3 position offset
            pd.push_back(3);
            pd.push_back(-2);
            // add enemy 1 position offset
            pd.push_back(-4);
            pd.push_back(5);
            // add enemy 2 position offset
            pd.push_back(-2);
            pd.push_back(1);
            // add enemy 3 position offset
            pd.push_back(-6);
            pd.push_back(0);
            // add enemy 4 position offset
            pd.push_back(4);
            pd.push_back(0);
            // add enemy 5 position offset
            pd.push_back(-4);
            pd.push_back(-4);
        },
        4 => {
            // add decoration 1 position offset
            pd.push_back(-3);
            pd.push_back(4);
            // add decoration 2 position offset
            pd.push_back(3);
            pd.push_back(2);
            // add decoration 3 position offset
            pd.push_back(5);
            pd.push_back(-1);
            // add enemy 1 position offset
            pd.push_back(2);
            pd.push_back(6);
            // add enemy 2 position offset
            pd.push_back(-5);
            pd.push_back(1);
            // add enemy 3 position offset
            pd.push_back(-2);
            pd.push_back(1);
            // add enemy 4 position offset
            pd.push_back(-4);
            pd.push_back(-3);
            // add enemy 5 position offset
            pd.push_back(1);
            pd.push_back(-6);
        },
        _ => {
            // add decoration 1 position offset
            pd.push_back(3);
            pd.push_back(3);
            // add decoration 2 position offset
            pd.push_back(2);
            pd.push_back(-2);
            // add decoration 3 position offset
            pd.push_back(-5);
            pd.push_back(-5);
            // add enemy 1 position offset
            pd.push_back(4);
            pd.push_back(6);
            // add enemy 2 position offset
            pd.push_back(-3);
            pd.push_back(4);
            // add enemy 3 position offset
            pd.push_back(5);
            pd.push_back(0);
            // add enemy 4 position offset
            pd.push_back(-6);
            pd.push_back(-2);
            // add enemy 5 position offset
            pd.push_back(-1);
            pd.push_back(-4);
        },
    }
    pd
}