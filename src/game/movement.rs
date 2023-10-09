use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::game::map::{Biome, get_surrounding_tiles, WorldMap};
use crate::game::player::LocalPlayer;

/// Just a vec2 that describes the size of a bounding box around the entity
#[derive(Component)]
pub struct Collider(pub Vec2);

/// Move a movable entity. Takes in a reference to that entity's , the Time resource, a query of possible colliding entities, a direction and a speed.
/// Checks for collision against another entity and then the map before finishing movement.
pub fn move_unit(
    pos: &mut Transform,
    this_collider: &Collider,
    map: &Res<WorldMap>,
    time: &Res<Time>,
    colliders: &Query<(&Transform, &Collider), Without<LocalPlayer>>,
    dir: &Vec2,
    speed: f32,
) {
    let mut can_move = true;
    let dir = dir.normalize_or_zero();
    let new_pos: Vec3 = Vec3 {
        x: pos.translation.x + dir.x * speed * time.delta_seconds(),
        y: pos.translation.y + dir.y * speed * time.delta_seconds(),
        z: 0.0,
    };

    // Check collision against other entities
    // TODO: break this out into another function?
    for (transform, collider) in colliders.iter() {
        // TODO: This seems like a terrible hack. Is there a better way to check if these references are the same?
        if (this_collider as *const Collider) == (collider as *const Collider) {
            continue;
        }
        if let Some(Collision) = collide(new_pos, this_collider.0, transform.translation, collider.0) {
            // TODO: update movement vector to account for the collision?
            can_move = false;
            // if we've found out we can't move, we can break for now
            // if we end up trying to update movement in here, will have to not break here in case we collide in multiple places?
            break;
        } else {
            // can move
        }
    }

    // check collision against map tiles
    // TODO: Need to do some math to figure out where the entity is relative to the tile
    // TODO: This crashes if you try to move outside of the map
    let nearby = get_surrounding_tiles(&new_pos, &map.biome_map);
    if nearby[1][1] == Biome::Wall {
        can_move = false;
    }

    if can_move {
        pos.translation.x = new_pos.x;
        pos.translation.y = new_pos.y;
    }
}