use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::players::Players;

use super::components::*;



pub fn despawn_mainmenu(
    mut commands: Commands,
    mmenu_q: Query<Entity, With<MainMenuRoot>>,
) {
    for mmenu in mmenu_q.iter() {
        commands.entity(mmenu).despawn_recursive();
    }
}

pub fn spawn_mainmenu(
    mut commands: Commands,
    players: Res<Players>,
    window_q: Query<&Window, With<PrimaryWindow>>
) {
    let window_height = window_q.single().height();

    commands.spawn((
        MainMenuRoot,
        NodeBundle {
            style: Style {
                width:  Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items:         AlignItems::Center,
                flex_direction:   FlexDirection::Column,
                row_gap: Val::Percent(2.0),
                ..default()
            },
            ..default()
        }
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                "TicTacToe",
                TextStyle {
                    font_size: window_height * 0.15,
                    color: Color::GRAY,
                    ..default()
                }
            ),
            ..default()
        });
        let button_h = window_height * 0.1;
        let text_h   = button_h * 0.66;
        spawn_mainmenu_xo_buttons(
            parent, 
            match players.player_x.is_real {
                true  => "X:person",
                false => "X:npc",
            },
            match players.player_o.is_real {
                true  => "O:person",
                false => "O:npc",
            }, 
            text_h,
            button_h,
        );
        spawn_mainmenu_button(
            parent,
            "New game",
            text_h,
            button_h,
            MainMenuButtonStart
        );
        // spawn_mainmenu_button(
        //     parent,
        //     "Exit",
        //     text_h,
        //     button_h,
        //     MainMenuButtonExit
        // );
    });
}

fn spawn_mainmenu_xo_buttons(
    parent: &mut ChildBuilder,
    text_left:  &str,
    text_right: &str,
    text_height_px:   f32,
    button_height_px: f32,
) {
    parent.spawn(NodeBundle {
        style: Style {
            width:  Val::Percent(70.0),
            height: Val::Px(button_height_px),
            justify_content: JustifyContent::SpaceBetween,
            flex_direction:   FlexDirection::Row,
            column_gap: Val::Vh(2.0),
            ..default()
        },
        ..default()
    })
    .with_children(|line| {
        line.spawn((
            MainMenuButtonX,
            ButtonBundle {
                background_color: BackgroundColor(Color::GRAY),
                style: Style {
                    width:  Val::Percent(100.0),
                    height: Val::Px(button_height_px),
                    justify_content: JustifyContent::Center,
                    align_items:         AlignItems::Center,
                    ..default()
                },
                ..default()
            }
        ))
        .with_children(|button| {
            button.spawn((
                MainMenuTextX,
                TextBundle {
                    text: Text::from_section(
                        text_left,
                        TextStyle {
                            font_size: text_height_px,
                            color: Color::BLACK,
                            ..default()
                        }
                    ),
                    ..default()
                }
            ));
        });
        line.spawn((
            MainMenuButtonO,
            ButtonBundle {
                background_color: BackgroundColor(Color::GRAY),
                style: Style {
                    width:  Val::Percent(100.0),
                    height: Val::Px(button_height_px),
                    justify_content: JustifyContent::Center,
                    align_items:         AlignItems::Center,
                    ..default()
                },
                ..default()
            }
        ))
        .with_children(|button| {
            button.spawn((
                MainMenuTextO,
                TextBundle {
                    text: Text::from_section(
                        text_right,
                        TextStyle {
                            font_size: text_height_px,
                            color: Color::BLACK,
                            ..default()
                        }
                    ),
                    ..default()
                }
            ));
        });
    });
}

fn spawn_mainmenu_button<T: Bundle>(
    parent: &mut ChildBuilder,
    text: &str,
    text_height_px:   f32,
    button_height_px: f32,
    additional_components: T,
) {
    parent.spawn((
        additional_components,
        ButtonBundle {
            background_color: BackgroundColor(Color::GRAY),
            style: Style {
                width:  Val::Percent(70.0),
                height: Val::Px(button_height_px),
                justify_content: JustifyContent::Center,
                align_items:         AlignItems::Center,
                ..default()
            },
            ..default()
        }
    ))
    .with_children(|button| {
        button.spawn(TextBundle {
            text: Text::from_section(
                text,
                TextStyle {
                    font_size: text_height_px,
                    color: Color::BLACK,
                    ..default()
                }
            ),
            ..default()
        });
    });
}