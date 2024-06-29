use bevy::prelude::*;

use super::GRID_SIZE;



// #[derive(Default)]
pub enum FieldTile {
    // #[default]
    Empty,
    X,
    O,
}

// #[derive(Component, Default)]
#[derive(Component)]
pub struct Field {
    pub tiles: [[FieldTile; 3]; 3]
}

impl Default for Field {
    fn default() -> Self {
        Field { tiles: [
            [FieldTile::X,     FieldTile::O,     FieldTile::Empty],
            [FieldTile::Empty, FieldTile::Empty, FieldTile::Empty],
            [FieldTile::Empty, FieldTile::Empty, FieldTile::Empty]
        ] }
    }
}



pub fn draw_field(
    mut gizmos: Gizmos,
    field_query: Query<&Field>,
) {
    if let Err(_) = field_query.get_single() { return; }
    let field = field_query.single();

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