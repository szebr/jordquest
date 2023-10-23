use bevy::prelude::*;
use crate::{AppState, net};
use crate::game::enemy;
use crate::Atlas;
use std::time::Duration;

use super::enemy::spawn_new_enemy;

pub struct CampPlugin;

#[derive(Component)]
pub struct Camp(pub Vec2);

impl Plugin for CampPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(OnEnter(AppState::Game), setup);
    }
}

pub fn setup(
    mut commands: Commands,
    entity_atlas:Res<Atlas>,
) {

    spawn_new_enemy(commands, entity_atlas, 1);
    
    
}