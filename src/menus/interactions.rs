use bevy::prelude::*;

use crate::menus::components::*;
use crate::AppState;
use crate::menus::NetworkAddresses;

pub fn interact_with_button<B: ButtonTypeTrait>(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<B::Marker>)>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = Color::GRAY.into();
                app_state_next_state.set(B::app_state());
            }
            Interaction::Hovered => {
                *background_color = Color::GRAY.into();
            }
            Interaction::None => {
                *background_color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

pub fn interact_with_host_button(
    button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<HostButton>)>,
    app_state_next_state: ResMut<NextState<AppState>>,
) {
    interact_with_button::<HostButtonType>(button_query, app_state_next_state);
}

pub fn interact_with_join_button(
    button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<JoinButton>)>,
    app_state_next_state: ResMut<NextState<AppState>>,
) {
    interact_with_button::<JoinButtonType>(button_query, app_state_next_state);
}

pub fn interact_with_back_button(
    button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<BackToMainMenu>)>,
    app_state_next_state: ResMut<NextState<AppState>>,
) {
    interact_with_button::<BackButtonType>(button_query, app_state_next_state);
}

pub fn interact_with_credits_button(
    button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<CreditsButton>)>,
    app_state_next_state: ResMut<NextState<AppState>>,
) {
    interact_with_button::<CreditsButtonType>(button_query, app_state_next_state);
}

pub fn update_input<T: InputType>(
    mut char_events: EventReader<ReceivedCharacter>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Text, &mut T)>,
    switch_query: Option<Query<&Switch>>,
) {
    if let Some(mut switch_query) = switch_query {
        let mut active = false;
        for switch in switch_query.iter_mut() {
            active = T::is_active(switch);
        }
        if !T::is_valid(active) {
            return;
        }
    }
    let mut new_char = None;
    for event in char_events.iter() {
        new_char = Some(event.char);
    }

    if let Some(new_char) = new_char {
        for (mut text, mut input_type) in query.iter_mut() {
            if new_char != '\u{8}' {
                text.sections[0].value.push(new_char);
                input_type.push_char(new_char);
            }
            if keyboard_input.just_pressed(KeyCode::Back) {
                if !input_type.is_empty() {
                    text.sections[0].value.pop();
                    input_type.pop_char();
                }
            }
        }
    }
}
pub fn update_host_input(
    char_events: EventReader<ReceivedCharacter>,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&mut Text, &mut HostPortInput)>,
) {
    update_input::<HostPortInput>(char_events, keyboard_input, query, None);
}

pub fn update_join_port_input(
    char_events: EventReader<ReceivedCharacter>,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&mut Text, &mut JoinPortInput)>,
    switch_query: Query<&Switch>,
) {
    update_input::<JoinPortInput>(char_events, keyboard_input, query, Some(switch_query));
}

pub fn update_join_ip_input(
    char_events: EventReader<ReceivedCharacter>,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&mut Text, &mut JoinIPInput)>,
    switch_query: Query<&Switch>,
) {
    update_input::<JoinIPInput>(char_events, keyboard_input, query, Some(switch_query));
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