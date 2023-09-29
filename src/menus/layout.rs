use bevy::prelude::*;

use crate::menus::components::*;

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn spawn_credits_page(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let credit_page_entity = build_credits_page(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
    mut commands: Commands, 
    main_menu_query: Query<Entity, With<MainMenu>>,
){
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn(
            (NodeBundle {
                style: Style{
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color:: WHITE.into(),
                ..default()
        },
        MainMenu{},

    ))
    //play but
    .with_children(|parent|{
        //title
        parent.spawn(
            (
                ButtonBundle{
                    style: Style{
                        width: Val::Px(600.0),
                        height: Val::Px(80.0),
                        margin: UiRect {
                            left: Val::Px(8.),
                            right: Val::Px(8.),
                            top: Val::Px(0.0),
                            bottom: Val::Px(60.0)
                        },
                        justify_content:JustifyContent::Center,
                        align_items:AlignItems::Center,
                        ..default()
                    },
                    background_color: Color:: WHITE.into(),
                    ..default()
                },
            )
        )
        .with_children(|parent|{
            parent.spawn(
                TextBundle{
                    text: Text {
                        sections: vec![TextSection::new(
                            "Red Delicious",
                            TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 64.0, color: Color:: RED },
                        )],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                });
        });

        parent.spawn(
            (
                ButtonBundle{
                    style: Style{
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        margin: UiRect {
                            left: Val::Px(8.),
                            right: Val::Px(8.),
                            top: Val::Px(0.0),
                            bottom: Val::Px(8.0)
                        },
                        justify_content:JustifyContent::Center,
                        align_items:AlignItems::Center,
                        ..default()
                    },
                    background_color: Color:: rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                },
                PlayButton{},
            )
        )
        .with_children(|parent|{
            parent.spawn(
                TextBundle{
                    text: Text {
                        sections: vec![TextSection::new(
                            "Play",
                            TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 20.0, color: Color:: WHITE },
                        )],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                });
        });
        //credits but
        parent.spawn(
            (
                ButtonBundle{
                    style: Style{
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        margin: UiRect {
                            left: Val::Px(8.),
                            right: Val::Px(8.),
                            top: Val::Px(0.0),
                            bottom: Val::Px(8.0)
                        },
                        justify_content:JustifyContent::Center,
                        align_items:AlignItems::Center,
                        ..default()
                    },
                    background_color: Color:: rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                },
                CreditsButton{},
            )
        )
        .with_children(|parent|{
            parent.spawn(
                TextBundle{
                    text: Text {
                        sections: vec![TextSection::new(
                            "Credits page",
                            TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 20.0, color: Color:: WHITE },
                        )],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                });
        });
    })

    .id();

    main_menu_entity
}

pub fn build_credits_page(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let credits_page_entity = commands
        .spawn(
            (NodeBundle {
                style: Style{
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color:: WHITE.into(),
                ..default()
        },
    ))
    .id();

    credits_page_entity
}