use bevy::prelude::*;

/// Just a vec2 that describes the size of a bounding box around the entity
#[derive(Component)]
pub struct Collider(pub Vec2);