mod game_state;
mod level;
mod player;
mod tile;
mod main_camera;

use {
    bevy::prelude::*, bevy_rapier2d::prelude::*, bevy_tnua::prelude::*,
    bevy_tnua_rapier2d::TnuaRapier2dPlugin, game_state::GameState,
    leafwing_input_manager::prelude::*, player::PlayerAction, tile::TILE_SIZE,
    bevy_ecs_tilemap::prelude::*,
};

fn main() {
    App::new()
        .init_state::<GameState>()
        .insert_resource({
            let mut rapier_cfg = RapierConfiguration::new(1.);
            rapier_cfg.timestep_mode = TimestepMode::Fixed {
                dt: Time::<Fixed>::default().timestep().as_secs_f32(),
                substeps: 1,
            };
            rapier_cfg
        })
        .add_plugins((
            (
                DefaultPlugins,
                InputManagerPlugin::<PlayerAction>::default(),
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(TILE_SIZE.x).in_fixed_schedule(),
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
        .run();
}
