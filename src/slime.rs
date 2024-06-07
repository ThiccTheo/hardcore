// use {
//     super::{
//         animation::{self, AnimationIndices, AnimationTimer},
//         game_state::GameState,
//         level,
//         sprite_flip::Flippable,
//     },
//     bevy::prelude::*,
//     bevy_rapier2d::prelude::*,
// };

// const SLIME_Z: f32 = 3.;
// pub const SLIME_ID: u8 = SLIME_Z as u8;

// pub struct SlimePlugin;

// impl Plugin for SlimePlugin {
//     fn build(&self, app: &mut App) {
//         app.add_event::<SlimeSpawnEvent>()
//             .add_systems(
//                 OnEnter(GameState::Playing),
//                 on_slime_spawn.after(level::signal_entity_spawns),
//             )
//             .add_systems(
//                 Update,
//                 slime_animation
//                     .before(animation::adjust_sprite_indices)
//                     .run_if(in_state(GameState::Playing)),
//             )
//             .add_systems(
//                 FixedUpdate,
//                 slime_movement
//                     .after(physics::process_collisions)
//                     .before(physics::apply_forces)
//                     .run_if(in_state(GameState::Playing)),
//             );
//     }
// }

// #[derive(Component, Default)]
// struct Slime {
//     can_jump: bool,
// }

// #[derive(Event)]
// pub struct SlimeSpawnEvent {
//     pub pos: Vec2,
// }

// fn on_slime_spawn(
//     mut slime_spawn_evr: EventReader<SlimeSpawnEvent>,
//     mut cmds: Commands,
//     asset_server: Res<AssetServer>,
//     mut tex_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
// ) {
//     for &SlimeSpawnEvent { pos } in slime_spawn_evr.read() {
//         cmds.spawn((
//             Slime::default(),
//             SpriteSheetBundle {
//                 texture: asset_server.load("images/slime.png"),
//                 atlas: TextureAtlas {
//                     layout: tex_atlas_layouts.add(TextureAtlasLayout::from_grid(
//                         Vec2::splat(16.),
//                         4,
//                         1,
//                         None,
//                         None,
//                     )),
//                     index: 0,
//                 },
//                 transform: Transform::from_translation(pos.extend(SLIME_Z)),
//                 ..default()
//             },
//             Flippable::default(),
//             AnimationIndices::default(),
//             AnimationTimer(Timer::from_seconds(default(), TimerMode::Repeating)),
//             KinematicCharacterController::default(),
//             Collider::capsule_y(1.375, 4.),
//             Friction::coefficient(3.),
//             Velocity::zero(),
//             TerminalVelocity(Vec2::new(50., 200.)),
//             Acceleration(Vec2::new(300., 500.)),
//             NetDirection { x: 1, y: -1 },
//             Grounded::default(),
//         ));
//     }
// }

// fn slime_movement(mut slime_qry: Query<(&mut Slime, &mut Velocity, &mut Acceleration, &Grounded)>) {
//     for (mut slime, mut slime_vel, mut slime_acc, slime_grounded) in slime_qry.iter_mut() {
//         (*slime_vel, *slime_acc) = if slime_grounded.0 {
//             (Velocity::zero(), Acceleration(Vec2::ZERO))
//         } else {
//             (*slime_vel, Acceleration(Vec2::new(300., 500.)))
//         };
//         if slime.can_jump {
//             slime_vel.linvel.y = 200.;
//             slime.can_jump = false;
//         }
//     }
// }

// fn slime_animation(
//     mut slime_qry: Query<
//         (
//             &mut Slime,
//             &TextureAtlas,
//             &mut AnimationIndices,
//             &mut AnimationTimer,
//             &Grounded,
//         ),
//         With<Slime>,
//     >,
// ) {
//     for (
//         mut slime,
//         slime_tex_atlas,
//         mut slime_animation_idxs,
//         mut slime_animation_timer,
//         slime_grounded,
//     ) in slime_qry.iter_mut()
//     {
//         let (idle_idxs, idle_timer) = (
//             AnimationIndices { first: 0, last: 0 },
//             AnimationTimer(Timer::from_seconds(0., TimerMode::Repeating)),
//         );
//         let (jump_idxs, jump_timer) = (
//             AnimationIndices { first: 0, last: 3 },
//             AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
//         );
//         if slime_grounded.0 {
//             if *slime_animation_idxs != jump_idxs {
//                 *slime_animation_idxs = jump_idxs;
//                 *slime_animation_timer = jump_timer;
//             } else if slime_tex_atlas.index == jump_idxs.last {
//                 slime.can_jump = true;
//             }
//         } else if *slime_animation_idxs != idle_idxs {
//             *slime_animation_idxs = idle_idxs;
//             *slime_animation_timer = idle_timer;
//         }
//     }
// }
