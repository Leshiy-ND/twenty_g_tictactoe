use bevy::prelude::*;
// use bevy::app::AppExit;

use crate::AppState;
use crate::game::field::Field;
use crate::game::players::Players;
use crate::game::score::Score;

use super::components::*;



pub fn update_hovered_button(
    field: Res<Field>,
    mut button_q: Query<
        (&Interaction, &mut BackgroundColor, Option<&IngameUiButtonNext>)
    >,
) {
    for (
        interaction,
        mut bg_color,
        next_option,
    )
    in button_q.iter_mut() {
        if let Some(_) = next_option {
            if !field.is_finished() {
                bg_color.0 = Color::GRAY.with_a(0.3);
                continue;
            }
        }
        match interaction {
            Interaction::Hovered => bg_color.0 = Color::WHITE,
            _                    => bg_color.0 = Color::GRAY,
        }
    }
}

pub fn interact_with_button_exit(
    mut next_appstate: ResMut<NextState<AppState>>,
    button_query: Query<&Interaction, With<IngameUiButtonExit>>,
) {
    if *button_query.single() != Interaction::Pressed { return; }
    next_appstate.set(AppState::MainMenu);
}

pub fn interact_with_button_next(
    mut field: ResMut<Field>,
    mut players: ResMut<Players>,
    button_query: Query<&Interaction, With<IngameUiButtonNext>>,
) {
    if *button_query.single() != Interaction::Pressed { return; }
    if !field.is_finished() { return; }

    players.turn_of_x = true;
    field.clear();
}



pub fn update_timer(
    players: Res<Players>,
    mut timer_q: Query<&mut Text, With<IngameUiTimer>>
) {
    timer_q.single_mut().sections[0].value = players.get_timer_str();
}

pub fn updates_scores(
    score: Res<Score>,
    mut text_set: ParamSet<(
        Query<&mut Text, With< IngameUiScoreX     >>,
        Query<&mut Text, With< IngameUiScoreO     >>,
        Query<&mut Text, With< IngameUiScoreDraws >>,
    )>
) {
    if !score.is_changed() { return; }

    text_set.p0().single_mut().sections[0].value = score.x     .to_string();
    text_set.p1().single_mut().sections[0].value = score.o     .to_string();
    text_set.p2().single_mut().sections[0].value = score.draws .to_string();
}