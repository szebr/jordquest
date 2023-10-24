// The idea behind this file is to hold components which will be used across gameplay files
// Components which are only used locally can be left inside a more localized file.
use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub current: u8,
    pub max: u8,
}

#[derive(Component)]
pub struct Enemy(pub u8);  // holds id

#[derive(Component)]
pub struct Player(pub u8);  // holds id