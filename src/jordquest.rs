use bevy::prelude::*;
use crate::input::*;
use crate::player;
use crate::enemy;

pub struct JordQuestPlugin;

#[derive(Resource)]
pub struct TickNum(pub u16);

pub enum AbilityType {
    Bite
}

//TODO tick rollover is not even REMOTELY addressed
pub struct Ability {
    pub ready_at: u16,
    pub duration: u16,
    pub ability_type: AbilityType
}

#[derive(Component)]
pub struct Character {
    pub health: f32,  // TODO what type should this be?
    pub speed: f32,
    pub abilities: Vec<Ability>
}

const TICKRATE: u8 = 30;

impl Plugin for JordQuestPlugin {
    fn build(&self, app: &mut App) {
        let key_binds = KeyBinds::new();
        let mouse_binds = MouseBinds::new();
        let input_state = InputState::new_with_bindings(key_binds, mouse_binds);
        app.insert_resource(input_state)
            .insert_resource(FixedTime::new_from_secs(1. / (TICKRATE as f32)))
            .insert_resource(TickNum { 0: 0 })
            .add_systems(Startup, (setup, player::spawn, enemy::spawn))
            // FixedUpdate runs every simulation tick
            .add_systems(FixedUpdate, increment_tick)
            // Update runs every drawing frame
            .add_systems(Update, (
                update_key_state,
                update_mouse_state,
                player::attack.after(update_mouse_state),
                player::movement.after(update_key_state),
                player::update_sprite,
                enemy::movement,
                ));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
}

fn increment_tick(
    //mut last_time: Local<f32>,
    //time: Res<Time>,
    //fixed_time: Res<FixedTime>,
    mut tick_num: ResMut<TickNum>) {
    tick_num.0 += 1;
}

