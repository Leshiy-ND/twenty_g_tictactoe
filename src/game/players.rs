use std::borrow::Borrow;

use bevy::prelude::*;
use rand::prelude::random;

use super::{cursor_position::CursorPositon, Field, FieldTile};



pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(Players {
                turn_of_x: true,
                player_x: Player { is_npc: false },
                player_o: Player { is_npc: false },
                npc_thinking_timer: 2.0
            }) 

            // Update systems
            .add_systems(PreUpdate, process_player_turn)

            // End
            ;
    }
}



#[derive(Resource)]
pub struct Players {
    turn_of_x: bool,
    player_x: Player,
    player_o: Player,
    npc_thinking_timer: f32,
}

#[derive(Component)]
pub struct Player {
    is_npc: bool
}



pub fn process_player_turn(
    mouse_input: Res<ButtonInput<MouseButton>>,
    cursor_position: Res<CursorPositon>,
    mut players: ResMut<Players>,
    time: Res<Time>,

    mut q_field: Query<&mut Field>,
) {
    if let Err(_) = q_field.get_single() { return; }
    let mut field = q_field.single_mut();

    let current_player = if players.turn_of_x {players.player_x.borrow()} else {players.player_o.borrow()};
    #[allow(unused_assignments)]
    let mut made_turn = false;
    
    if current_player.is_npc {
        players.npc_thinking_timer -= time.delta_seconds();
        if players.npc_thinking_timer > 0.0 { return; }

        // NPC LOGIC

        made_turn = true;
    } else {
        if !mouse_input.just_pressed(MouseButton::Left) { return; }
    
        if let Err(_) = field.set_tile_by_cursor(
            cursor_position.0,
            if players.turn_of_x {FieldTile::X} else {FieldTile::O}
        ) {
            return;
        }

        made_turn = true;
    }

    if made_turn {
        players.turn_of_x = !players.turn_of_x;
        players.npc_thinking_timer = 0.5 + random::<f32>() * 2.0;
    }
}
