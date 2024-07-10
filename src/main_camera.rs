use {
    super::{level::LEVEL_SIZE, player::Player, tile::TILE_SIZE},
    crate::GameState,
    bevy::prelude::*,
};

#[derive(Component)]
pub struct MainCamera;

fn follow_player(
    mut cam_qry: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player_qry: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let mut cam_xform = cam_qry.single_mut();
    let player_xform = player_qry.single();

    cam_xform.translation = player_xform
        .translation
        .truncate()
        .extend(cam_xform.translation.z);
}

fn clamp_camera_to_tilemap(
    mut cam_qry: Query<(&Camera, &OrthographicProjection, &mut Transform), With<MainCamera>>,
) {
    let (cam, cam_proj, mut cam_xform) = cam_qry.single_mut();
    let Some(scaled_viewport_size) = cam
        .logical_viewport_size()
        .map(|logical_viewport_size| logical_viewport_size * cam_proj.scale)
    else {
        return;
    };

    let tilemap_size_px = LEVEL_SIZE * TILE_SIZE;
    let (tilemap_left_px, tilemap_right_px, tilemap_top_px, tilemap_bottom_px) = (
        -tilemap_size_px.x / 2.,
        tilemap_size_px.x / 2.,
        tilemap_size_px.y / 2.,
        -tilemap_size_px.y / 2.,
    );

    if tilemap_size_px.x > scaled_viewport_size.x {
        cam_xform.translation.x = cam_xform.translation.x.clamp(
            tilemap_left_px + scaled_viewport_size.x / 2.,
            tilemap_right_px - scaled_viewport_size.x / 2.,
        );
    }
    if tilemap_size_px.y > scaled_viewport_size.y {
        cam_xform.translation.y = cam_xform.translation.y.clamp(
            tilemap_bottom_px + scaled_viewport_size.y / 2.,
            tilemap_top_px - scaled_viewport_size.y / 2.,
        );
    }
}

// temporary
fn adjust_camera_zoom(
    mut cam_qry: Query<&mut OrthographicProjection, With<MainCamera>>,
    kb: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    let mut cam_proj = cam_qry.single_mut();

    if kb.pressed(KeyCode::NumpadSubtract) {
        cam_proj.scale += dt;
    }
    if kb.pressed(KeyCode::NumpadAdd) {
        cam_proj.scale -= dt;
    }
}

pub fn main_camera_plugin(app: &mut App) {
    app.insert_resource(ClearColor(Color::srgb_u8(208, 187, 148)))
        .add_systems(OnEnter(GameState::Setup), |mut cmds: Commands| {
            cmds.spawn((MainCamera, Camera2dBundle::default()));
        })
        .add_systems(
            Update,
            (adjust_camera_zoom, clamp_camera_to_tilemap)
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            FixedPostUpdate,
            follow_player.run_if(in_state(GameState::Playing)),
        );
}
