use std::borrow::Borrow;

use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::random;

use crate::AppState;

use super::cursor_position::CursorPositon;
use super::field::{Field, FieldTile};



pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(Players {
                turn_of_x: true,
                player_x: Player { is_real: false },
                player_o: Player { is_real: true  },
                npc_thinking_timer: 2.0,
                timer: Stopwatch::new()
            }) 

            // State Systems
            .add_systems(OnEnter(AppState::InGame), clear_players)

            // Update systems
            .add_systems(PreUpdate, process_player_turn
                .run_if(in_state(AppState::InGame))
            )

            // End
            ;
    }
}



#[derive(Resource)]
pub struct Players {
    pub turn_of_x: bool,
    pub player_x: Player,
    pub player_o: Player,
    npc_thinking_timer: f32,
    timer: Stopwatch,
}

impl Players {
    pub fn get_timer_str(&self) -> String {
        let secs = self.timer.elapsed_secs() as u32;
        format!("{:0>2}:{:0>2}", secs / 60, secs % 60)
    }
}

#[derive(Component)]
pub struct Player {
    pub is_real: bool,
}



pub fn clear_players(
    mut players: ResMut<Players>,
) {
    players.turn_of_x = true;
    players.timer.reset();
}

pub fn process_player_turn(
    mouse_input: Res<ButtonInput<MouseButton>>,
    cursor_position: Res<CursorPositon>,
    mut players: ResMut<Players>,
    mut field: ResMut<Field>,
    time: Res<Time>,
) {
    players.timer.tick(time.delta());
    if field.is_finished() { return; }

    let current_player = if players.turn_of_x {players.player_x.borrow()} else {players.player_o.borrow()};
    let tile_to_set  = if players.turn_of_x {FieldTile::X} else {FieldTile::O};

    #[allow(unused_assignments)]
    let mut made_turn = false;
    
    if current_player.is_real {
        if !mouse_input.just_pressed(MouseButton::Left) { return; }
    
        if let Err(_) = field.set_tile_by_cursor(
            cursor_position.0,
            tile_to_set
        ) {
            return;
        }

        made_turn = true;
    } else {
        players.npc_thinking_timer -= time.delta_seconds();
        if players.npc_thinking_timer > 0.0 { return; }

        // NPC LOGIC

        let mut about_to_win:  Option<UVec2> = None;
        let mut about_to_lose: Option<UVec2> = None;

        let mut check_about_tos = |poses: [UVec2; 3]| {

            match FieldTile::is_about_to_be_closed([
                field.get_tile(poses[0].y, poses[0].x),
                field.get_tile(poses[1].y, poses[1].x),
                field.get_tile(poses[2].y, poses[2].x),
            ]) {
                Err(_) => (),
                Ok((i, tile)) => {
                    let about_to =
                    if tile == tile_to_set { &mut about_to_win  }
                    else                   { &mut about_to_lose };
                    if let None = about_to {
                        *about_to = Some(poses[i]);
                    }
                },
            }
        };

        loop {
            // ABOUT TO WIN or LOSE

            // Lines
            for y in 0..3 { check_about_tos([ UVec2 { y, x: 0 }, UVec2 { y, x: 1 }, UVec2 { y, x: 2 } ]); }
            for x in 0..3 { check_about_tos([ UVec2 { x, y: 0 }, UVec2 { x, y: 1 }, UVec2 { x, y: 2 } ]); }

            // Diagonals
            check_about_tos([ UVec2 { x: 0, y: 0 }, UVec2 { x: 1, y: 1 }, UVec2 { x: 2, y: 2 } ]);
            check_about_tos([ UVec2 { x: 0, y: 2 }, UVec2 { x: 1, y: 1 }, UVec2 { x: 2, y: 0 } ]);

            if let Some(pos) = about_to_win {
                field.set_tile(pos.y, pos.x, tile_to_set);
                made_turn = true;
                break;
            }
            if let Some(pos) = about_to_lose {
                field.set_tile(pos.y, pos.x, tile_to_set);
                made_turn = true;
                break;
            }

            // RANDOM MOVE

            loop {
                let rand_i = random::<u32>() % 9;
                let rand_y = rand_i / 3;
                let rand_x = rand_i % 3;
                
                if field.get_tile(rand_y, rand_x) == FieldTile::Empty {
                    field.set_tile(rand_y, rand_x, tile_to_set);
                    made_turn = true;
                    break;
                }
            }

            break;
        }

        // NPC LOGIC
    }

    if made_turn {
        players.turn_of_x = !players.turn_of_x;
        players.npc_thinking_timer = 0.5 + random::<f32>() * 2.0;
    }
}
