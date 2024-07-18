use {
    super::main_camera::MainCamera,
    bevy::{prelude::*, window::PrimaryWindow},
};

#[derive(Resource, Default)]
pub struct MousePosition(Vec2);

impl MousePosition {
    pub fn x(&self) -> f32 {
        self.0.x
    }

    pub fn y(&self) -> f32 {
        self.0.y
    }

    pub fn as_vec(&self) -> Vec2 {
        self.0
    }
}

fn update_mouse_position(
    mut old_mouse_pos: ResMut<MousePosition>,
    win_qry: Query<&Window, With<PrimaryWindow>>,
    cam_qry: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let win = win_qry.single();
    let (cam, cam_glob_xform) = cam_qry.single();

    let Some(new_mouse_pos) = win
        .cursor_position()
        .and_then(|mouse_pos| cam.viewport_to_world_2d(cam_glob_xform, mouse_pos))
    else {
        return;
    };
    old_mouse_pos.0 = new_mouse_pos;
}

pub fn mouse_position_plugin(app: &mut App) {
    app.insert_resource(MousePosition::default())
        .add_systems(Update, update_mouse_position);
}
