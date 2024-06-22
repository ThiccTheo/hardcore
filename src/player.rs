use {
    super::{
        animation::{self, AnimationContext, AnimationIndices, AnimationTimer},
        game_state::GameState,
        level,
        sprite_flip::Flippable,
        tile::TILE_SIZE,
    },
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
    bevy_tnua::{
        builtins::TnuaBuiltinJumpState,
        control_helpers::{TnuaSimpleAirActionsCounter, TnuaSimpleFallThroughPlatformsHelper},
        prelude::*,
        TnuaAnimatingState, TnuaAnimatingStateDirective, TnuaGhostSensor, TnuaProximitySensor,
    },
    bevy_tnua_rapier2d::{TnuaRapier2dIOBundle, TnuaRapier2dSensorShape},
    leafwing_input_manager::prelude::*,
    maplit::hashmap,
    std::{f32::consts::FRAC_PI_4, time::Duration},
};

const PLAYER_Z: f32 = 4.;
pub const PLAYER_ID: u8 = PLAYER_Z as u8;
const PLAYER_COLLIDER_HALF_HEIGHT: f32 = 16.;
const PLAYER_COLLDIER_RADIUS: f32 = 16.;

#[derive(Component)]
pub struct Player;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Reflect)]
pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    Jump,
    DropDown,
    EnterDoor,
}

#[derive(Hash, Eq, PartialEq)]
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

#[derive(Resource)]
struct PlayerAssets {
    tex: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

fn on_player_spawn(
    mut player_spawn_evr: EventReader<PlayerSpawnEvent>,
    mut cmds: Commands,
    player_assets: Res<PlayerAssets>,
) {
    cmds.spawn((
        (
            Player,
            AnimationIndices::default(),
            AnimationTimer::default(),
            Flippable::default(),
        ),
        SpriteSheetBundle {
            texture: player_assets.tex.clone_weak(),
            atlas: TextureAtlas {
                layout: player_assets.layout.clone_weak(),
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
            (PlayerAction::DropDown, KeyCode::KeyS),
            (PlayerAction::EnterDoor, KeyCode::KeyW),
        ])),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::capsule_y(PLAYER_COLLIDER_HALF_HEIGHT, PLAYER_COLLDIER_RADIUS),
        TnuaRapier2dIOBundle::default(),
        TnuaControllerBundle::default(),
        TnuaSimpleAirActionsCounter::default(),
        TnuaSimpleFallThroughPlatformsHelper::default(),
        TnuaGhostSensor::default(),
        TnuaRapier2dSensorShape(Collider::cuboid(PLAYER_COLLDIER_RADIUS - 2., 0.)),
        TnuaAnimatingState::<PlayerAnimation>::default(),
    ));
}

fn player_movement(
    mut player_qry: Query<(
        &ActionState<PlayerAction>,
        &mut TnuaController,
        &mut TnuaSimpleAirActionsCounter,
        &mut Flippable,
        &mut TnuaSimpleFallThroughPlatformsHelper,
        &mut TnuaProximitySensor,
        &TnuaGhostSensor,
        &AnimationIndices,
    )>,
    player_animation_ctx: Res<AnimationContext<PlayerAnimation>>,
) {
    let (
        player_in,
        mut player_kcc,
        mut player_air_actions_count,
        mut player_flippable,
        mut player_ghost_platforms_helper,
        mut player_prox_sensor,
        player_ghost_sensor,
        player_animation_idxs,
    ) = player_qry.single_mut();
    player_air_actions_count.update(&player_kcc);

    player_kcc.basis(TnuaBuiltinWalk {
        max_slope: FRAC_PI_4,
        spring_dampening: 0.5,
        float_height: PLAYER_COLLIDER_HALF_HEIGHT + PLAYER_COLLDIER_RADIUS + 14.,
        air_acceleration: 2. * TILE_SIZE.x,
        acceleration: 2. * TILE_SIZE.x,
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

    let mut ghost_platforms_handle = player_ghost_platforms_helper.with(
        &mut player_prox_sensor,
        player_ghost_sensor,
        PLAYER_COLLIDER_HALF_HEIGHT + PLAYER_COLLDIER_RADIUS,
    );

    if player_in.pressed(&PlayerAction::DropDown) {
        ghost_platforms_handle.try_falling(true);
    } else if *player_animation_idxs
        != player_animation_ctx
            .get(&PlayerAnimation::Jumping)
            .unwrap()
            .0
    {
        ghost_platforms_handle.dont_fall();
    }
}

fn player_animation(
    mut player_qry: Query<(
        &mut TnuaAnimatingState<PlayerAnimation>,
        &TnuaController,
        &mut AnimationIndices,
        &mut AnimationTimer,
    )>,
    player_animation_ctx: Res<AnimationContext<PlayerAnimation>>,
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
            (*player_animation_idxs, *player_animation_timer) =
                player_animation_ctx.get(state).unwrap().clone()
        }
    }
}

pub fn player_plugin(app: &mut App) {
    app.add_event::<PlayerSpawnEvent>()
        .insert_resource(AnimationContext::<PlayerAnimation>(hashmap! {
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
            )
        }))
        .add_systems(
            Startup,
            |mut cmds: Commands,
             asset_server: Res<AssetServer>,
             mut tex_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>| {
                cmds.insert_resource(PlayerAssets {
                    tex: asset_server.load("player.png"),
                    layout: tex_atlas_layouts.add(TextureAtlasLayout::from_grid(
                        Vec2::new(80., 110.),
                        9,
                        3,
                        None,
                        None,
                    )),
                });
            },
        )
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
