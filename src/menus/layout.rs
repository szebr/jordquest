use bevy::prelude::*;
use bevy::prelude::Deref;
use bevy::prelude::DerefMut;
use bevy::prelude::Timer;

#[derive(Component, Deref, DerefMut)]
pub struct PopupTimer(Timer);

use crate::menus::components::*;

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn spawn_credits_page(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    // let credit_page_entity = build_credits_page(&mut commands, &asset_server);
    
    // commands.entity(credit_page_entity).despawn_recursive();
    // for (entity, _ui_Image) in &mut query.iter() {
    //     thread::sleep(time::Duration::from_secs(5));
    //     commands.entity(credit_page_entity).remove_children(&[entity]);
    //     print!("despawned");
    // }
        // let image_names = vec![
    //     "brendan_credits_slide.png", 
    //     "CreditAlexLampe.png", 
    //     "CreditGarrettDiCenzo.jpg", 
    //     "CreditIanWhitfield.png", 
    //     "CreditJordanBrudenell.png", 
    //     "CreditRuohengXu.jpg", 
    //     "CreditSamDurigon.png"
    // ];

    // TODO: Find a way to not spawn the game 
    commands.spawn(SpriteBundle {
        texture: asset_server.load("brendan_credits_slide.png"),
        transform: Transform::from_xyz(0., 0., -0.9),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(3., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("CreditAlexLampe.png"),
        transform: Transform::from_xyz(0., 0., -0.8),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(6., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("CreditGarrettDiCenzo.png"),
        transform: Transform::from_xyz(0., 0., -0.7),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(9., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("CreditIanWhitfield.png"),
        transform: Transform::from_xyz(0., 0., -0.6),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(12., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("CreditJordanBrudenell.png"),
        transform: Transform::from_xyz(0., 0., -0.5),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(15., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("CreditRuohengXu.png"),
        transform: Transform::from_xyz(0., 0., -0.4),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(18., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("CreditSamDurigon.png"),
        transform: Transform::from_xyz(0., 0., -0.3),
        ..default()
    })
    .insert(PopupTimer(Timer::from_seconds(27., TimerMode::Once)));
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.2,0.2,0.2),
            custom_size: Some(Vec2::new(1280.,720.)),
            ..default()
        },
        ..default()
    });

}

pub fn show_popup(time: Res<Time>, mut popup: Query<(&mut PopupTimer, &mut Transform)>) {
    for (mut timer, mut transform) in popup.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.z += 1.;
        }
    }
}

pub fn despawn_main_menu(
    mut commands: Commands, 
    main_menu_query: Query<Entity, With<MainMenu>>,
){
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn despawn_credits_page(
    mut commands: Commands, 
    credits_page_query: Query<Entity, With<CreditsPage>>,
){
    if let Ok(credits_page_entity) = credits_page_query.get_single() {
        commands.entity(credits_page_entity).despawn_recursive();
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
    // let image_names = vec![
    //     "brendan_credits_slide.png", 
    //     "CreditAlexLampe.png", 
    //     "CreditGarrettDiCenzo.jpg", 
    //     "CreditIanWhitfield.png", 
    //     "CreditJordanBrudenell.png", 
    //     "CreditRuohengXu.jpg", 
    //     "CreditSamDurigon.png"
    // ];
    
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
        CreditsPage{},
    ))
    // .with_children(|parent|{
    //     parent.spawn(
    //         ImageBundle {
    //             style: Style{
    //                 width: Val::Percent(100.0),
    //                 height: Val::Percent(100.0),
    //                 ..default()
    //             },
    //             image: UiImage{
    //                 texture: asset_server.load("brendan_credits_slide.png"),
    //                 ..default()
    //             },
    //             z_index: ZIndex::Local(0),
    //             ..default()
    //         });
    // })
    // .with_children(|parent|{
    //     parent.spawn(
    //         ImageBundle {
    //             style: Style{
    //                 width: Val::Percent(100.0),
    //                 height: Val::Percent(100.0),
    //                 ..default()
    //             },
    //             image: UiImage{
    //                 texture: asset_server.load("CreditAlexLampe.png"),
    //                 ..default()
    //             },
    //             z_index: ZIndex::Local(1),
    //             ..default()
    //         });
    // })
    // .with_children(|parent|{
    //     parent.spawn(
    //         ImageBundle {
    //             style: Style{
    //                 width: Val::Percent(100.0),
    //                 height: Val::Percent(100.0),
    //                 ..default()
    //             },
    //             image: UiImage{
    //                 texture: asset_server.load("CreditGarrettDiCenzo.jpg"),
    //                 ..default()
    //             },
    //             z_index: ZIndex::Local(2),
    //             ..default()
    //         });
    // })
    // .with_children(|parent|{
    //     parent.spawn(
    //         ImageBundle {
    //             style: Style{
    //                 width: Val::Percent(100.0),
    //                 height: Val::Percent(100.0),
    //                 ..default()
    //             },
    //             image: UiImage{
    //                 texture: asset_server.load("CreditIanWhitfield.png"),
    //                 ..default()
    //             },
    //             z_index: ZIndex::Local(3),
    //             ..default()
    //         });
    // })
    // .with_children(|parent|{
    //     parent.spawn(
    //         ImageBundle {
    //             style: Style{
    //                 width: Val::Percent(100.0),
    //                 height: Val::Percent(100.0),
    //                 ..default()
    //             },
    //             image: UiImage{
    //                 texture: asset_server.load("CreditJordanBrudenell.png"),
    //                 ..default()
    //             },
    //             z_index: ZIndex::Local(4),
    //             ..default()
    //         });
    // })
    // .with_children(|parent|{
    //     parent.spawn(
    //         ImageBundle {
    //             style: Style{
    //                 width: Val::Percent(100.0),
    //                 height: Val::Percent(100.0),
    //                 ..default()
    //             },
    //             image: UiImage{
    //                 texture: asset_server.load("CreditRuohengXu.jpg"),
    //                 ..default()
    //             },
    //             z_index: ZIndex::Local(5),
    //             ..default()
    //         });
    // })
    // .with_children(|parent|{
    //     parent.spawn(
    //         ImageBundle {
    //             style: Style{
    //                 width: Val::Percent(100.0),
    //                 height: Val::Percent(100.0),
    //                 ..default()
    //             },
    //             image: UiImage{
    //                 texture: asset_server.load("CreditSamDurigon.png"),
    //                 ..default()
    //             },
    //             z_index: ZIndex::Local(6),
    //             ..default()
    //         });
    // })

    .id();

    credits_page_entity
}