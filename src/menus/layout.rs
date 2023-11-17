use crate::game::camera::SpatialCameraBundle;
use crate::game::components::*;
use crate::game::ROUND_TIME;
use crate::menus::components::*;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::Deref;
use bevy::prelude::DerefMut;
use bevy::prelude::Timer;
use bevy::prelude::*;
use crate::Atlas;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
pub const PADDING: f32 = 20.0;

#[derive(Component, Deref, DerefMut)]
pub struct PopupTimer(Timer);

#[derive(Component)]
pub struct GameTimer {
    remaining_time: f32, // time in seconds
}

pub fn show_popup(time: Res<Time>, mut popup: Query<(&mut PopupTimer, &mut Transform)>) {
    for (mut timer, mut transform) in popup.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.z += 10.;
        }
    }
}

fn spawn_title(parent: &mut EntityCommands, font: &Handle<Font>, title: &str) {
    let title_node = parent
        .commands()
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Px(600.0),
                height: Val::Px(80.0),
                margin: UiRect {
                    left: Val::Px(8.),
                    right: Val::Px(8.),
                    top: Val::Px(0.0),
                    bottom: Val::Px(60.0),
                },
                ..default()
            },
            background_color: BackgroundColor::from(Color::WHITE),
            ..default()
        })
        .id();

    let text = parent
        .commands()
        .spawn(TextBundle::from_section(
            title,
            TextStyle {
                font: font.clone(),
                font_size: 64.0,
                color: Color::RED,
            },
        ))
        .id();
    parent.commands().entity(title_node).add_child(text);
    parent.add_child(title_node);
}

fn spawn_flex_column<T: Bundle>(commands: &mut Commands, page: T) -> Entity {
    return spawn_flex_column_colored(commands, page, Color::WHITE);
}

fn spawn_flex_column_colored<T: Bundle>(
    commands: &mut Commands,
    page: T,
    color: Color
) -> Entity {
    let menu = commands.spawn((
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: BackgroundColor::from(color),
            ..default()
        },
        page,
    ));
    return menu.id();
}

fn spawn_button<T: Bundle>(
    parent: &mut EntityCommands,
    font: &Handle<Font>,
    button: T,
    title: &str,
) {
    let button = parent
        .commands()
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(500.0),
                    height: Val::Px(80.0),
                    margin: UiRect {
                        left: Val::Px(8.),
                        right: Val::Px(8.),
                        top: Val::Px(0.0),
                        bottom: Val::Px(8.0),
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            },
            button,
        ))
        .id();
    let text = parent
        .commands()
        .spawn(
            TextBundle::from_section(
                title,
                TextStyle {
                    font: font.clone(),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::Center),
        )
        .id();
    parent.commands().entity(button).add_child(text);
    parent.add_child(button);
}

fn spawn_input<T: Bundle, U: Bundle>(
    parent: &mut EntityCommands,
    font: &Handle<Font>,
    button: T,
    input: U,
    title: &str,
) {
    let input_node = parent
        .commands()
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(500.0),
                    height: Val::Px(80.0),
                    margin: UiRect {
                        left: Val::Px(8.),
                        right: Val::Px(8.),
                        top: Val::Px(0.0),
                        bottom: Val::Px(8.0),
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            },
            Switch {
                host_port: false,
                port: false,
                ip: false,
                host: false,
                num_camps: false,
                num_chests: false,
                enemy_per_camp: false,
                map_seed: false,
                eid_percentage: false,
            },
            button,
        ))
        .id();

    let text = parent
        .commands()
        .spawn((
            TextBundle::from_section(
                title,
                TextStyle {
                    font: font.clone(),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::Center),
            input,
        ))
        .id();
    parent.commands().entity(input_node).add_child(text);
    parent.add_child(input_node);
}

fn spawn_text<T: Bundle, U: Bundle>(
    parent: &mut EntityCommands,
    font: &Handle<Font>,
    button: T,
    input: U,
    title: &str,
) {
    let input_node = parent
        .commands()
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(500.0),
                    height: Val::Px(80.0),
                    margin: UiRect {
                        left: Val::Px(8.),
                        right: Val::Px(8.),
                        top: Val::Px(0.0),
                        bottom: Val::Px(8.0),
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            },
            Switch {
                host_port: false,
                port: false,
                ip: false,
                host: false,
                num_camps: false,
                num_chests: false,
                enemy_per_camp: false,
                map_seed: false,
                eid_percentage: false,
            },
            button,
        ))
        .id();

    let text = parent
        .commands()
        .spawn((
            TextBundle::from_section(
                title,
                TextStyle {
                    font: font.clone(),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::Center),
            input,
        ))
        .id();
    parent.commands().entity(input_node).add_child(text);
    parent.add_child(input_node);
}
pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let main_menu_id = spawn_flex_column(&mut commands, MainMenu);
    let mut main_menu = commands.entity(main_menu_id);
    spawn_title(&mut main_menu, &font, "JORDQUEST");
    spawn_button(&mut main_menu, &font, HostButton, "Host");
    spawn_button(&mut main_menu, &font, JoinButton, "Join");
    spawn_button(&mut main_menu, &font, ControlsButton, "Controls");
    spawn_button(&mut main_menu, &font, CreditsButton, "Credits");
}

fn add_credits_slide(
    commands: &mut Commands,
    asset_server: &AssetServer,
    filename: &str,
    index: usize,
) {
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load(filename),
                transform: Transform::from_xyz(0., 0., -1.0 + (0.1 * index as f32))
                    .with_scale(Vec3::new(0.5, 0.5, 1.0)),
                ..default()
            },
            Popup,
        ))
        .insert(PopupTimer(Timer::from_seconds(
            2.0 * index as f32,
            TimerMode::Once,
        )));
}

pub fn spawn_credits_page(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut cameras: Query<&mut Transform, With<SpatialCameraBundle>>,
) {
    for mut tf in cameras.iter_mut() {
        let translation = Vec3::new(0.0, 0.0, 1.0);
        tf.translation = translation;
    }
    add_credits_slide(&mut commands, &asset_server, "brendan_credits_slide.png", 0);
    add_credits_slide(&mut commands, &asset_server, "CreditAlexLampe.png", 1);
    add_credits_slide(&mut commands, &asset_server, "CreditGarrettDiCenzo.png", 2);
    add_credits_slide(&mut commands, &asset_server, "CreditIanWhitfield.png", 3);
    add_credits_slide(&mut commands, &asset_server, "CreditJordanBrudenell.png", 4);
    add_credits_slide(&mut commands, &asset_server, "CreditRuohengXu.png", 5);
    add_credits_slide(&mut commands, &asset_server, "CreditSamDurigon.png", 6);
    // after this, you just have to quit to restart
}

pub fn despawn_credits_page(
    mut commands: Commands,
    credits_page_query: Query<Entity, With<Popup>>,
) {
    for entity in credits_page_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn despawn_host_page(mut commands: Commands, host_page_entity: Query<Entity, With<HostPage>>) {
    if let Ok(host_page_entity) = host_page_entity.get_single() {
        commands.entity(host_page_entity).despawn_recursive();
    }
}

pub fn spawn_host_page(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let host_page_id = spawn_flex_column(&mut commands, HostPage);
    let mut host_page = commands.entity(host_page_id);
    spawn_title(&mut host_page, &font, "Host Game");
    spawn_input(
        &mut host_page,
        &font,
        (),
        HostPortInput {
            port: String::new(),
        },
        "Port: ",
    );
    spawn_input(
        &mut host_page,
        &font,
        NumCampsButton,
        NumCampsInput {
            value: String::new(),
        },
        "Number of Camps: ",
    );
    spawn_input(
        &mut host_page,
        &font,
        NumChestsButton,
        NumChestsInput {
            value: String::new(),
        },
        "Number of Chests: ",
    );
    spawn_input(
        &mut host_page,
        &font,
        EnemiesPerCampButton,
        EnemiesPerCampInput {
            value: String::new(),
        },
        "Number of Enemies Per Camp: ",
    );
    spawn_input(
        &mut host_page,
        &font,
        MapSeedButton,
        MapSeedInput {
            value: String::new(),
        },
        "Map Seed: ",
    );
    spawn_input(
        &mut host_page,
        &font,
        EidPercentageButton,
        EidPercentageInput {
            value: String::new(),
        },
        "EID Percentage: ",
    );
    spawn_button(&mut host_page, &font, HostPortSaveButton, "Host Now");
    spawn_button(&mut host_page, &font, BackToMainMenu, "Back");
}

pub fn despawn_join_page(mut commands: Commands, join_page_entity: Query<Entity, With<JoinPage>>) {
    if let Ok(join_page_entity) = join_page_entity.get_single() {
        commands.entity(join_page_entity).despawn_recursive();
    }
}

pub fn spawn_join_page(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let join_page_id = spawn_flex_column(&mut commands, JoinPage);
    let mut join_page = commands.entity(join_page_id);
    spawn_title(&mut join_page, &font, "Join a game");
    spawn_input(
        &mut join_page,
        &font,
        JoinPortButton,
        JoinPortInput {
            port: String::new(),
        },
        "Your Port: ",
    );
    spawn_input(
        &mut join_page,
        &font,
        JoinHostPortButton,
        JoinHostPortInput {
            port: String::new(),
        },
        "Host Port: ",
    );
    spawn_input(
        &mut join_page,
        &font,
        JoinIpButton,
        JoinIPInput { ip: String::new() },
        "Host IP: ",
    );
    spawn_button(&mut join_page, &font, JoinSaveButton, "Join Now");
    spawn_button(&mut join_page, &font, BackToMainMenu, "Back");
}

pub fn despawn_controls_page(
    mut commands: Commands,
    controls_page_entity: Query<Entity, With<ControlsPage>>,
) {
    if let Ok(controls_page_entity) = controls_page_entity.get_single() {
        commands.entity(controls_page_entity).despawn_recursive();
    }
}

pub fn spawn_controls_page(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text = commands
        .spawn(
            TextBundle::from_section(
                "Movement - WASD\n\
        Attack - Left Click\n\
        Interact - E\n\
        Quit Game - Esc",
                TextStyle {
                    font: font.clone(),
                    font_size: 24.0,
                    color: Color::BLACK,
                },
            )
            .with_text_alignment(TextAlignment::Center),
        )
        .id();
    let controls_page_id = spawn_flex_column(&mut commands, ControlsPage);
    let mut controls_page = commands.entity(controls_page_id);
    spawn_title(&mut controls_page, &font, "Controls");
    controls_page.add_child(text);
    spawn_button(&mut controls_page, &font, BackToMainMenu, "Back");
}

pub fn despawn_in_game_ui(mut commands: Commands, in_game_ui: Query<Entity, With<InGameUi>>) {
    if let Ok(in_game_ui) = in_game_ui.get_single() {
        commands.entity(in_game_ui).despawn_recursive();
    }
}

pub fn spawn_in_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn((
        TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(PADDING),
                top: Val::Px(SCREEN_HEIGHT - PADDING - 64.0),
                ..Default::default()
            },
            text: Text::from_section(
                "Score: 0",
                TextStyle {
                    font: font.clone(),
                    font_size: 64.0,
                    color: Color::RED,
                },
            )
            .with_alignment(TextAlignment::Left),
            ..Default::default()
        },
        ScoreDisplay,
        InGameUi,
    ));

    // Timer Display
    commands.spawn((
        TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(SCREEN_WIDTH / 2.0 - 100.0),
                top: Val::Px(PADDING),
                ..Default::default()
            },
            text: Text::from_section(
                "5:00",
                TextStyle {
                    font: font.clone(),
                    font_size: 64.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            ..Default::default()
        },
        GameTimer {
            remaining_time: ROUND_TIME,
        },
    ));

    // Powerup Display
    commands.spawn((
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(PADDING),
                top: Val::Px(SCREEN_HEIGHT / 2. - 176.),
                ..default()
            },
            image: asset_server.load("powerup_icons.png").into(),
            ..default()
        },
        InGameUi,
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(PADDING + 72.),
                    top: Val::Px(SCREEN_HEIGHT / 2. - 137.),
                    row_gap: Val::Px(40.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    ..default()
                },
                ..default()
            },
            InGameUi,
        ))
        .with_children(|parent| {
            for i in 0..NUM_POWERUPS {
                parent.spawn((
                    TextBundle::from_section(
                        "1.00x",
                        TextStyle {
                            font: font.clone(),
                            font_size: 32.0,
                            color: Color::WHITE,
                        },
                    ),
                    PowerupDisplayText(i as u8), // Dunno whether to attach this to NodeBundle or the individual TextBundles
                ));
            }
        });
}

pub fn update_time_remaining_system(
    time: Res<Time>,
    mut game_timer_query: Query<(&mut GameTimer, &mut Text)>,
    mut join_page_query: Query<&mut Style, With<JoinPage>>,
    mut game_over_query: Query<&mut Style, (With<GameOver>, Without<JoinPage>)>,
) {
    for (mut timer, mut text) in &mut game_timer_query {
        if timer.remaining_time > 0.0 {
            timer.remaining_time -= time.delta_seconds();

            let minutes = (timer.remaining_time / 60.0) as i32;
            let seconds = (timer.remaining_time % 60.0) as i32;

            text.sections[0].value = format!("{:02}:{:02}", minutes, seconds);
        } else {
            // TODO Handle game over logic
            for mut style in join_page_query.iter_mut() {
                style.display = Display::None;
            }
            for mut style in game_over_query.iter_mut() {
                style.display = Display::Flex;
            }
        }
    }
}

pub fn despawn_game_over_screen(
    mut commands: Commands,
    game_over_entity: Query<Entity, With<GameOver>>,
) {
    if let Ok(game_over_entity) = game_over_entity.get_single() {
        commands.entity(game_over_entity).despawn_recursive();
    }
}

pub fn spawn_game_over_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let color = Color::Rgba {
        red: (1.0),
        green: (1.0),
        blue: (1.0),
        alpha: (0.5),
    };
    let game_over_id = spawn_flex_column_colored(&mut commands, GameOver, color);
    let mut game_over = commands.entity(game_over_id);
    spawn_title(&mut game_over, &font, "Game Over");
    spawn_button(&mut game_over, &font, CreditsButton, "Credits");
    // no "back to main menu", just quit game and reopen it.
}

pub fn despawn_connecting_page(
    mut commands: Commands,
    connecting_page_entity: Query<Entity, With<ConnectingPage>>
) {
    if let Ok(connecting_page_entity) = connecting_page_entity.get_single() {
        commands.entity(connecting_page_entity).despawn_recursive();
    }
}

pub fn spawn_connecting_page(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let connecting_id = spawn_flex_column(&mut commands, ConnectingPage);
    let mut connecting = commands.entity(connecting_id);
    spawn_title(&mut connecting, &font, "Connecting...");
}

pub fn spawn_leaderboard_ui(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    stats_query: Query<&Player, With<Player>>
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let measure_names = ["Player", "Score", "Enemy Kills", "Player Kills", "Camps Captured", "Deaths", "KD"];
    // background
    let leaderboard_entity = commands
        .spawn((NodeBundle {
            style: Style {
                display: Display::None,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect {
                    left: Val::Px(40.),
                    right: Val::Px(40.),
                    top: Val::Px(120.),
                    bottom: Val::Px(40.),
                },
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.5, 0.5, 0.5, 0.5)),
            ..default()
        }, 
        LeaderboardUi)).id();
    // title
    let title_entity = commands
        .spawn(TextBundle::from_section(
            "Leaderboard".to_string(),
            TextStyle {
                font: font.clone(),
                font_size: 64.0,
                color: Color::RED,
            },
        )).id();
    commands.entity(leaderboard_entity).push_children(&[title_entity]);
    // field names
    let measures_entity = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(70.0),
                height: Val::Percent(20.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                padding: UiRect {
                    left: Val::Px(20.),
                    right: Val::Px(20.),
                    top: Val::Px(20.),
                    bottom: Val::Px(20.),
                },
                margin: UiRect {
                    left: Val::Px(0.),
                    right: Val::Px(0.),
                    top: Val::Px(40.),
                    bottom: Val::Px(0.),
                },
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.5, 0.5, 0.5, 0.5)),
            ..default()
        })
        .with_children(|parent| {
            for i in 0..measure_names.len() {
                parent.spawn(TextBundle::from_section(
                    measure_names[i],
                    TextStyle {
                        font: font.clone(),
                        font_size: 28.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    width: Val::Percent(100.0 / 21.0),
                    margin: UiRect {
                        left: Val::Percent(100.0 / 21.0),
                        right: Val::Percent(100.0 / 21.0),
                        top: Val::Px(0.),
                        bottom: Val::Px(0.),
                    },
                    ..default()
                }));
            }
        }).id();
    commands.entity(leaderboard_entity).push_children(&[measures_entity]);
    // player stats
    let player_icons = vec!["jordan_icon.png", "ian_icon.png", "sam_icon.png", "kevin_icon.png"];
    for i in 0..4 {
        let player_stats_entity = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(70.0),
                height: Val::Percent(15.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                padding: UiRect {
                    left: Val::Px(20.),
                    right: Val::Px(20.),
                    top: Val::Px(20.),
                    bottom: Val::Px(20.),
                },
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.5, 0.5, 0.5, 0.5)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((ImageBundle {
                image: asset_server.load(player_icons[i]).into(),
                style: Style {
                    width: Val::Percent(100.0 / 21.0),
                    max_height: Val::Percent(100.0),
                    margin: UiRect {
                        left: Val::Percent(100.0 / 21.0),
                        right: Val::Percent(100.0 / 21.0),
                        top: Val::Px(0.),
                        bottom: Val::Px(0.),
                    },
                    ..default()
                },
                ..default()
            },
            PlayerStatDisplay {
                player_id: i as u8,
                stat_id: 0,
            }));
            for j in 1..7 {
                parent.spawn((TextBundle::from_section(
                    j.to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    width: Val::Percent(100.0 / 21.0),
                    margin: UiRect {
                        left: Val::Percent(100.0 / 21.0),
                        right: Val::Percent(100.0 / 21.0),
                        top: Val::Px(0.),
                        bottom: Val::Px(0.),
                    },
                    ..default()
                }),
                PlayerStatDisplay {
                    player_id: i as u8,
                    stat_id: j as u8,
                }));
            }
        }).id();
        commands.entity(leaderboard_entity).push_children(&[player_stats_entity]);
    }
}

pub fn despawn_leaderboard_ui(
    mut commands: Commands,
    leaderboard_entity: Query<Entity, With<LeaderboardUi>>,
) {
    if let Ok(leaderboard_entity) = leaderboard_entity.get_single() {
        commands.entity(leaderboard_entity).despawn_recursive();
    }
}