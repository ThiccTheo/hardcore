mod animation;
mod combat;
mod game_state;
mod level;
mod main_camera;
mod mouse_position;
mod player;
mod skeleton;
mod slime;
mod sprite_flip;
mod status_effects;
mod tile;

use {
    bevy::{
        diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
        prelude::*,
        window::{PresentMode, WindowMode, WindowResolution},
    },
    bevy_ecs_tilemap::TilemapPlugin,
    bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter},
    bevy_rapier2d::prelude::*,
    game_state::GameState,
    leafwing_input_manager::prelude::*,
    player::PlayerAction,
};

fn main() {
    App::new()
        .init_state::<GameState>()
        .insert_resource({
            let mut rapier_config = RapierConfiguration::default();
            rapier_config.timestep_mode = TimestepMode::Fixed {
                dt: Time::<Fixed>::default().timestep().as_secs_f32(),
                substeps: 1,
            };
            rapier_config
        })
        .add_plugins((
            (
                DefaultPlugins
                    .set(ImagePlugin::default_nearest())
                    .set(WindowPlugin {
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
                FrameTimeDiagnosticsPlugin,
                // LogDiagnosticsPlugin::default(),
                FramepacePlugin,
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.).in_fixed_schedule(),
                RapierDebugRenderPlugin::default(),
                InputManagerPlugin::<PlayerAction>::default(),
                TilemapPlugin,
            ),
            (
                mouse_position::mouse_position_plugin,
                sprite_flip::sprite_flip_plugin,
                animation::animation_plugin,
                main_camera::main_camera_plugin,
                level::level_plugin,
                tile::tile_plugin,
                player::player_plugin,
                //skeleton::skeleton_plugin,
                //slime::slime_plugin,
                combat::combat_plugin,
                status_effects::status_effects_plugin,
            ),
        ))
        .add_systems(PostStartup, cap_fps)
        .run();
}

fn cap_fps(mut fps_settings: ResMut<FramepaceSettings>) {
    fps_settings.limiter = Limiter::from_framerate(64.);
}
