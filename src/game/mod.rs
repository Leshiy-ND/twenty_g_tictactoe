use bevy::prelude::*;

pub mod common;
use common::*;

mod cursor_position;
use cursor_position::CursorPositionPlugin;

mod field;
use field::*;

mod players;
use players::PlayersPlugin;



pub struct TheGamePlugin;

impl Plugin for TheGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins(CursorPositionPlugin)
            .add_plugins(PlayersPlugin)

            // Startup systems
            .add_systems(Startup, spawn_camera)
            .add_systems(Startup, new_game)

            // Update systems
            .add_systems(Update, draw_field)

            // End
            ;
    }
}



fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}



fn new_game(
    mut commands: Commands
) {
    commands.spawn( Field::default() );
}