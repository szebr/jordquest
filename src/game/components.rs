use bevy::prelude::*;

pub const TITLE: &str = "Game";
pub const WIN_W: f32 = 1280.;
pub const WIN_H: f32 = 720.;

pub const PLAYER_SPEED: f32 = 500.;

pub const TILE_SIZE: f32 = 100.;
pub const LEVEL_W: f32 = 1920.;
pub const LEVEL_H: f32 = 1080.;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Brick;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec2,
}
