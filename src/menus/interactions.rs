use bevy::prelude::*;

use crate::menus::components::*;
use crate::AppState;

pub fn interact_with_host_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<HostButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = Color::GRAY.into();
                app_state_next_state.set(AppState::Hosting);
            }
            Interaction::Hovered => {
                *background_color = Color::GRAY.into();
            }
            Interaction::None => {
                *background_color = Color:: rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

pub fn interact_with_join_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<JoinButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = Color::GRAY.into();
                app_state_next_state.set(AppState::Joining);
            }
            Interaction::Hovered => {
                *background_color = Color::GRAY.into();
            }
            Interaction::None => {
                *background_color = Color:: rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}



pub fn update_host_input(
    mut char_events: EventReader<ReceivedCharacter>,
    mut query: Query<(&mut Text, &mut HostPortInput)>,
) {
    let mut new_char = None;
    for event in char_events.iter() {
        new_char = Some(event.char);
    }

    if let Some(new_char) = new_char {
        for (mut text, mut host_port_input) in query.iter_mut() {
            text.sections[0].value.push(new_char);
            host_port_input.port.push(new_char);
            //println!("Current port value: {}", host_port_input.port);
        }
    }
}

pub fn save_host_input(
    mut net_address_query: Query<&mut NetworkAdresses>,
    host_port_query: Query<&HostPortInput>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<HostPortSaveBut>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                for host_port_input in host_port_query.iter() {
                    for mut net_address in net_address_query.iter_mut() {
                        net_address.host = host_port_input.port.clone();
                        println!("Current port value: {}", net_address.host);
                    }
                }
                app_state_next_state.set(AppState::Game);
            }
            Interaction::Hovered => {
                *background_color = Color::GRAY.into();
            }
            Interaction::None => {
                *background_color = Color:: rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

pub fn switch_input_joinpage(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Switch>),
    >,
    mut switch_query: Query<&mut Switch>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                for mut switch in switch_query.iter_mut() {
                    if switch.port {
                        switch.port = false;
                    }else {
                        switch.port = true;
                    }
                }
            }
            Interaction::Hovered => {
                *background_color = Color::GRAY.into();
            }
            Interaction::None => {
                *background_color = Color:: rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

pub fn update_join_port_input(
    mut char_events: EventReader<ReceivedCharacter>,
    mut query: Query<(&mut Text, &mut JoinPortInput)>,
    mut switch_query: Query<&Switch>,
) {
    let mut active = false;
    for mut switch in switch_query.iter_mut() {
        active = switch.port;
    }
    if active {
        let mut new_char = None;
        for event in char_events.iter() {
            new_char = Some(event.char);
        }

        if let Some(new_char) = new_char {
            for (mut text, mut join_port_input) in query.iter_mut() {
                text.sections[0].value.push(new_char);
                join_port_input.port.push(new_char);
                println!("Current port value: {}", join_port_input.port);
            }
        }
    }
}


pub fn interact_with_back_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackToMainMenu>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = Color::GRAY.into();
                app_state_next_state.set(AppState::MainMenu);
            }
            Interaction::Hovered => {
                *background_color = Color::GRAY.into();
            }
            Interaction::None => {
                *background_color = Color:: rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

pub fn interact_with_credits_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<CreditsButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = Color::GRAY.into();
                app_state_next_state.set(AppState::Credits);
            }
            Interaction::Hovered => {
                *background_color = Color::GRAY.into();
            }
            Interaction::None => {
                *background_color = Color:: rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}