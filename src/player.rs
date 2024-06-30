use {
    super::{
        animation::{self, AnimationIndices, AnimationState, AnimationTimer},
        combat::Health,
        door::Door,
        game_state::{GameState, PlayingEntity},
        level,
        sprite_flip::Flippable,
        texture_atlas_owner::TextureAtlasOwner,
        tile::{TILE_SIZE, TILE_Z},
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
    std::{f32::consts::FRAC_PI_4, time::Duration},
};

const PLAYER_Z: f32 = TILE_Z + 2.;
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

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum PlayerAnimation {
    Idling,
    Running,
    Jumping,
    Falling,
}

impl AnimationState for PlayerAnimation {
    fn indices(self) -> AnimationIndices {
        match self {
            PlayerAnimation::Idling => AnimationIndices { first: 0, last: 0 },
            PlayerAnimation::Running => AnimationIndices { first: 9, last: 10 },
            PlayerAnimation::Jumping => AnimationIndices { first: 1, last: 1 },
            PlayerAnimation::Falling => AnimationIndices { first: 2, last: 2 },
        }
    }

    fn timer(self) -> AnimationTimer {
        match self {
            PlayerAnimation::Idling => AnimationTimer::zero(),
            PlayerAnimation::Running => AnimationTimer::new(Duration::from_secs_f32(3f32.recip())),
            PlayerAnimation::Jumping => AnimationTimer::zero(),
            PlayerAnimation::Falling => AnimationTimer::zero(),
        }
    }
}

#[derive(Event)]
pub struct PlayerSpawnEvent {
    pub pos: Vec2,
}

fn on_player_spawn(
    mut player_spawn_evr: EventReader<PlayerSpawnEvent>,
    mut cmds: Commands,
    player_assets: Res<TextureAtlasOwner<Player>>,
) {
    cmds.spawn((
        (
            Player,
            PlayingEntity,
            AnimationIndices::default(),
            AnimationTimer::default(),
            Flippable::default(),
            Health(5),
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
            (PlayerAction::Jump, KeyCode::KeyW),
            (PlayerAction::DropDown, KeyCode::KeyS),
            (PlayerAction::EnterDoor, KeyCode::Space),
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
    mut player_qry: Query<
        (
            Entity,
            &ActionState<PlayerAction>,
            &mut TnuaController,
            &mut TnuaSimpleAirActionsCounter,
            &mut Flippable,
            &mut TnuaSimpleFallThroughPlatformsHelper,
            &mut TnuaProximitySensor,
            &TnuaGhostSensor,
            &AnimationIndices,
        ),
        With<Player>,
    >,
    door_qry: Query<(&Door, Entity), (With<Collider>, With<Sensor>)>,
    rapier_ctx: Res<RapierContext>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (
        player_id,
        player_in,
        mut player_kcc,
        mut player_air_actions_count,
        mut player_flippable,
        mut player_ghost_platforms_helper,
        mut player_prox_sensor,
        player_ghost_sensor,
        player_animation_idxs,
    ) = player_qry.single_mut();

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

    player_air_actions_count.update(&player_kcc);

    if player_in.pressed(&PlayerAction::Jump) {
        player_kcc.action(TnuaBuiltinJump {
            height: TILE_SIZE.y * 1.5,
            allow_in_air: player_air_actions_count.air_count_for(TnuaBuiltinJump::NAME) < 2,
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
    } else if *player_animation_idxs != PlayerAnimation::Jumping.indices() {
        ghost_platforms_handle.dont_fall();
    }

    if player_in.pressed(&PlayerAction::EnterDoor)
        && rapier_ctx.intersection_pair(
            player_id,
            door_qry
                .iter()
                .filter(|(&door, _)| door == Door::Exit)
                .map(|(_, door_id)| door_id)
                .next()
                .unwrap(),
        ) == Some(true)
    {
        next_state.set(GameState::Transition);
    }
}

fn player_animation(
    mut player_qry: Query<
        (
            &mut TnuaAnimatingState<PlayerAnimation>,
            &TnuaController,
            &mut AnimationIndices,
            &mut AnimationTimer,
        ),
        With<Player>,
    >,
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
            (*player_animation_idxs, *player_animation_timer) = (state.indices(), state.timer());
        }
    }
}

pub fn player_plugin(app: &mut App) {
    app.add_event::<PlayerSpawnEvent>()
        .add_systems(
            Startup,
            |mut cmds: Commands,
             asset_server: Res<AssetServer>,
             mut tex_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>| {
                cmds.insert_resource(TextureAtlasOwner::<Player>::new(
                    asset_server.load("player.png"),
                    tex_atlas_layouts.add(TextureAtlasLayout::from_grid(
                        Vec2::new(80., 110.),
                        9,
                        3,
                        None,
                        None,
                    )),
                ));
            },
        )
        .add_systems(
            OnEnter(GameState::Playing),
            on_player_spawn.after(level::signal_level_object_spawns),
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
