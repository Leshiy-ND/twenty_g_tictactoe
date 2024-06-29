use bevy::prelude::*;

pub mod common;
use common::*;

mod field;
use field::*;



pub struct TheGamePlugin;

impl Plugin for TheGamePlugin {
    fn build(&self, app: &mut App) {
        app
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