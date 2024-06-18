use {
    super::{
        animation::{self, AnimationIndices, AnimationTimer},
        game_state::GameState,
        level,
        sprite_flip::Flippable,
        tile::TILE_SIZE,
    },
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
    bevy_tnua::{
        builtins::TnuaBuiltinJumpState, control_helpers::TnuaSimpleAirActionsCounter, prelude::*,
        TnuaAnimatingState, TnuaAnimatingStateDirective, TnuaGhostSensor,
    },
    bevy_tnua_rapier2d::{TnuaRapier2dIOBundle, TnuaRapier2dSensorShape},
    leafwing_input_manager::prelude::*,
    std::{f32::consts::FRAC_PI_4, time::Duration},
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

enum PlayerAnimation {
    Idling,
    Running,
    Jumping,
    Falling,
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
        Flippable::default(),
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
        TnuaAnimatingState::<PlayerAnimation>::default(),
        AnimationIndices::default(),
        AnimationTimer::default(),
    ));
}

fn player_movement(
    mut player_qry: Query<(
        &ActionState<PlayerAction>,
        &mut TnuaController,
        &mut TnuaSimpleAirActionsCounter,
        &mut Flippable,
    )>,
) {
    let (player_in, mut player_kcc, mut player_air_actions_count, mut player_flippable) =
        player_qry.single_mut();
    player_air_actions_count.update(&player_kcc);

    player_kcc.basis(TnuaBuiltinWalk {
        max_slope: FRAC_PI_4,
        spring_dampening: 0.5,
        float_height: 50.,
        desired_velocity: 3.
            * TILE_SIZE.x
            * if player_in.pressed(&PlayerAction::MoveLeft)
                && player_in.released(&PlayerAction::MoveRight)
            {
                player_flippable.flip_x = true;
                -Vec3::X
            } else if player_in.pressed(&PlayerAction::MoveRight)
                && player_in.released(&PlayerAction::MoveLeft)
            {
                player_flippable.flip_x = false;
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

fn player_animation(
    mut player_qry: Query<(
        &mut TnuaAnimatingState<PlayerAnimation>,
        &TnuaController,
        &mut AnimationIndices,
        &mut AnimationTimer,
    )>,
) {
    let (
        mut player_animating_state,
        player_kcc,
        mut player_animation_idxs,
        mut player_animation_timer,
    ) = player_qry.single_mut();

    match player_animating_state.update_by_discriminant({
        match player_kcc.action_name() {
            Some(TnuaBuiltinJump::NAME) => {
                match player_kcc.concrete_action::<TnuaBuiltinJump>().unwrap().1 {
                    TnuaBuiltinJumpState::NoJump => return,
                    TnuaBuiltinJumpState::StartingJump { .. }
                    | TnuaBuiltinJumpState::SlowDownTooFastSlopeJump { .. }
                    | TnuaBuiltinJumpState::MaintainingJump
                    | TnuaBuiltinJumpState::StoppedMaintainingJump => PlayerAnimation::Jumping,
                    TnuaBuiltinJumpState::FallSection => PlayerAnimation::Falling,
                }
            }
            _ => {
                let Some((_, basis_state)) = player_kcc.concrete_basis::<TnuaBuiltinWalk>() else {
                    return;
                };
                if basis_state.standing_on_entity().is_none() {
                    PlayerAnimation::Falling
                } else if basis_state.running_velocity.x.abs() > 0. {
                    PlayerAnimation::Running
                } else {
                    PlayerAnimation::Idling
                }
            }
        }
    }) {
        TnuaAnimatingStateDirective::Maintain { .. } => (),
        TnuaAnimatingStateDirective::Alter { state, .. } => {
            (*player_animation_idxs, *player_animation_timer) = match state {
                PlayerAnimation::Idling => (
                    AnimationIndices { first: 0, last: 0 },
                    AnimationTimer::new(Duration::from_secs_f32(0.)),
                ),
                PlayerAnimation::Running => (
                    AnimationIndices { first: 9, last: 10 },
                    AnimationTimer::new(Duration::from_secs_f32(3f32.recip())),
                ),
                PlayerAnimation::Jumping => (
                    AnimationIndices { first: 1, last: 1 },
                    AnimationTimer::new(Duration::from_secs_f32(0.)),
                ),
                PlayerAnimation::Falling => (
                    AnimationIndices { first: 2, last: 2 },
                    AnimationTimer::new(Duration::from_secs_f32(0.)),
                ),
            }
        }
    }
}

pub fn player_plugin(app: &mut App) {
    app.add_event::<PlayerSpawnEvent>()
        .add_systems(
            OnEnter(GameState::Playing),
            on_player_spawn.after(level::signal_entity_spawns),
        )
        .add_systems(
            Update,
            player_animation
                .before(animation::adjust_sprite_indices)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            FixedUpdate,
            player_movement
                .in_set(TnuaUserControlsSystemSet)
                .run_if(in_state(GameState::Playing)),
        );
}
