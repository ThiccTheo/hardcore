use {
    super::{
        animation::{self, AnimationIndices, AnimationTimer},
        game_state::GameState,
        mouse_position::MousePosition,
        physics::{self, Acceleration, Grounded, NetDirection, TerminalVelocity},
        sprite_flip::Flippable,
    },
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
    leafwing_input_manager::prelude::*,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    discrete_player_input,
                    update_player_animation.before(animation::adjust_sprite_indices),
                )
                    .distributive_run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                FixedUpdate,
                continuous_player_input
                    .after(physics::process_collisions)
                    .before(physics::apply_forces)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component, Default)]
pub struct Player {
    pub can_jump: bool,
}

#[derive(Actionlike, Hash, Clone, PartialEq, Eq, Reflect)]
pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    Jump,
    Attack,
}

fn spawn_player(
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
            texture: asset_server.load("player.png"),
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
            ..default()
        },
        Flippable::default(),
        AnimationIndices::default(),
        AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
        KinematicCharacterController::default(),
        Collider::capsule_y(4., 4.),
        Friction::coefficient(3.),
        Velocity::zero(),
        TerminalVelocity(Vec2::new(50., 200.)),
        Acceleration(Vec2::new(300., 500.)),
        NetDirection { x: 0, y: -1 },
        Grounded::default(),
    ))
    .with_children(|parent| {
        let mut cam = Camera2dBundle::default();
        cam.projection.scale /= 4.;
        parent.spawn(cam);
    });

    // TEMPORARY TEMPORARY TEMPORARY TEMPORARY
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(300., 16.)),
                ..default()
            },
            transform: Transform::from_xyz(0., -30., 1.),
            ..default()
        },
        Collider::cuboid(150., 8.),
    ));
}

fn discrete_player_input(
    mut player_qry: Query<(
        &mut Player,
        &Transform,
        &ActionState<PlayerAction>,
        &Grounded,
        &mut Flippable,
    )>,
    mouse_pos: Res<MousePosition>,
) {
    let (mut player, player_xform, player_actions, player_grounded, mut player_flippable) =
        player_qry.single_mut();

    if player_actions.just_pressed(&PlayerAction::Jump) && player_grounded.0 {
        player.can_jump = true;
    }
    if player_actions.just_pressed(&PlayerAction::Attack) {
        player_flippable.flip_x = player_xform.translation.x > mouse_pos.x;
    }
}

fn continuous_player_input(
    mut player_qry: Query<(
        &mut Player,
        &ActionState<PlayerAction>,
        &mut Velocity,
        &mut NetDirection,
        &mut Grounded,
        &mut Flippable,
    )>,
) {
    let (
        mut player,
        player_actions,
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
    if player.can_jump {
        player.can_jump = false;
        player_grounded.0 = false;
        player_vel.linvel.y = 200.;
    }
}

fn update_player_animation(
    mut player_qry: Query<
        (
            &mut AnimationIndices,
            &TextureAtlas,
            &Grounded,
            &NetDirection,
            &ActionState<PlayerAction>,
        ),
        With<Player>,
    >,
) {
    let (
        mut player_animation_indices,
        player_tex_atlas,
        player_grounded,
        player_net_dir,
        player_actions,
    ) = player_qry.single_mut();

    let jumping = AnimationIndices { first: 3, last: 3 };
    let walking = AnimationIndices { first: 1, last: 2 };
    let idling = AnimationIndices { first: 0, last: 0 };
    let attacking = AnimationIndices { first: 4, last: 5 };

    let attack_in_progress = *player_animation_indices == attacking
        && player_animation_indices.last != player_tex_atlas.index;

    if player_actions.pressed(&PlayerAction::Attack) {
        if *player_animation_indices != attacking {
            *player_animation_indices = attacking;
        }
    } else if !player_grounded.0 {
        if *player_animation_indices != jumping && !attack_in_progress {
            *player_animation_indices = jumping;
        }
    } else if player_net_dir.x != 0 {
        if *player_animation_indices != walking && !attack_in_progress {
            *player_animation_indices = walking;
        }
    } else if *player_animation_indices != idling && !attack_in_progress {
        *player_animation_indices = idling;
    }
}
