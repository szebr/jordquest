use bevy::prelude::*;

use crate::menus::components::*;
use crate::AppState;
use crate::menus::NetworkAddresses;

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
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Text, &mut HostPortInput)>,
) {
    let mut new_char = None;
    for event in char_events.iter() {
        new_char = Some(event.char);
    }

    if let Some(new_char) = new_char {
        for (mut text, mut host_port_input) in query.iter_mut() {
            if new_char != '\u{8}' {
                text.sections[0].value.push(new_char);
                host_port_input.port.push(new_char);
            }
            //println!("Current port value: {}", host_port_input.port);
            if keyboard_input.just_pressed(KeyCode::Back) {
                if !host_port_input.port.is_empty() {
                    //println!("Current port value: {}", host_port_input.port);
                    //println!("Before: {:?}", text.sections[0].value);
                    text.sections[0].value.pop();
                    //println!("After: {:?}", text.sections[0].value);
                    host_port_input.port.pop();
                }
            }
        }
    }
}

pub fn save_host_input(
    mut is_host: ResMut<crate::net::IsHost>,
    mut net_address: ResMut<NetworkAddresses>,
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
                    net_address.host_port = host_port_input.port.clone();
                    is_host.0 = true;
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
    keyboard_input: Res<Input<KeyCode>>,
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
            for (mut text, mut host_port_input) in query.iter_mut() {
                if new_char != '\u{8}' {
                    text.sections[0].value.push(new_char);
                    host_port_input.port.push(new_char);
                }
                //println!("Current port value: {}", host_port_input.port);
                if keyboard_input.just_pressed(KeyCode::Back) {
                    if !host_port_input.port.is_empty() {
                        //println!("Current port value: {}", host_port_input.port);
                        //println!("Before: {:?}", text.sections[0].value);
                        text.sections[0].value.pop();
                        //println!("After: {:?}", text.sections[0].value);
                        host_port_input.port.pop();
                    }
                }
            }
        }
    }
}

pub fn update_join_ip_input(
    mut char_events: EventReader<ReceivedCharacter>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Text, &mut JoinIPInput)>,
    mut switch_query: Query<&Switch>,
) {
    let mut active = false;
    for mut switch in switch_query.iter_mut() {
        active = switch.port;
    }
    if !active {
        let mut new_char = None;
        for event in char_events.iter() {
            new_char = Some(event.char);
        }

        if let Some(new_char) = new_char {
            for (mut text, mut join_ip_input) in query.iter_mut() {
                if new_char != '\u{8}' {
                    text.sections[0].value.push(new_char);
                    join_ip_input.ip.push(new_char);
                }
                //println!("Current port value: {}", host_port_input.port);
                if keyboard_input.just_pressed(KeyCode::Back) {
                    if !join_ip_input.ip.is_empty() {
                        //println!("Current port value: {}", host_port_input.port);
                        //println!("Before: {:?}", text.sections[0].value);
                        text.sections[0].value.pop();
                        //println!("After: {:?}", text.sections[0].value);
                        join_ip_input.ip.pop();
                    }
                }
            }
        }
    }
}

pub fn save_join_input(
    mut is_host: ResMut<crate::net::IsHost>,
    mut net_address: ResMut<NetworkAddresses>,
    join_port_query: Query<&JoinPortInput>,
    join_ip_query: Query<&JoinIPInput>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<JoinSaveBut>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                for join_port_input in join_port_query.iter() {
                    net_address.client_port = join_port_input.port.clone();
                }
                for join_ip_input in join_ip_query.iter() {
                    net_address.ip = join_ip_input.ip.clone();
                }
                is_host.0 = false;
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