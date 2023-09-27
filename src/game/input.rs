use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::input::mouse::MouseButtonInput;
use crate::player;

#[derive(Component, Resource, Default, Clone, Copy)]
pub struct InputState {
    pub movement: Vec2,
    pub attack: bool
}

// NET STRUCT
pub struct InputStateBuffer {
    buffer: [InputState; player::MAX_PLAYERS],
    count: usize
}

#[derive(Resource)]
pub struct KeyBinds {
    up: KeyCode,
    down: KeyCode,
    left: KeyCode,
    right: KeyCode
}

impl KeyBinds {
    // later on, we should have a constructor that reads bindings from a file
    pub fn new() -> KeyBinds {
        KeyBinds {
            up: KeyCode::W,
            down: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D
        }
    }
}

#[derive(Resource)]
pub struct MouseBinds {
    attack: MouseButton
}

impl MouseBinds {
    // later on, we should have a constructor that reads bindings from a file
    pub fn new() -> MouseBinds {
        MouseBinds {
            attack: MouseButton::Left
        }
    }
}



// this lookup table prevents square root math at runtime for movement
// each cardinal direction is given a bit and or'd together to create the index
const MOVE_VECTORS: [Vec2; 16] = [
    Vec2 { x:0., y:0. },  // 0000
    Vec2 { x:0., y:1. },  // 0001
    Vec2 { x:0., y:-1. }, // 0010
    Vec2 { x:0., y:0. },  // 0011
    Vec2 { x:-1., y:0. },  // 0100
    Vec2 { x:-0.707, y:0.707 },  // 0101
    Vec2 { x:-0.707, y:-0.707 },  // 0110
    Vec2 { x:-1., y:0. },  // 0111
    Vec2 { x:1., y:0. },  // 1000
    Vec2 { x:0.707, y:0.707 },  // 1001
    Vec2 { x:0.707, y:-0.707 },  // 1010
    Vec2 { x:1., y:0. },  // 1011
    Vec2 { x:0., y:0. },  // 1100
    Vec2 { x:0., y:1. },  // 1101
    Vec2 { x:0., y:-1. },  // 1110
    Vec2 { x:0., y:0. },  // 1111
];

// on FixedUpdate schedule just before tick finishes
pub fn update_movement_vector(
    keyboard_input: Res<Input<KeyCode>>,
    player_id: Res<player::PlayerID>,
    mut players: Query<(&player::Player, &mut InputState)>,
    key_binds: Res<KeyBinds>
) {
    let mut mv: usize = keyboard_input.pressed(key_binds.up) as usize * 0b0001;
    mv |= keyboard_input.pressed(key_binds.down) as usize * 0b0010;
    mv |= keyboard_input.pressed(key_binds.left) as usize * 0b0100;
    mv |= keyboard_input.pressed(key_binds.right) as usize * 0b1000;
    for (pl, mut is) in &mut players {
        if pl.id == player_id.0 {
            is.movement = MOVE_VECTORS[mv];
        }
    }
}

// on Update schedule
pub fn handle_mouse_button_events(
    mut er: EventReader<MouseButtonInput>,
    mouse_binds: Res<MouseBinds>,
    mut input_state: ResMut<InputState>
) {
    for e in er.iter() {
        if e.button == mouse_binds.attack {
            input_state.attack = e.state == ButtonState::Pressed;
            // TODO if you click and release within one tick, the input will be missed!!
        }

    }
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(KeyBinds::new());
    commands.insert_resource(MouseBinds::new());
}