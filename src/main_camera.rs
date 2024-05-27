use {
    super::{game_state::GameState, level::LEVEL_SIZE, player::Player, tile::TILE_SIZE},
    bevy::prelude::*,
};

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb_u8(208, 187, 148)))
            .add_systems(Startup, spawn_main_camera)
            .add_systems(
                Update,
                clamp_camera_to_tilemap.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                FixedPostUpdate,
                follow_player.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_main_camera(mut cmds: Commands) {
    let mut cam = Camera2dBundle::default();
    cam.projection.scale /= 4.;
    cmds.spawn((MainCamera, cam));
}

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
    let Some(cam_viewport) = cam.logical_viewport_size() else {
        return;
    };

    let tilemap_size_px = LEVEL_SIZE * TILE_SIZE;
    let clamp_correction = Vec2::new(
        if LEVEL_SIZE.x as usize % 2 == 0 {
            0.
        } else {
            TILE_SIZE.x / 2.
        },
        if LEVEL_SIZE.y as usize % 2 == 0 {
            0.
        } else {
            TILE_SIZE.y / 2.
        },
    );
    let (tilemap_left_px, tilemap_right_px, tilemap_top_px, tilemap_bottom_px) = (
        -tilemap_size_px.x / 2. - clamp_correction.x,
        tilemap_size_px.x / 2. + clamp_correction.x,
        tilemap_size_px.y / 2. + clamp_correction.y,
        -tilemap_size_px.y / 2. - clamp_correction.y,
    );

    let scale_correction = 2.
        * if cam_proj.scale < 1. {
            cam_proj.scale.recip()
        } else {
            cam_proj.scale
        };

    if (tilemap_size_px.x / 2.) > (cam_viewport.x / scale_correction) {
        cam_xform.translation.x = cam_xform.translation.x.clamp(
            tilemap_left_px + cam_viewport.x / scale_correction,
            tilemap_right_px - cam_viewport.x / scale_correction,
        );
    }
    if (tilemap_size_px.y / 2.) > (cam_viewport.y / scale_correction) {
        cam_xform.translation.y = cam_xform.translation.y.clamp(
            tilemap_bottom_px + cam_viewport.y / scale_correction,
            tilemap_top_px - cam_viewport.y / scale_correction,
        );
    }
}
