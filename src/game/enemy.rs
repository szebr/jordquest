use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use crate::{AppState, net};
use crate::Atlas;
use movement::correct_wall_collisions;
use crate::game::buffers::{CircularBuffer, PosBuffer};
use crate::game::components::*;
use crate::net::{is_client, is_host, TickNum};
use crate::game::components::PowerUpType;
use crate::game::map::{Biome, TILESIZE, MAPSIZE, WorldMap};
use crate::game::movement;
use crate::game::player::PlayerShield;
use super::player::PLAYER_DEFAULT_DEF;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use crate::PowerupAtlas;

pub const ENEMY_SIZE: Vec2 = Vec2 { x: 32., y: 32. };
pub const ENEMY_SPEED: f32 = 150. / net::TICKRATE as f32;
pub const ENEMY_MAX_HP: u8 = 100;
pub const AGGRO_RANGE: f32 = 200.0;
pub const ATTACK_RATE: f32 = 4.0;
// special enemy modifiers are all multiplicative
pub const SPECIAL_ATTACK_RADIUS_MOD: f32 = 1.5;
pub const SPECIAL_MAX_HP_MOD: f32 = 1.5; // cannot be more than 2.55 due to u8 max
pub const SPECIAL_ATTACK_RATE_MOD: f32 = 0.5;


const CIRCLE_RADIUS: f32 = 64.;
const CIRCLE_DAMAGE: u8 = 15;

#[derive(Component)]
pub struct EnemyWeapon;

#[derive(Component)]
pub struct LastAttacker(pub Option<u8>);

#[derive(Component)]
pub struct Aggro(pub Option<u8>);

#[derive(Component)]
struct DespawnEnemyWeaponTimer(Timer);

#[derive(Component)]
pub struct SpawnEnemyWeaponTimer(Timer);

#[derive(Component)]
pub struct EnemyRegenTimer(Timer);

#[derive(Component)]
pub struct IsSpecial(bool);

#[derive(Component)]
pub struct SpawnPosition(pub Vec2);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (
                fixed_aggro,
                fixed_move.after(fixed_aggro),
                fixed_resolve.run_if(in_state(AppState::Game)).after(fixed_move).after(net::lerp::resolve_collisions),
                ).run_if(is_host)
            )
            .add_systems(Update, (
                handle_packet.run_if(is_client),
                update_enemies.after(handle_packet),
                handle_attack.after(update_enemies),
                enemy_regen_health.after(update_enemies),
            ))
            .add_systems(OnExit(AppState::Game), remove_enemies);
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    entity_atlas: &Res<Atlas>, 
    id: u8, 
    campid: u8, 
    pos: Vec2, 
    sprite: i32, 
    power_up_type: PowerUpType,
    chance_drop_powerup: bool,
    is_special: bool,
) {
    let pb = PosBuffer(CircularBuffer::new_from(pos));
    let mut pu: [u8; NUM_POWERUPS];
    pu = [0; NUM_POWERUPS];
    pu[power_up_type as usize] = 1;
    let enemy_hp;
    let enemy_attack_rate;
    if is_special {
        enemy_hp = (ENEMY_MAX_HP as f32 * SPECIAL_MAX_HP_MOD) as u8;
        enemy_attack_rate = ATTACK_RATE * SPECIAL_ATTACK_RATE_MOD;
    } else {
        enemy_hp = ENEMY_MAX_HP;
        enemy_attack_rate = ATTACK_RATE;
    }

    let enemy_entity = commands.spawn((
        Enemy(id),
        pb,
        SpawnPosition(pos),
        Health {
            current: enemy_hp,
            max: enemy_hp,
            dead: false,
        },
        EnemyCamp(campid),
        SpriteSheetBundle {
            texture_atlas: entity_atlas.handle.clone(),
            sprite: TextureAtlasSprite { index: entity_atlas.coord_to_index(0, sprite), ..default()},
            //TODO: change this to translate based on parent xyz
            transform: Transform::from_xyz(0., 0., 2.),
            ..default()
        },
        Collider(ENEMY_SIZE),
        LastAttacker(None),
        StoredPowerUps
        {
            power_ups: pu,
        },
        ChanceDropPWU(chance_drop_powerup),
        Aggro(None),
        SpawnEnemyWeaponTimer(Timer::from_seconds(enemy_attack_rate, TimerMode::Repeating)),//add a timer to spawn the enemy attack very 4 seconds
        EnemyRegenTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
        IsSpecial(is_special),
    )).id();
    if is_special {
        let special_entity = commands.spawn(SpriteBundle {
            texture: asset_server.load("Special_Enemy.png"),
            transform: Transform::from_xyz(0.0, 0.0, -0.5),
            ..default()
        }).id();
        commands.entity(enemy_entity).add_child(special_entity);
    }
}

pub fn remove_enemies(mut commands: Commands, enemies: Query<Entity, With<Enemy>>) {
    for e in enemies.iter() {
        commands.entity(e).despawn_recursive();
    }
}

// try to attack the player if they are aggroed
pub fn handle_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut query_enemies: Query<(Entity, &Health, &Transform, &mut SpawnEnemyWeaponTimer, &Aggro, &IsSpecial), (With<Enemy>, Without<Player>)>,
    mut player_query: Query<(&Transform, &mut Health, &StoredPowerUps, &PlayerShield), With<Player>>
) {
    for (enemy_entity, enemy_hp, enemy_transform, mut spawn_timer, aggro, is_special) in query_enemies.iter_mut() {
        if enemy_hp.current <= 0 || aggro.0 == None { continue; }
        spawn_timer.0.tick(time.delta());
        if spawn_timer.0.finished() {
            let attack_radius;
            if is_special.0 {
                attack_radius = SPECIAL_ATTACK_RADIUS_MOD;
            } else {
                attack_radius = 1.0;
            }
            let attack = commands.spawn((SpriteBundle {
                texture: asset_server.load("EnemyAttack01.png").into(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 5.0),
                    scale: Vec3::new(attack_radius, attack_radius, 1.0),
                    ..Default::default()
                },
                ..Default::default() },
                EnemyWeapon,
                Fade {current: 1.0, max: 1.0})).id();
            let enemy_entity = commands.get_entity(enemy_entity);
            if enemy_entity.is_none() {continue}
            let mut enemy_entity = enemy_entity.unwrap();
            enemy_entity.add_child(attack);
            for (player_transform, mut player_hp, player_power_ups, shield) in player_query.iter_mut() {
                let circle_radius;
                if is_special.0 {
                    circle_radius = CIRCLE_RADIUS * SPECIAL_ATTACK_RADIUS_MOD;
                } else {
                    circle_radius = CIRCLE_RADIUS;
                }
                if player_transform.translation.distance(enemy_transform.translation) < circle_radius {
                    // must check if damage reduction is greater than damage dealt, otherwise subtraction overflow or player will gain health
                    if shield.active { continue }
                    // Multiply enemy's damage value by player's default defense and DAMAGE_REDUCTION_UP ^ stacks of damage reduction
                    let dmg: u8 = (CIRCLE_DAMAGE as f32 * PLAYER_DEFAULT_DEF * DAMAGE_REDUCTION_UP.powf(player_power_ups.power_ups[PowerUpType::DamageReductionUp as usize] as f32)) as u8;
                    if dmg > 0
                    {
                        match player_hp.current.checked_sub(dmg) {
                            Some(v) => {
                                player_hp.current = v;
                            }
                            None => {
                                // player would die from hit
                                player_hp.current = 0;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn update_enemies(
    mut commands: Commands,
    mut enemies: Query<(Entity, &Health, &LastAttacker, &StoredPowerUps, &mut TextureAtlasSprite, &Transform, &EnemyCamp, &ChanceDropPWU), With<Enemy>>,
    mut player: Query<(&mut Stats, &Player)>,
    powerup_atlas: Res<PowerupAtlas>,
    mut camp_query: Query<(&Camp, &mut CampEnemies, &CampStatus), With<Camp>>,
) {
    for (e, hp, la, spu, mut sp, tf, ec_num, cdpu) in enemies.iter_mut() {
        if hp.current <= 0 {
            if cdpu.0{
                // drop powerups by cycling through the stored powerups of the enemy
                // and spawning the appropriate one
                for (index, &element) in spu.power_ups.iter().enumerate() {
                    if element == 1
                    {
                        commands.spawn((SpriteSheetBundle {
                            texture_atlas: powerup_atlas.handle.clone(),
                            sprite: TextureAtlasSprite { index: powerup_atlas.coord_to_index(0, index as i32), ..Default::default() },
                            transform: Transform {
                                translation: Vec3::new(tf.translation.x, tf.translation.y, 1.0),
                                ..Default::default()
                            },
                            ..Default::default()},
                                        PowerUp(unsafe { std::mem::transmute(index as u8) } ),
                        ));
                    }
                }
            }
            // decrement the enemy counter of the camp that this enemy is apart of
            for (camp_num, mut enemies_in_camp, camp_status) in camp_query.iter_mut() {
                if camp_num.0 == ec_num.0 {
                    enemies_in_camp.current_enemies -= 1;
                }

                // check if the camp is cleared and assign 5 points for clearing the camp
                if enemies_in_camp.current_enemies == 0 && camp_status.status == true{
                    for (mut stats, pl) in player.iter_mut() {
                        if pl.0 == la.0.expect("camp has no attacker") {
                            if Some(stats.score.checked_add(5)) != None {
                                stats.score += 5;
                            }
                            if Some (stats.camps_captured.checked_add(1)) != None {
                                stats.camps_captured += 1;
                            }
                            println!("5 points awarded for clearing camp {}", camp_num.0)
                        }
                    }
                }
            }

            // despawn the enemy and increment the score of the player who killed it
            commands.entity(e).despawn_recursive();
            for (mut stats, pl) in player.iter_mut() {
                if pl.0 == la.0.expect("died with no attacker?") {
                    if Some(stats.score.checked_add(1)) != None {
                        stats.score += 1;
                    }
                    if Some (stats.enemies_killed.checked_add(1)) != None {
                        stats.enemies_killed += 1
                    }
                }
            }
            continue;
        }
        let damage = hp.current as f32 / hp.max as f32;
        sp.color = Color::Rgba {red: 1.0, green: damage, blue: damage, alpha: 1.0};
    }
}

pub fn fixed_aggro(
    tick: Res<net::TickNum>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut enemies: Query<(Entity, &PosBuffer, &mut Aggro, &mut SpawnEnemyWeaponTimer, &IsSpecial), With<Enemy>>,
    players: Query<(&Player, &PosBuffer, &Health), Without<Enemy>>
) {
    for (enemy_entity, epb, mut aggro, mut wep_timer, is_special) in &mut enemies {
        let prev = epb.0.get(tick.0.wrapping_sub(1));
        let mut closest_player = None;
        let mut best_distance = f32::MAX;
        for (pl, ppb, hp) in &players {
            if hp.dead { continue }
            let dist = ppb.0.get(tick.0).distance(*prev);
            if dist < best_distance {
                best_distance = dist;
                closest_player = Some(pl);
            }
        }
        if best_distance > AGGRO_RANGE || closest_player.is_none() {
            if aggro.0.is_some() {
                //TODO show lost contact
            }
            aggro.0 = None;
        }
        else {
            if aggro.0.is_none() {
                let exlaim = commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("aggro.png").into(),
                        transform: Transform {
                            translation: Vec3::new(0.0, 32., 2.5),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Fade {
                        current: 2.0,
                        max: 2.0
                    }
                )).id();
                commands.entity(enemy_entity).push_children(&[exlaim]);
                let _ = aggro.0.insert(closest_player.unwrap().0);
                wep_timer.0.reset();
            }
        }
    }
}

pub fn fixed_move(
    tick: Res<net::TickNum>,
    mut enemies: Query<(&mut PosBuffer, &Aggro, &SpawnPosition), (With<Enemy>, Without<Player>)>,
    players: Query<(&Player, &PosBuffer), (With<Player>, Without<Enemy>)>,
    map: Res<WorldMap>
) {
    for (mut epb, aggro, spawn_pos) in &mut enemies {
        let prev = epb.0.get(tick.0.wrapping_sub(1));
        let mut next = prev.clone();

        'mov: {
            if aggro.0.is_none() {
                // move the enemy to their spawn position
                let displacement = spawn_pos.0 - *prev;
                if !(displacement.length() < CIRCLE_RADIUS) {
                    let posit = find_next(&map.biome_map, *prev, spawn_pos.0);
                    let movement = (posit - *prev).normalize() * ENEMY_SPEED;
                    next += movement;
                }
            } else {
                let aggro = aggro.0.unwrap();
                let mut ppbo = None;
                for (pl, ppb) in &players {
                    if pl.0 == aggro {
                        ppbo = Some(ppb);
                    }
                }
                if ppbo.is_none() { break 'mov }
                let player_pos = ppbo.unwrap().0.get(tick.0.wrapping_sub(1));

                let displacement = *player_pos - *prev;
                if !(displacement.length() < CIRCLE_RADIUS) {
                    let posit = find_next(&map.biome_map, *prev, *player_pos);
                    let movement = (posit - *prev).normalize() * ENEMY_SPEED;
                    next += movement;
                }
            }
        }
        epb.0.set(tick.0, next);
    }
}

pub fn find_next(
    map: &[[Biome; MAPSIZE]; MAPSIZE],
    s: Vec2,
    t: Vec2,
) -> Vec2 {
    let start = convert_vec(s);
    let target = convert_vec(t);

    // generate copy of map that uses integers instead of Biomes
    // declare:
    let mut u_map = [[0; MAPSIZE]; MAPSIZE];

    // edit:
    for x in 0..MAPSIZE {
        for y in 0..MAPSIZE {
            u_map[x][y] = match map[x][y] {
                Biome::Wall => 1,
                _ => 0,
            };
        }
    }

    // get path
    let path = a_star(&u_map, start, target);
    let pivots = find_pivot_points(path);


    // return next node if there's more than one tile to travel to get to player
    let mut go_to = target.clone();

    if pivots.len() > 1 {
        go_to = pivots[1].clone();
    }


    convert_back(go_to)
}

// fitting the tile values to the code below
pub fn convert_vec(vec: Vec2) -> V2 {
    let col = (vec.x + (TILESIZE * MAPSIZE / 2) as f32) as usize / TILESIZE;
    let row = (-vec.y + (TILESIZE * MAPSIZE / 2) as f32) as usize / TILESIZE;
    let v2 = V2 { x: col, y: row };
    v2
}

// converting back to the overworld values
pub fn convert_back(v2: V2) -> Vec2 {
    let x = (v2.x * TILESIZE) as f32 - (TILESIZE * MAPSIZE / 2) as f32;
    let y = -(v2.y as isize * TILESIZE as isize) as f32 + (TILESIZE * MAPSIZE / 2) as f32;
    Vec2 { x, y }
}

// structs for a_star
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct V2 {
    x: usize,
    y: usize,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node {
    position: V2,
    cost: usize,
}

// ordering for nodes in heap
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// check if position in map is valid
pub fn is_valid_position(map: &[[i32; MAPSIZE]]) -> Box<dyn Fn(V2) -> bool + '_> {
    Box::new(move |pos| pos.x < MAPSIZE && pos.y < map.len() && map[pos.y][pos.x] != 1)
}

// get path from hash table
pub fn reconstruct_path(from: HashMap<V2, V2>, mut current: V2) -> Vec<V2> {

    let mut path = vec![current];

    while from.contains_key(&current) {
        current = from[&current];
        path.push(current);
    }

    path.reverse();

    path
}

pub fn a_star(map: &[[i32; MAPSIZE]], start: V2, target: V2) -> Vec<V2> {
    let is_valid_position = is_valid_position(map);

    // pq for open list
    let mut open_list = BinaryHeap::new();

    // store the previous position for each position
    let mut from = HashMap::new();

    // store the cost of reaching each position from the start
    let mut g_score = HashMap::new();

    // start position in g_score
    g_score.insert(start, 0);
    // add start node to open list
    open_list.push(Node {
        position: start,
        cost: 0,
    });

    // A* algorithm
    while let Some(Node { position, ..}) = open_list.pop() {
        // reached the target, return the path
        if position == target {
            return reconstruct_path(from, target);
        }

        // check neighbors
        for dx in 0..3 {
            for dy in 0..3 {
                // skip current
                if dx == 1 && dy == 1 {
                    continue;
                }

                // find neighbor
                let neighbor = V2 {
                    x: (position.x as isize + dx as isize - 1) as usize,
                    y: (position.y as isize + dy as isize - 1) as usize,
                };

                // skip invalid neighbors
                if !is_valid_position(neighbor) {
                    continue;
                }

                let tentative_g_score = g_score.get(&position).unwrap_or(&0) + 1;
                // if new path is better/worse
                if !g_score.contains_key(&neighbor) || tentative_g_score < *g_score.get(&neighbor).unwrap() {

                    from.insert(neighbor, position);
                    g_score.insert(neighbor, tentative_g_score);

                    let priority = tentative_g_score + heuristic(neighbor, target);
                    open_list.push(Node {
                        position: neighbor,
                        cost: priority,
                    });
                }
            }
        }
    }

    // none
    Vec::new()
}

// Manhattan distance heuristic
fn heuristic(a: V2, b: V2) -> usize {
    ((a.x as isize - b.x as isize).abs() + (a.y as isize - b.y as isize).abs()) as usize
}

fn find_pivot_points(path: Vec<V2>) -> Vec<V2> {
    let mut pivots = Vec::new();

    if path.len() < 3 {
        return path; // not enough points to determine pivots
    }

    for i in 1..path.len() - 1 {
        let prev_slope = (path[i].y as f32 - path[i - 1].y as f32) / (path[i].x as f32 - path[i - 1].x as f32);
        let next_slope = (path[i + 1].y as f32 - path[i].y as f32) / (path[i + 1].x as f32 - path[i].x as f32);

        // Compare slopes
        if prev_slope != next_slope {
            pivots.push(path[i]);
        }
    }

    // Add the last point as a pivot
    pivots.push(path[path.len() - 1]);

    pivots
}

// Enemy regen health system
pub fn enemy_regen_health(
    tick: Res<net::TickNum>,
    time: Res<Time>,
    mut enemies: Query<(&mut PosBuffer, &mut Health, &mut TextureAtlasSprite, &Aggro, &SpawnPosition, &mut EnemyRegenTimer), With<Enemy>>,
) {
    for (epb, mut hp, mut sprite, aggro, sp, mut timer) in enemies.iter_mut() {
        let prev = epb.0.get(tick.0.wrapping_sub(1));
        if aggro.0.is_none() {
            // move the enemy to their spawn position
            let displacement = sp.0 - *prev;
            if displacement.length() < CIRCLE_RADIUS {
                timer.0.tick(time.delta());
                if timer.0.finished() {
                    if hp.current < hp.max {
                        hp.current += 10;
                        let damage = hp.current as f32 / hp.max as f32;
                        sprite.color = Color::Rgba {red: 1.0, green: damage, blue: damage, alpha: 1.0};
                    }
                }
            }
        }
    }
}
/// Resolve enemy wall collisions
pub fn fixed_resolve(
    mut enemies: Query<(&mut PosBuffer, &Collider), With<Enemy>>,
    map: Res<WorldMap>,
    tick: Res<TickNum>,
) {
    for (enemy_pos_buffer, collider) in &mut enemies {
        let pos_buffer = enemy_pos_buffer.into_inner();
        let pos = pos_buffer.0.get(tick.0);
        let mut pos3 = Vec3::new(pos.x, pos.y, 0.0);
        pos3 = correct_wall_collisions(&pos3, &collider.0, &map.biome_map);
        pos_buffer.0.set(tick.0, pos3.xy());
    }
}

pub fn handle_packet(
    mut enemy_reader: EventReader<net::packets::EnemyTickEvent>,
    mut enemy_query: Query<(&Enemy, &mut PosBuffer)>
) {
    //TODO if you receive info that your predicted local position is wrong, it needs to be corrected
    for ev in enemy_reader.iter() {
        // TODO this is slow but i have no idea how to make the borrow checker okay
        //   with the idea of an array of player PosBuffer references
        for (en, mut pb) in &mut enemy_query {
            if en.0 == ev.tick.id {
                pb.0.set(ev.seq_num, ev.tick.pos);
            }
        }
    }
}
