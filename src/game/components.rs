// The idea behind this file is to hold components which will be used across gameplay files
// Components which are only used locally can be left inside a more localized file.
use bevy::prelude::*;
use core::fmt::Debug;

#[derive(Component)]
pub struct Health {
    pub current: u8,
    pub max: u8,
    pub dead: bool
}

#[derive(Component)]
pub struct Fade {
    pub current: f32,
    pub max: f32
}

pub const NUM_POWERUPS: usize = 5;
pub const DAMAGE_DEALT_UP: u8 = 10;
pub const DAMAGE_REDUCTION_UP: f32 = 0.9;
pub const MEAT_VALUE: u8 = 30;
pub const ATTACK_SPEED_UP: f32 = 1.1;
pub const MOVEMENT_SPEED_UP: u8 = 15;
pub const CHEST_CONTENTS: usize = 5;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PowerUpType {
    Meat = 0,
    DamageDealtUp,
    DamageReductionUp,
    AttackSpeedUp,
    MovementSpeedUp,
}

#[derive(Component, Eq, PartialEq, Clone)]
pub struct StoredPowerUps{
    pub power_ups: [u8; NUM_POWERUPS],
    // 0: MaxHPUp, 1: DamageReductionUp, 2: DamageDealtUp, 3: AttackSpeedUp, 4: MovementSpeedUp
}

#[derive(Component)]
pub struct ChanceDropPWU(pub bool);

#[derive(Component)]
pub struct PowerUp(pub PowerUpType);

/// Just a vec2 that describes the size of a bounding box around the entity
#[derive(Component)]
pub struct Collider(pub Vec2);

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct PowerupDisplayText(pub u8);

#[derive(Component, Clone)]
pub struct Stats{
    pub score: u8,
    pub enemies_killed: u8,
    pub players_killed: u8,
    pub camps_captured: u8,
    pub deaths: u8,
    pub kd_ratio: f32,
}

#[derive(Component)]
pub struct StatDisplayText(pub u8);

#[derive(Component)]
pub struct Enemy(pub u8);  // holds id

#[derive(Component)]
pub struct Player(pub u8);  // holds id

// camp stuff
#[derive(Component)]
pub struct Camp(pub u8); // holds id


#[derive(Component)]
pub struct Grade(pub u8);

#[derive(Component)]
pub struct CampEnemies{
    pub max_enemies: u8, 
    pub current_enemies: u8,
}

#[derive(Component)]
pub struct CampStatus(pub bool); // true if camp is captured, false if not

#[derive(Component)]
pub struct EnemyCamp(pub u8); // holds id of enemy's parent camp

#[derive(Component)]
pub struct ItemChest{
    pub id: u8,
    pub contents: [u8; CHEST_CONTENTS],
}


