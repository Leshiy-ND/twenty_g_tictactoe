use bevy::prelude::*;
// use bevy::app::AppExit;

use crate::AppState;
use crate::game::players::Players;

use super::components::*;



pub fn update_hovered_button(
    mut button_q: Query<
        (
            &Interaction, &mut BackgroundColor,
            Option<&MainMenuButtonX>, Option<&MainMenuButtonO>
        ),
        Changed<Interaction>
    >,
) {
    for (
        interaction,
        mut bg_color,
        x_option,
        o_option,
    )
    in button_q.iter_mut() {
        match interaction {
            Interaction::Hovered => bg_color.0 = 
                if      let Some(_) = x_option { Color::CYAN   }
                else if let Some(_) = o_option { Color::ORANGE }
                else                           { Color::WHITE  },
            _                    => bg_color.0 = Color::GRAY,
        }
    }
}

pub fn update_hovered_button_text(
    mut button_q: Query<
        (&Interaction, Option<&MainMenuButtonX>, Option<&MainMenuButtonO>),
        Changed<Interaction>
    >,
    mut text_set: ParamSet<(
        Query<&mut Text, With<MainMenuTextX>>,
        Query<&mut Text, With<MainMenuTextO>>,
    )>,
) {
    for (
        interaction,
        x_option,
        o_option,
    )
    in button_q.iter_mut() {
        if let Some(_) = x_option {
            let mut text_x_q = text_set.p0();
            let mut text_x = text_x_q.single_mut();
            match interaction {
                Interaction::None => text_x.sections[0].style.color = Color::CYAN.with_l(0.75),
                _                 => text_x.sections[0].style.color = Color::BLACK,
            }
        }
        if let Some(_) = o_option {
            let mut text_o_q = text_set.p1();
            let mut text_o = text_o_q.single_mut();
            match interaction {
                Interaction::None => text_o.sections[0].style.color = Color::ORANGE.with_l(0.75),
                _                 => text_o.sections[0].style.color = Color::BLACK,
            }
        }
    }
}



pub fn interact_with_button_x(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut players: ResMut<Players>,
    button_query: Query<&Interaction, With<MainMenuButtonX>>,
    mut text_query: Query<&mut Text, With<MainMenuTextX>>,
) {
    if *button_query.single() != Interaction::Pressed { return; }
    if !mouse_input.just_pressed(MouseButton::Left) { return; }

    players.player_x.is_real = !players.player_x.is_real;
    text_query.single_mut().sections[0].value = match players.player_x.is_real {
        true  => "X:person",
        false => "X:npc",
    }.into();
}
pub fn interact_with_button_o(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut players: ResMut<Players>,
    button_query: Query<&Interaction, With<MainMenuButtonO>>,
    mut text_query: Query<&mut Text, With<MainMenuTextO>>,
) {
    if *button_query.single() != Interaction::Pressed { return; }
    if !mouse_input.just_pressed(MouseButton::Left) { return; }

    players.player_o.is_real = !players.player_o.is_real;
    text_query.single_mut().sections[0].value = match players.player_o.is_real {
        true  => "O:person",
        false => "O:npc",
    }.into();
}

pub fn interact_with_button_start(
    mut next_appstate: ResMut<NextState<AppState>>,
    button_query: Query<&Interaction, With<MainMenuButtonStart>>,
) {
    if *button_query.single() != Interaction::Pressed { return; }
    next_appstate.set(AppState::InGame);
}

// pub fn interact_with_button_exit(
//     button_query: Query<&Interaction, With<MainMenuButtonExit>>,
//     mut appexit_event: EventWriter<AppExit>
// ) {
//     if *button_query.single() != Interaction::Pressed { return; }
//     appexit_event.send(AppExit);
// }