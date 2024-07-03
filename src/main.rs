use bevy::{prelude::*, window::WindowResolution};

mod game;
use game::TheGamePlugin;
use game::common::GRID_SIZE;

mod main_menu;
use main_menu::MainMenuPlugin;

mod ingame_ui;
use ingame_ui::IngameUiPlugin;



fn main() {
    App::new()
        // States
        .init_state::<AppState>()

        // Plugins
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(
                        Window {
                            resolution: WindowResolution::new(
                                 (GRID_SIZE * 4) as f32,
                                (GRID_SIZE * 4) as f32
                            ),
                            ..default()
                        }
                    ),
                    ..default()
                }),
            TheGamePlugin,
            IngameUiPlugin,
            MainMenuPlugin
        ))

        // Run
        .run();
}



#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}
