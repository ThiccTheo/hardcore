mod game_state;
mod level;
mod main_camera;
mod player;
mod tile;

use {
    bevy::{
        //diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
        prelude::*,
        window::{PresentMode, WindowMode, WindowResolution},
    }, bevy_ecs_tilemap::prelude::*, bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter}, bevy_rapier2d::prelude::*, bevy_tnua::prelude::*, bevy_tnua_rapier2d::TnuaRapier2dPlugin, game_state::GameState, leafwing_input_manager::prelude::*, player::PlayerAction, tile::TILE_SIZE
};

fn main() {
    App::new()
        .init_state::<GameState>()
        .insert_resource({
            let mut rapier_cfg = RapierConfiguration::new(TILE_SIZE.x);
            rapier_cfg.timestep_mode = TimestepMode::Fixed {
                dt: Time::<Fixed>::default().timestep().as_secs_f32(),
                substeps: 1,
            };
            rapier_cfg
        })
        .add_plugins((
            (
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync,
                        mode: WindowMode::Windowed,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: WindowResolution::new(1280., 720.),
                        title: String::from("Hardcore"),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
                //FrameTimeDiagnosticsPlugin,
                //LogDiagnosticsPlugin::default(),
                FramepacePlugin,
                InputManagerPlugin::<PlayerAction>::default(),
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(TILE_SIZE.x)
                    .in_fixed_schedule(),
                RapierDebugRenderPlugin::default(),
                TnuaRapier2dPlugin::new(FixedUpdate),
                TnuaControllerPlugin::new(FixedUpdate),
                TilemapPlugin,
            ),
            (
                main_camera::main_camera_plugin,
                level::level_plugin,
                player::player_plugin,
                tile::tile_plugin,
            ),
        ))
        .add_systems(PostStartup, cap_fps)
        .run();
}

fn cap_fps(mut fps_settings: ResMut<FramepaceSettings>) {
    fps_settings.limiter = Limiter::from_framerate(500.);
}