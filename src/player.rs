use {
    super::game_state::GameState, bevy::prelude::*, bevy_rapier2d::prelude::*,
    bevy_tnua_rapier2d::TnuaRapier2dIOBundle, bevy_tnua::prelude::*,
};

fn on_player_spawn(mut cmds: Commands) {
    cmds.spawn((
        TransformBundle::from_transform(Transform::from_xyz(0., 2., 1.)),
        VisibilityBundle::default(),
        RigidBody::Dynamic,
        Collider::capsule_y(0.5, 0.5),
        TnuaRapier2dIOBundle::default(),
        TnuaControllerBundle::default(),
    ))
    .with_children(|parent| {
        parent.spawn(Camera2dBundle::default());
    });
}

pub fn player_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), on_player_spawn);
}
