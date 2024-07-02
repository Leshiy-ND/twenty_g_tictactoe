use bevy::prelude::*;

use crate::AppState;

use super::GRID_SIZE;



pub struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<EndGameEvent>() 

            // Resources
            .init_resource::<Field>() 

            // Update systems
            .add_systems(Update, (
                    chech_field,
                    end_game,
                )
                .run_if(in_state(AppState::InGame))
            )

            // PostUpdate systems
            .add_systems(PostUpdate, draw_field
                .run_if(in_state(AppState::InGame))
            )

            // End
            ;
    }
}



#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum FieldTile {
    #[default]
    Empty,
    X,
    O,
}

impl FieldTile {
    pub fn is_about_to_be_closed(tiles: [FieldTile; 3]) -> Result<(usize, FieldTile), ()> {
        if tiles[0] != FieldTile::Empty && tiles[0] == tiles[1] && tiles[2] == FieldTile::Empty { return Ok( (2, tiles[0]) ); }
        if tiles[0] != FieldTile::Empty && tiles[0] == tiles[2] && tiles[1] == FieldTile::Empty { return Ok( (1, tiles[0]) ); }
        if tiles[1] != FieldTile::Empty && tiles[1] == tiles[2] && tiles[0] == FieldTile::Empty { return Ok( (0, tiles[1]) ); }
        Err(())
    }
}



#[derive(Resource, Default)]
pub struct Field {
    tiles: [[FieldTile; 3]; 3],
    end_game_option: Option<FieldTile>,
}

impl Field {
    pub fn is_finished(&self) -> bool {
        match self.end_game_option {
            Some(_) => true,
            None    => false,
        }
    }

    pub fn get_tile(&self, y: u32, x: u32) -> FieldTile {
        self.tiles[y as usize][x as usize]
    }

    pub fn set_tile(&mut self, y: u32, x: u32, tile: FieldTile) {
        self.tiles[y as usize][x as usize] = tile;
    }

    pub fn set_tile_by_cursor(&mut self, position: Vec2, tile: FieldTile) -> Result<(), ()> {
        if let Some(_) = self.end_game_option { return Err(()); }
        if position.x.abs().max(position.y.abs()) > GRID_SIZE as f32 * 1.5 { return Err(()); }

        let x =     (position.x + GRID_SIZE as f32 * 1.5) as usize / GRID_SIZE as usize;
        let y = 2 - (position.y + GRID_SIZE as f32 * 1.5) as usize / GRID_SIZE as usize;

        match self.tiles[y][x] {
            FieldTile::Empty => {
                self.tiles[y][x] = tile;
                return Ok(());
            },
            _ => return Err(()),
        }
    }
}



#[derive(Event)]
struct EndGameEvent(FieldTile);

fn end_game(
    mut end_game_event: EventReader<EndGameEvent>,
    mut field: ResMut<Field>,
) {
    for event in end_game_event.read() {
        field.end_game_option = Some(event.0);
    }
}

fn chech_field(
    mut end_game_event: EventWriter<EndGameEvent>,
    field: Res<Field>,
) {
    if !field.is_changed() { return; }

    let ending = {
        let mut full = true;

        for line in field.tiles.iter() {
            for tile in line.iter() {
                match tile {
                    FieldTile::Empty => {
                        full = false;
                        break;
                    },
                    _ => (),
                }
            }
            if !full { break; }
        }
        full
    };

    for line in field.tiles.iter() {
        let first_tile = line[0];
        if first_tile == FieldTile::Empty { continue; }
        if first_tile != line[1]          { continue; }
        if first_tile != line[2]          { continue; }
        end_game_event.send(EndGameEvent(first_tile));
        return;
    }
    for x in 0..3 {
        let first_tile = field.tiles[0][x];
        if first_tile == FieldTile::Empty  { continue; }
        if first_tile != field.tiles[1][x] { continue; }
        if first_tile != field.tiles[2][x] { continue; }
        end_game_event.send(EndGameEvent(first_tile));
        return;
    }
    loop {
        let center_tile = field.tiles[1][1];
        if center_tile == FieldTile::Empty { break; }
        
        loop {
            if center_tile != field.tiles[0][0] { break; }
            if center_tile != field.tiles[2][2] { break; }
            end_game_event.send(EndGameEvent(center_tile));
            return;
        }
        loop {
            if center_tile != field.tiles[0][2] { break; }
            if center_tile != field.tiles[2][0] { break; }
            end_game_event.send(EndGameEvent(center_tile));
            return;
        }
        break;
    }
    
    if ending {
        end_game_event.send(EndGameEvent(FieldTile::Empty));
    }
}



pub fn draw_field(
    mut gizmos: Gizmos,
    field: Res<Field>,
) {
    let (mut x, mut y);

    (x, y) = (-(GRID_SIZE as f32 * 0.5), -(GRID_SIZE as f32 * 1.5));
    for _ in 0..2 {
        gizmos.line_2d(
            Vec2 { x, y },
            Vec2 { x, y: y + (GRID_SIZE * 3) as f32 },
            Color::WHITE
        );
        x += GRID_SIZE as f32;
    }

    (y, x) = (-(GRID_SIZE as f32 * 0.5), y);
    for _ in 0..2 {
        gizmos.line_2d(
            Vec2 { x, y },
            Vec2 { x: x + (GRID_SIZE * 3) as f32, y },
            Color::WHITE
        );
        y += GRID_SIZE as f32;
    }

    if let Some(win) = field.end_game_option {
        gizmos.rect_2d(
            Vec2::ZERO,
            0.0,
            Vec2::splat((GRID_SIZE * 3) as f32),
            match win {
                FieldTile::Empty => Color::WHITE,
                FieldTile::X     => Color::CYAN,
                FieldTile::O     => Color::ORANGE,
            }
            .with_a(0.5)
        )
    }

    let raduis_cross  = GRID_SIZE as f32 * 0.30;
    let raduis_cicrle = GRID_SIZE as f32 * 0.35;

    y = GRID_SIZE as f32;
    for line in field.tiles.iter() {
        x = -(GRID_SIZE as f32);
        
        for tile in line.iter() {
            match tile {
                FieldTile::Empty => (),
                FieldTile::X => {
                    gizmos.line_2d(
                        Vec2 { x: x - raduis_cross, y: y - raduis_cross },
                          Vec2 { x: x + raduis_cross, y: y + raduis_cross },
                        Color::CYAN
                    );
                    gizmos.line_2d(
                        Vec2 { x: x - raduis_cross, y: y + raduis_cross },
                          Vec2 { x: x + raduis_cross, y: y - raduis_cross },
                        Color::CYAN
                    );
                },
                FieldTile::O => {
                    gizmos.circle_2d(
                        Vec2 { x, y },
                        raduis_cicrle,
                        Color::ORANGE
                    );
                },
            }
            x += GRID_SIZE as f32;
        }
        y -= GRID_SIZE as f32;
    }
}