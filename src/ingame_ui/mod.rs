use bevy::prelude::*;

mod components;

mod interactions;
use interactions::*;

mod layout;
use layout::*;

use crate::AppState;



pub struct IngameUiPlugin;

impl Plugin for IngameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // State Systems
            .add_systems(OnEnter (AppState::InGame),   spawn_ingame_ui)
            .add_systems(OnExit  (AppState::InGame), despawn_ingame_ui)

            // Update Systems
            .add_systems(Update, (
                    update_hovered_button,
                    update_timer,
                    updates_scores,
                    interact_with_button_exit,
                    interact_with_button_next,
                )
                .run_if(in_state(AppState::InGame))
            )
            
            // End
            ;
    }
}
