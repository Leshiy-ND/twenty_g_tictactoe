use bevy::{prelude::*, window::WindowResolution};

// mod game;
// use game::TheGamePlugin;
// use game::common::GRID_SIZE;



fn main() {
    App::new()
        // Plugins
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(
                        Window {
                            resolution: WindowResolution::new(
                                 500.0,
                                500.0
                            ),
                            ..default()
                        }
                    ),
                    ..default()
                }),
            // TheGamePlugin,
        ))

        // Run
        .run();
}
