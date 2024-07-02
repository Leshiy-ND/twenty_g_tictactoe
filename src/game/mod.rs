use bevy::prelude::*;

pub mod common;
use common::*;

mod cursor_position;
use cursor_position::CursorPositionPlugin;

mod field;
use field::FieldPlugin;

pub mod players;
use players::PlayersPlugin;



pub struct TheGamePlugin;

impl Plugin for TheGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins(CursorPositionPlugin)
            .add_plugins(FieldPlugin)
            .add_plugins(PlayersPlugin)

            // Startup systems
            .add_systems(Startup, spawn_camera)

            // End
            ;
    }
}



fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}
