use bevy::{prelude::*, window::PrimaryWindow};



pub struct CursorPositionPlugin;

impl Plugin for CursorPositionPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<CursorPositon>() 

            // Update systems
            .add_systems(PreUpdate, update_cursor_position)

            // End
            ;
    }
}



#[derive(Resource, Default)]
pub struct CursorPositon(pub Vec2);

pub fn update_cursor_position(
    mut cursor_position: ResMut<CursorPositon>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    if let Err(_) = q_window.get_single() { return; }
    let window = q_window.single();

    if let Err(_) = q_camera.get_single() { return; }
    let (camera_cmp, camera_gtf) = q_camera.single();

    if let Some(position) = window.cursor_position()
    .and_then(|cursor| camera_cmp.viewport_to_world_2d(camera_gtf, cursor))
    {
        cursor_position.0 = position;
    }
}