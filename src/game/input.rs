use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::input::mouse::MouseButtonInput;
use crate::{player, net};
use crate::game::player::LocalPlayer;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, handle_mouse_button_events)
            .add_systems(FixedUpdate, update_movement_vector.before(player::fixed));
    }
}

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
const DIAG: f32 = std::f32::consts::SQRT_2 / 2.;
const MOVE_VECTORS: [Vec2; 16] = [
    Vec2 { x:0., y:0. },  // 0000
    Vec2 { x:0., y:1. },  // 0001
    Vec2 { x:0., y:-1. }, // 0010
    Vec2 { x:0., y:0. },  // 0011
    Vec2 { x:-1., y:0. },  // 0100
    Vec2 { x:-DIAG, y:DIAG },  // 0101
    Vec2 { x:-DIAG, y:-DIAG },  // 0110
    Vec2 { x:-1., y:0. },  // 0111
    Vec2 { x:1., y:0. },  // 1000
    Vec2 { x:DIAG, y:DIAG },  // 1001
    Vec2 { x:DIAG, y:-DIAG },  // 1010
    Vec2 { x:1., y:0. },  // 1011
    Vec2 { x:0., y:0. },  // 1100
    Vec2 { x:0., y:1. },  // 1101
    Vec2 { x:0., y:-1. },  // 1110
    Vec2 { x:0., y:0. },  // 1111
];

// on FixedUpdate schedule before player::fixed
pub fn update_movement_vector(
    keyboard_input: Res<Input<KeyCode>>,
    tick: Res<net::TickNum>,
    player_id: Res<player::PlayerID>,
    mut players: Query<&mut player::Player>,
    key_binds: Res<KeyBinds>
) {
    let mut mv: usize = keyboard_input.pressed(key_binds.up) as usize * 0b0001;
    mv |= keyboard_input.pressed(key_binds.down) as usize * 0b0010;
    mv |= keyboard_input.pressed(key_binds.left) as usize * 0b0100;
    mv |= keyboard_input.pressed(key_binds.right) as usize * 0b1000;
    for mut pl in &mut players {
        if pl.id == player_id.0 {
            //TODO might be better to mutate in place
            let mut pt = pl.get(tick.0 - 1).clone();
            pt.input.movement = MOVE_VECTORS[mv];
            pl.set(tick.0, pt);
        }
    }
}

// on Update schedule
pub fn handle_mouse_button_events(
    mut er: EventReader<MouseButtonInput>,
    mouse_binds: Res<MouseBinds>,
    tick: Res<net::TickNum>,
    mut players: Query<&mut player::Player, With<LocalPlayer>>,
) {
    for mut pl in &mut players {
        for e in er.iter() {
            if e.button == mouse_binds.attack {
                //TODO might be better to mutate in place
                let mut pt = pl.get(tick.0 - 1).clone();
                pt.input.attack = e.state == ButtonState::Pressed;
                pl.set(tick.0, pt);
                // TODO if you click and release within one tick, the input will be missed!!
            }
        }
    }
}

pub fn startup(mut commands: Commands) {
    commands.insert_resource(KeyBinds::new());
    commands.insert_resource(MouseBinds::new());
}