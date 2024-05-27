use {
    super::{
        animation::{self, AnimationIndices, AnimationTimer},
        game_state::GameState,
        level,
        physics::{self, Acceleration, Grounded, NetDirection, TerminalVelocity},
        sprite_flip::Flippable,
    },
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
};

const SKELETON_Z: f32 = 2.;
pub const SKELETON_ID: u8 = SKELETON_Z as u8;

pub struct SkeletonPlugin;

impl Plugin for SkeletonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SkeletonSpawnEvent>()
            .add_systems(
                OnEnter(GameState::Playing),
                on_skeleton_spawn.after(level::signal_entity_spawns),
            )
            .add_systems(
                Update,
                skeleton_animation
                    .before(animation::adjust_sprite_indices)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                FixedUpdate,
                skeleton_movement
                    .after(physics::process_collisions)
                    .before(physics::apply_forces)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
struct Skeleton;

#[derive(Event)]
pub struct SkeletonSpawnEvent {
    pub pos: Vec2,
}

fn on_skeleton_spawn(
    mut skeleton_spawn_evr: EventReader<SkeletonSpawnEvent>,
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut tex_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for &SkeletonSpawnEvent { pos } in skeleton_spawn_evr.read() {
        cmds.spawn((
            Skeleton,
            SpriteSheetBundle {
                texture: asset_server.load("images/skeleton.png"),
                atlas: TextureAtlas {
                    layout: tex_atlas_layouts.add(TextureAtlasLayout::from_grid(
                        Vec2::splat(16.),
                        3,
                        1,
                        None,
                        None,
                    )),
                    index: 0,
                },
                transform: Transform::from_translation(pos.extend(SKELETON_Z)),
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
            NetDirection { x: 1, y: -1 },
            Grounded::default(),
        ));
    }
}

fn skeleton_movement(
    mut skeleton_qry: Query<(&Velocity, &mut NetDirection, &mut Flippable), With<Skeleton>>,
) {
    for (skeleton_vel, mut skeleton_net_dir, mut skeleton_flippable) in skeleton_qry.iter_mut() {
        if skeleton_vel.linvel.x == 0. {
            skeleton_net_dir.x = !skeleton_net_dir.x;
            skeleton_flippable.flip_x = !skeleton_flippable.flip_x;
        }
    }
}

fn skeleton_animation(
    mut skeleton_qry: Query<
        (&mut AnimationIndices, &mut AnimationTimer, &Grounded),
        With<Skeleton>,
    >,
) {
    for (mut skeleton_animation_idxs, mut skeleton_animation_timer, skeleton_grounded) in
        skeleton_qry.iter_mut()
    {
        let (idle_idxs, idle_timer) = (
            AnimationIndices { first: 0, last: 0 },
            AnimationTimer(Timer::from_seconds(0., TimerMode::Repeating)),
        );
        let (walk_idxs, walk_timer) = (
            AnimationIndices { first: 1, last: 2 },
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
        );
        if skeleton_grounded.0 {
            if *skeleton_animation_idxs != walk_idxs {
                *skeleton_animation_idxs = walk_idxs;
                *skeleton_animation_timer = walk_timer;
            }
        } else if *skeleton_animation_idxs != idle_idxs {
            *skeleton_animation_idxs = idle_idxs;
            *skeleton_animation_timer = idle_timer;
        }
    }
}
