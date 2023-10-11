use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::game::map::{Biome, get_surrounding_tiles, WorldMap};
use crate::game::player::LocalPlayer;

/// Just a vec2 that describes the size of a bounding box around the entity
#[derive(Component)]
pub struct Collider(pub Vec2);