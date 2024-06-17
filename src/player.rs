use {
    super::game_state::GameState,
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
    bevy_tnua::{control_helpers::TnuaSimpleAirActionsCounter, prelude::*, TnuaGhostSensor},
    bevy_tnua_rapier2d::{TnuaRapier2dIOBundle, TnuaRapier2dSensorShape},
    leafwing_input_manager::prelude::*,
};

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Reflect)]
pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    Jump,
}

fn on_player_spawn(mut cmds: Commands) {
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::splat(32.)),
                ..default()
            },
            ..default()
        },
        InputManagerBundle::with_map(InputMap::new([
            (PlayerAction::MoveLeft, KeyCode::KeyA),
            (PlayerAction::MoveRight, KeyCode::KeyD),
            (PlayerAction::Jump, KeyCode::Space),
        ])),
        RigidBody::Dynamic,
        Collider::cuboid(16., 16.),
        TnuaRapier2dIOBundle::default(),
        TnuaControllerBundle::default(),
        TnuaSimpleAirActionsCounter::default(),
        TnuaGhostSensor::default(),
        TnuaRapier2dSensorShape(Collider::cuboid(16., 0.5)),
    ))
    .with_children(|parent| {
        parent.spawn(Camera2dBundle::default());
    });

    cmds.spawn((
        Collider::cuboid(100., 100.),
        RigidBody::Fixed,
        TransformBundle::from_transform(Transform::from_xyz(0., -500., 0.)),
    ));
}

fn player_movement(
    mut player_qry: Query<(
        &ActionState<PlayerAction>,
        &mut TnuaController,
        &mut TnuaSimpleAirActionsCounter,
    )>,
) {
    let (player_in, mut player_kcc, mut player_air_actions_count) = player_qry.single_mut();
    player_air_actions_count.update(&player_kcc);

    player_kcc.basis(TnuaBuiltinWalk {
        float_height: 30.,
        desired_velocity: (if player_in.pressed(&PlayerAction::MoveLeft) {
            -Vec2::X * 10.
        } else if player_in.pressed(&PlayerAction::MoveRight) {
            Vec2::X * 10.
        } else {
            Vec2::ZERO
        })
        .extend(0.),
        ..default()
    });

    if player_in.pressed(&PlayerAction::Jump) {
        player_kcc.action(TnuaBuiltinJump {
            height: 100.,
            allow_in_air: false,
            shorten_extra_gravity: 0.,
            ..default()
        });
    }
}

pub fn player_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), on_player_spawn)
        .add_systems(
            FixedUpdate,
            player_movement
                .in_set(TnuaUserControlsSystemSet)
                .run_if(in_state(GameState::Playing)),
        );
}
