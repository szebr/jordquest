use bevy::prelude::*;
use crate::AppState;
use crate::game::enemy;
use crate::Atlas;

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

    enemy::spawn_enemy(&mut commands, &entity_atlas, 1, Vec2{x: -100., y: -100.}, 5);
    enemy::spawn_enemy(&mut commands, &entity_atlas, 1, Vec2{x: -100., y: 100.}, 4);
    
}