use {
    super::{
        animation::{self, AnimationIndices, AnimationTimer},
        game_state::GameState,
        level,
        mouse_position::MousePosition,
        physics::{self, Acceleration, Grounded, NetDirection, TerminalVelocity},
        sprite_flip::Flippable,
    },
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
    leafwing_input_manager::prelude::*,
};

const PLAYER_Z: f32 = 3.;
pub const PLAYER_ID: u8 = PLAYER_Z as u8;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
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
                player_input
                    .after(physics::process_collisions)
                    .before(physics::apply_forces)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component, Default)]
pub struct Player {
    pub can_jump: bool,
    pub is_jumping: bool,
    pub can_attack: bool,
    pub is_attacking: bool,
}

#[derive(Actionlike, Hash, Clone, PartialEq, Eq, Reflect)]
pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    Jump,
    Attack,
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
        Player::default(),
        InputManagerBundle::<PlayerAction> {
            input_map: InputMap::new([
                (PlayerAction::MoveLeft, KeyCode::KeyA),
                (PlayerAction::MoveRight, KeyCode::KeyD),
                (PlayerAction::Jump, KeyCode::Space),
            ])
            .insert(PlayerAction::Attack, MouseButton::Left)
            .clone(),
            ..default()
        },
        SpriteSheetBundle {
            texture: asset_server.load("images/player.png"),
            atlas: TextureAtlas {
                layout: tex_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    Vec2::splat(16.),
                    4,
                    2,
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
        AnimationIndices::default(),
        AnimationTimer(Timer::from_seconds(default(), TimerMode::Repeating)),
        KinematicCharacterController::default(),
        Collider::capsule_y(3.75, 4.),
        Friction::coefficient(3.),
        Velocity::zero(),
        TerminalVelocity(Vec2::new(50., 200.)),
        Acceleration(Vec2::new(300., 500.)),
        NetDirection { x: 0, y: -1 },
        Grounded::default(),
    ));
}

fn player_input(
    mut player_qry: Query<(
        &mut Player,
        &ActionState<PlayerAction>,
        &Transform,
        &mut Velocity,
        &mut NetDirection,
        &mut Grounded,
        &mut Flippable,
    )>,
    mouse_pos: Res<MousePosition>,
) {
    let (
        mut player,
        player_actions,
        player_xform,
        mut player_vel,
        mut player_net_dir,
        mut player_grounded,
        mut player_flippable,
    ) = player_qry.single_mut();

    if player_actions.released(&PlayerAction::MoveLeft)
        && player_actions.released(&PlayerAction::MoveRight)
    {
        player_net_dir.x = 0;
    }
    if player_actions.pressed(&PlayerAction::MoveLeft) {
        player_net_dir.x = -1;
        player_flippable.flip_x = true;
    }
    if player_actions.pressed(&PlayerAction::MoveRight) {
        player_net_dir.x = 1;
        player_flippable.flip_x = false;
    }
    if player.can_attack && player_actions.just_pressed(&PlayerAction::Attack) {
        player_flippable.flip_x = player_xform.translation.x > mouse_pos.x;
        player.is_attacking = true;
        player.can_attack = false;
    }
    if player_grounded.0 {
        player.is_jumping = false;
        if player_actions.just_pressed(&PlayerAction::Jump) {
            player.can_jump = true;
        }
    }
    if player.can_jump {
        player.can_jump = false;
        player.is_jumping = true;
        player_grounded.0 = false;
        player_vel.linvel.y = 200.;
    }
}

fn player_animation(
    mut player_qry: Query<(
        &mut Player,
        &TextureAtlas,
        &mut AnimationIndices,
        &mut AnimationTimer,
        &Grounded,
        &NetDirection,
    )>,
) {
    let (
        mut player,
        player_tex_atlas,
        mut player_animation_idxs,
        mut player_animation_timer,
        player_grounded,
        player_net_dir,
    ) = player_qry.single_mut();

    let (attack_idxs, attack_timer) = (
        AnimationIndices { first: 4, last: 6 },
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
    );
    let (jump_idxs, jump_timer) = (
        AnimationIndices { first: 3, last: 3 },
        AnimationTimer(Timer::from_seconds(0., TimerMode::Repeating)),
    );
    let (walk_idxs, walk_timer) = (
        AnimationIndices { first: 1, last: 2 },
        AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
    );
    let (idle_idxs, idle_timer) = (
        AnimationIndices { first: 0, last: 0 },
        AnimationTimer(Timer::from_seconds(0., TimerMode::Repeating)),
    );

    if player.is_attacking {
        if *player_animation_idxs != attack_idxs {
            *player_animation_idxs = attack_idxs;
            *player_animation_timer = attack_timer;
        } else if player_tex_atlas.index == attack_idxs.last {
            *player_animation_idxs = idle_idxs.clone();
            *player_animation_timer = idle_timer;
            player.is_attacking = false;
        }
    } else if player.is_jumping && !player_grounded.0 {
        if *player_animation_idxs != jump_idxs {
            *player_animation_idxs = jump_idxs;
            *player_animation_timer = jump_timer;
        }
    } else if player_net_dir.x != 0 && player_grounded.0 {
        if *player_animation_idxs != walk_idxs {
            *player_animation_idxs = walk_idxs;
            *player_animation_timer = walk_timer;
        }
    } else if *player_animation_idxs != idle_idxs {
        *player_animation_idxs = idle_idxs.clone();
        *player_animation_timer = idle_timer;
    }

    if *player_animation_idxs == idle_idxs {
        player.can_attack = true;
    }
}
