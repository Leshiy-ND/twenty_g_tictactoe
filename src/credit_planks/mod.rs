use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;



pub struct CreditPlanksPlugin;

impl Plugin for CreditPlanksPlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup Systems
            .add_systems(Startup, spawn_credit_planks)
            
            // End
            ;
    }
}



fn spawn_credit_planks(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>
) {
    let window_height = window_q.single().height();

    commands.spawn(
        NodeBundle {
            style: Style {
                width:  Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Relative,
                ..default()
            },
            ..default()
        }
    )
    .with_children(|root| {
        let credit_plank_h = window_height* 0.05;

        root.spawn(
            NodeBundle {
                style: Style {
                    top:    Val::Px((window_height - credit_plank_h) / 2.0),
                    left:   Val::Px(credit_plank_h / 2.0 - window_height),
                    width:  Val::Percent(200.0),
                    height: Val::Px(credit_plank_h),
                    justify_content: JustifyContent::Center,
                    align_items:         AlignItems::Center,
                    position_type:     PositionType::Absolute,
                    ..default()
                },
                transform: Transform::from_rotation(
                    Quat::from_rotation_z(PI * 0.5)
                ),
                ..default()
            }
        )
        .with_children(|plank| {
            plank.spawn(
                TextBundle {
                    text: Text::from_section(
                        "Leshiy_ND * Bevy 0.13 * Leshiy_ND * Bevy 0.13 * Leshiy_ND",
                        TextStyle {
                            font_size: credit_plank_h * 0.66,
                            color: Color::GRAY.with_a(0.33),
                            ..default()
                        }
                    ),
                    ..default()
                }
            );
        });

        root.spawn(
            NodeBundle {
                style: Style {
                    top:    Val::Px((window_height - credit_plank_h) / 2.0),
                    left:   Val::Px(- credit_plank_h / 2.0),
                    width:  Val::Percent(200.0),
                    height: Val::Px(credit_plank_h),
                    justify_content: JustifyContent::Center,
                    align_items:         AlignItems::Center,
                    position_type:     PositionType::Absolute,
                    ..default()
                },
                transform: Transform::from_rotation(
                    Quat::from_rotation_z(- PI * 0.5)
                ),
                ..default()
            }
        )
        .with_children(|plank| {
            plank.spawn(
                TextBundle {
                    text: Text::from_section(
                        "Leshiy_ND * Bevy 0.13 * Leshiy_ND * Bevy 0.13 * Leshiy_ND",
                        TextStyle {
                            font_size: credit_plank_h * 0.66,
                            color: Color::GRAY.with_a(0.33),
                            ..default()
                        }
                    ),
                    ..default()
                }
            );
        });
    });
}