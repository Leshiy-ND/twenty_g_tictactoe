use bevy::prelude::*;

use crate::AppState;


pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(Score {
                x:     0,
                o:     0,
                draws: 0
            }) 

            // State Systems
            .add_systems(OnEnter(AppState::InGame), clear_score)

            // End
            ;
    }
}



#[derive(Resource)]
pub struct Score{
    pub x:     u32,
    pub o:     u32,
    pub draws: u32,
}

fn clear_score(
    mut score: ResMut<Score>
) {
    score.x     = 0;
    score.o     = 0;
    score.draws = 0;
}