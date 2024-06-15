mod game_state;
mod player;

use {
    bevy::prelude::*, bevy_rapier2d::prelude::*, bevy_tnua::prelude::*,
    bevy_tnua_rapier2d::TnuaRapier2dPlugin, game_state::GameState,
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
                RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule(),
                RapierDebugRenderPlugin::default(),
                TnuaRapier2dPlugin::new(FixedUpdate),
                TnuaControllerPlugin::new(FixedUpdate),
            ),
            (player::player_plugin),
        ))
        .run();
}
