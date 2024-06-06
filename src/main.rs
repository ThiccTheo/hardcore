mod animation;
mod combat;
mod game_state;
mod iframes;
mod level;
mod main_camera;
mod mouse_position;
mod physics;
mod player;
mod skeleton;
mod slime;
mod sprite_flip;
mod tile;
mod ui;

use {
    animation::AnimationPlugin,
    bevy::{
        diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
        prelude::*,
        window::{PresentMode, WindowMode, WindowResolution},
    },
    bevy_ecs_tilemap::TilemapPlugin,
    bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter},
    bevy_rapier2d::prelude::*,
    combat::CombatPlugin,
    game_state::GameState,
    iframes::IframesPlugin,
    leafwing_input_manager::prelude::*,
    level::LevelPlugin,
    main_camera::MainCameraPlugin,
    mouse_position::MousePositionPlugin,
    physics::PhysicsPlugin,
    player::{PlayerAction, PlayerPlugin},
    skeleton::SkeletonPlugin,
    slime::SlimePlugin,
    sprite_flip::SpriteFlipPlugin,
    tile::TilePlugin,
    ui::UiPlugin,
};

fn main() {
    App::new()
        .init_state::<GameState>()
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
                LogDiagnosticsPlugin::default(),
                FramepacePlugin,
                RapierPhysicsPlugin::<NoUserData>::default(),
                RapierDebugRenderPlugin::default(),
                InputManagerPlugin::<PlayerAction>::default(),
                TilemapPlugin,
            ),
            (
                MousePositionPlugin,
                SpriteFlipPlugin,
                AnimationPlugin,
                PhysicsPlugin,
                MainCameraPlugin,
                LevelPlugin,
                TilePlugin,
                PlayerPlugin,
                SkeletonPlugin,
                SlimePlugin,
                CombatPlugin,
                IframesPlugin,
                UiPlugin,
            ),
        ))
        .add_systems(PostStartup, cap_fps)
        .run();
}

fn cap_fps(mut fps_settings: ResMut<FramepaceSettings>) {
    fps_settings.limiter = Limiter::from_framerate(15.);
}
