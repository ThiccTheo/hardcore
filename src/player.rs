use {
    super::{game_state::GameState, level, tile::TILE_SIZE},
    bevy::{math::swizzles, prelude::*},
    bevy_rapier2d::prelude::*,
    bevy_tnua::{control_helpers::TnuaSimpleAirActionsCounter, prelude::*, TnuaGhostSensor},
    bevy_tnua_rapier2d::{TnuaRapier2dIOBundle, TnuaRapier2dSensorShape},
    leafwing_input_manager::prelude::*,
    std::f32::consts::FRAC_PI_4,
};

const PLAYER_Z: f32 = 4.;
pub const PLAYER_ID: u8 = PLAYER_Z as u8;

#[derive(Component)]
pub struct Player;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Reflect)]
pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    Jump,
}

#[derive(Event)]
pub struct PlayerSpawnEvent {
    pub pos: Vec2,
}

fn on_player_spawn(
    mut player_spawn_evr: EventReader<PlayerSpawnEvent>,
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut tex_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    cmds.spawn((
        Player,
        SpriteSheetBundle {
            texture: asset_server.load("player.png"),
            atlas: TextureAtlas {
                layout: tex_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    Vec2::new(80., 110.),
                    9,
                    3,
                    None,
                    None,
                )),
                index: 0,
            },
            transform: Transform::from_translation(
                player_spawn_evr.read().next().unwrap().pos.extend(PLAYER_Z),
            ),
            ..default()
        },
        InputManagerBundle::with_map(InputMap::new([
            (PlayerAction::MoveLeft, KeyCode::KeyA),
            (PlayerAction::MoveRight, KeyCode::KeyD),
            (PlayerAction::Jump, KeyCode::Space),
        ])),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::capsule_y(16., 16.),
        TnuaRapier2dIOBundle::default(),
        TnuaControllerBundle::default(),
        TnuaSimpleAirActionsCounter::default(),
        TnuaGhostSensor::default(),
        TnuaRapier2dSensorShape(Collider::cuboid(14., 0.)),
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
        max_slope: FRAC_PI_4,
        spring_dampening: 0.5,
        float_height: 55.,
        desired_velocity: 3.
            * TILE_SIZE.x
            * if player_in.pressed(&PlayerAction::MoveLeft) {
                -Vec3::X
            } else if player_in.pressed(&PlayerAction::MoveRight) {
                Vec3::X
            } else {
                Vec3::ZERO
            },
        ..default()
    });

    if player_in.pressed(&PlayerAction::Jump) {
        player_kcc.action(TnuaBuiltinJump {
            height: TILE_SIZE.y * 1.5,
            allow_in_air: false,
            shorten_extra_gravity: 0.,
            ..default()
        });
    }
}

pub fn player_plugin(app: &mut App) {
    app.add_event::<PlayerSpawnEvent>()
        .add_systems(
            OnEnter(GameState::Playing),
            on_player_spawn.after(level::signal_entity_spawns),
        )
        .add_systems(
            FixedUpdate,
            player_movement
                .in_set(TnuaUserControlsSystemSet)
                .run_if(in_state(GameState::Playing)),
        );
}
