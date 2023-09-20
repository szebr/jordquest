use bevy::prelude::*;

//TODO derive default?
#[derive(Resource)]
pub struct InputState {
    pub key_binds: KeyBinds,
    pub mouse_binds: MouseBinds,
    pub movement: Vec2,
    pub attack: bool
}

impl InputState {
    pub fn new_with_bindings(key_binds: KeyBinds, mouse_binds: MouseBinds) -> InputState {
        InputState {
            key_binds,
            mouse_binds,
            movement: Vec2 { x:0., y:0. },
            attack: false
        }
    }
}

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



// BRANCHLESS!!!!!!!
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

pub fn update_key_state(mut input_state: ResMut<InputState>, keyboard_input: Res<Input<KeyCode>>) {
    let mut mv: usize = keyboard_input.pressed(input_state.key_binds.up) as usize * 0b0001;
    mv |= keyboard_input.pressed(input_state.key_binds.down) as usize * 0b0010;
    mv |= keyboard_input.pressed(input_state.key_binds.left) as usize * 0b0100;
    mv |= keyboard_input.pressed(input_state.key_binds.right) as usize * 0b1000;
    input_state.movement = MOVE_VECTORS[mv];
}

pub fn update_mouse_state(mut input_state: ResMut<InputState>, mouse_input: Res<Input<MouseButton>>) {
    input_state.attack = mouse_input.pressed(input_state.mouse_binds.attack);
}