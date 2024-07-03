use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;



pub fn despawn_ingame_ui(
    mut commands: Commands,
    ui_q: Query<Entity, With<IngameUiRoot>>,
) {
    for ui in ui_q.iter() {
        commands.entity(ui).despawn_recursive();
    }
}

pub fn spawn_ingame_ui(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>
) {
    let window_height = window_q.single().height();
    let button_h = window_height * 0.075;
    let text_h = button_h * 0.7;

    commands.spawn((
        IngameUiRoot,
        NodeBundle {
            style: Style {
                width:  Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Relative,
                ..default()
            },
            ..default()
        }
    ))
    .with_children(|root| {
        root.spawn(
            NodeBundle {
                style: Style {
                    top:    Val::Px((window_height / 8.0 - button_h) / 2.0),
                    left:   Val::Percent(15.0),
                    width:  Val::Percent(70.0),
                    height: Val::Px(button_h),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items:         AlignItems::Center,
                    flex_direction:   FlexDirection::Row,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            }
        )
        .with_children(|upper_line| {
            upper_line.spawn((
                IngameUiButtonExit,
                ButtonBundle {
                    style: Style {
                        width:  Val::Vw(20.0),
                        height: Val::Px(button_h),
                        justify_content: JustifyContent::Center,
                        align_items:         AlignItems::Center,
                        ..default()
                    },
                    ..default()
                }
            ))
            .with_children(|button| {
                button.spawn(
                    TextBundle {
                        text: Text::from_section(
                            "Exit",
                            TextStyle {
                                font_size: text_h,
                                color: Color::BLACK,
                                ..default()
                            }
                        ),
                        ..default()
                    }
                );
            });

            upper_line.spawn((
                IngameUiTimer,
                TextBundle {
                    text: Text::from_section(
                        "00:00",
                        TextStyle {
                            font_size: text_h,
                            color: Color::WHITE.with_a(0.5),
                            ..default()
                        }
                    ),
                    ..default()
                }
            ));

            upper_line.spawn((
                IngameUiButtonNext,
                ButtonBundle {
                    style: Style {
                        width:  Val::Vw(20.0),
                        height: Val::Px(button_h),
                        justify_content: JustifyContent::Center,
                        align_items:         AlignItems::Center,
                        ..default()
                    },
                    ..default()
                }
            ))
            .with_children(|button| {
                button.spawn(
                    TextBundle {
                        text: Text::from_section(
                            "Next",
                            TextStyle {
                                font_size: text_h,
                                color: Color::BLACK,
                                ..default()
                            }
                        ),
                        ..default()
                    }
                );
            });
        });

        root.spawn(
            NodeBundle {
                style: Style {
                    // top:    Val::Auto,
                    bottom: Val::Px((window_height / 8.0 - button_h) / 2.0),
                    left:   Val::Percent(15.0),
                    width:  Val::Percent(70.0),
                    height: Val::Px(button_h),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items:         AlignItems::Center,
                    flex_direction:   FlexDirection::Row,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            }
        )
        .with_children(|bottom_line| {
            bottom_line.spawn((
                IngameUiScoreX,
                TextBundle {
                    text: Text::from_section(
                        "0",
                        TextStyle {
                            font_size: button_h,
                            color: Color::CYAN.with_a(0.5),
                            ..default()
                        }
                    ),
                    ..default()
                }
            ));
            bottom_line.spawn((
                IngameUiScoreDraws,
                TextBundle {
                    text: Text::from_section(
                        "0",
                        TextStyle {
                            font_size: text_h,
                            color: Color::WHITE.with_a(0.5),
                            ..default()
                        }
                    ),
                    ..default()
                }
            ));
            bottom_line.spawn((
                IngameUiScoreO,
                TextBundle {
                    text: Text::from_section(
                        "0",
                        TextStyle {
                            font_size: button_h,
                            color: Color::ORANGE.with_a(0.5),
                            ..default()
                        }
                    ),
                    ..default()
                }
            ));
        });
    });
}
