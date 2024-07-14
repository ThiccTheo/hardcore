mod animation;
mod asset_owner;
mod combat;
mod door;
mod level;
mod main_camera;
mod mouse_position;
mod player;
mod spike;
mod tile;
mod ui;
mod sprite_flip;

use {
    bevy::{
        //diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
        prelude::*,
        window::{PresentMode, WindowMode},
    },
    bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter},
    bevy_rapier2d::prelude::*,
    bevy_tnua::prelude::*,
    bevy_tnua_rapier2d::TnuaRapier2dPlugin,
    leafwing_input_manager::prelude::*,
    player::PlayerAction,
    static_assertions::const_assert,
    tile::TILE_SIZE,
};

const RESOLUTION: Vec2 = Vec2::new(1280., 720.);

const_assert!(RESOLUTION.x / RESOLUTION.y == 16. / 9.);

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum GameState {
    #[default]
    Setup,
    Playing,
    Transition,
}

fn main() {
    App::new()
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
                        resolution: RESOLUTION.into(),
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
                //RapierDebugRenderPlugin::default(),
                TnuaRapier2dPlugin::new(FixedUpdate),
                TnuaControllerPlugin::new(FixedUpdate),
            ),
            (
                main_camera::main_camera_plugin,
                mouse_position::mouse_position_plugin,
                animation::animation_plugin,
                sprite_flip::sprite_flip_plugin,
                ui::ui_plugin,
                combat::combat_plugin,
                level::level_plugin,
                player::player_plugin,
                tile::tile_plugin,
                door::door_plugin,
                spike::spike_plugin,
            ),
        ))
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .add_systems(
            PostStartup,
            |mut fps_settings: ResMut<FramepaceSettings>| {
                fps_settings.limiter = Limiter::from_framerate(500.)
            },
        )
        .add_systems(
            Update,
            (|mut next_state: ResMut<NextState<GameState>>| {
                next_state.set(GameState::Playing);
            })
            .run_if(in_state(GameState::Setup)),
        )
        .add_systems(
            OnTransition {
                entered: GameState::Transition,
                exited: GameState::Playing,
            },
            |mut next_state: ResMut<NextState<GameState>>| next_state.set(GameState::Playing),
        )
        .run();
}
