use {
    super::{
        animation::{self, AnimationIndices, AnimationTimer},
        game_state::GameState,
        level,
        mouse_position::MousePosition,
        sprite_flip::Flippable,
        status_effects::IsGrounded,
    },
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
    leafwing_input_manager::prelude::*,
};

const PLAYER_Z: f32 = 4.;
pub const PLAYER_ID: u8 = PLAYER_Z as u8;

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
        Collider::capsule_y(3.75, 4.),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        ExternalImpulse::default(),
        IsGrounded::default(),
    ));
}

fn player_movement(
    mut player_qry: Query<(
        &mut Player,
        &ActionState<PlayerAction>,
        &Transform,
        &mut ExternalImpulse,
        &mut Flippable,
        &mut IsGrounded,
    )>,
    mouse_pos: Res<MousePosition>,
) {
    let (mut player, player_actions, player_xform, mut player_ext_impulse, mut player_flippable, mut player_is_grounded) =
        player_qry.single_mut();
    if player_actions.pressed(&PlayerAction::MoveLeft) {
        player_ext_impulse.impulse.x = -1.;
        player_flippable.flip_x = true;
    }
    if player_actions.pressed(&PlayerAction::MoveRight) {
        player_ext_impulse.impulse.x = 1.;
        player_flippable.flip_x = false;
    }
    if player.can_attack && player_actions.just_pressed(&PlayerAction::Attack) {
        player_flippable.flip_x = player_xform.translation.x > mouse_pos.x;
        player.is_attacking = true;
        player.can_attack = false;
    }
    if player_is_grounded.0 {
        player.is_jumping = false;
        if player_actions.just_pressed(&PlayerAction::Jump) {
            player.can_jump = true;
        }
    }
    if player.can_jump {
        player.can_jump = false;
        player.is_jumping = true;
        player_is_grounded.0 = false;
        player_ext_impulse.impulse.y = 10.;
    }
}

fn player_animation(
    mut player_qry: Query<(
        &mut Player,
        &TextureAtlas,
        &mut AnimationIndices,
        &mut AnimationTimer,
        &IsGrounded,
    )>,
) {
    let (
        mut player,
        player_tex_atlas,
        mut player_animation_idxs,
        mut player_animation_timer,
        player_is_grounded,
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
    } else if player.is_jumping && !player_is_grounded.0 {
        if *player_animation_idxs != jump_idxs {
            *player_animation_idxs = jump_idxs;
            *player_animation_timer = jump_timer;
        }
    } /*else if player_net_dir.x != 0 && player_is_grounded.0 {
        if *player_animation_idxs != walk_idxs {
            *player_animation_idxs = walk_idxs;
            *player_animation_timer = walk_timer;
        }
    }*/ else if *player_animation_idxs != idle_idxs {
        *player_animation_idxs = idle_idxs.clone();
        *player_animation_timer = idle_timer;
    }

    if *player_animation_idxs == idle_idxs {
        player.can_attack = true;
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
            player_movement.run_if(in_state(GameState::Playing)),
        );
}
