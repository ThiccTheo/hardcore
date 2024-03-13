use bevy::{prelude::*, window::PrimaryWindow};

pub struct MousePositionPlugin;

impl Plugin for MousePositionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MousePosition::default())
            .add_systems(Update, update_mouse_position);
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct MousePosition(pub Vec2);

fn update_mouse_position(
    mut old_mouse_pos: ResMut<MousePosition>,
    primary_win_qry: Query<&Window, With<PrimaryWindow>>,
    cam_qry: Query<(&Camera, &GlobalTransform)>,
) {
    let primary_win = primary_win_qry.single();
    let (cam, cam_glob_xform) = cam_qry.single();

    let Some(new_mouse_pos) = primary_win
        .cursor_position()
        .and_then(|mouse_pos| cam.viewport_to_world_2d(cam_glob_xform, mouse_pos))
    else {
        return;
    };
    old_mouse_pos.0 = new_mouse_pos;
}
