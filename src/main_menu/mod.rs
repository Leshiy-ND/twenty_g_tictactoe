use bevy::prelude::*;

mod components;

mod interactions;
use interactions::*;

mod layout;
use layout::*;

use crate::AppState;



pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // State Systems
            .add_systems(OnEnter (AppState::MainMenu),   spawn_mainmenu)
            .add_systems(OnExit  (AppState::MainMenu), despawn_mainmenu)

            // Update Systems
            .add_systems(Update, (
                    update_hovered_button,
                    update_hovered_button_text,
                    interact_with_button_x,
                    interact_with_button_o,
                    interact_with_button_start,
                    // interact_with_button_exit,
                )
                .run_if(in_state(AppState::MainMenu))
            )
            
            // End
            ;
    }
}