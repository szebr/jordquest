// The idea behind this file is to hold components which will be used across gameplay files
// Components which are only used locally can be left inside a more localized file.
use bevy::prelude::*;
use crate::enemy;
use core::fmt::Debug;

#[derive(Component)]
pub struct Health {
    pub current: u8,
    pub max: u8,
}

#[derive(Debug)]
pub enum PowerUpType {
    DamageDealtUp,
    DamageReductionUp,
    MaxHPUp,
    AttackSpeedUp,
    MovementSpeedUp,
}

#[derive(Component)]
pub struct StoredPowerUps{
    pub power_ups: [u8; 5],
    // 0: DamageDealtUp, 1: DamageReductionUp, 2: MaxHPUp, 3: AttackSpeedUp, 4: MovementSpeedUp
}

#[derive(Component)]
pub struct PowerUp(pub PowerUpType);

/// Just a vec2 that describes the size of a bounding box around the entity
#[derive(Component)]
pub struct Collider(pub Vec2);

#[derive(Component)]
pub struct Score {
    pub current_score: u8,
}

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct Enemy(pub u8);  // holds id

#[derive(Component)]
pub struct Player(pub u8);  // holds id