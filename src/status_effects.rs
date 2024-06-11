use {
    super::game_state::GameState,
    bevy::{ecs::entity::EntityHashMap, prelude::*},
    bevy_rapier2d::prelude::*,
    std::{f32::consts::TAU, time::Duration},
};

#[derive(Component)]
pub struct Invincible {
    timer: Timer,
}

impl Invincible {
    const IFRAMES_FREQUENCY: f32 = 5.;

    pub fn new(duration: Duration) -> Self {
        Self {
            timer: Timer::new(duration, TimerMode::Once),
        }
    }
}

// HAS MOVED COULD BE BETTER
#[derive(Component, Deref, DerefMut, Default)]
pub struct IsGrounded(pub bool);

#[derive(Resource, Default, Deref, DerefMut)]
struct InitialYValues(EntityHashMap<f32>);

fn update_invincibility(
    time: Res<Time>,
    mut invincibility_qry: Query<(Entity, &mut Invincible, &mut Sprite)>,
    mut cmds: Commands,
) {
    let dt = time.delta();

    for (id, mut invincible, mut sprite) in invincibility_qry.iter_mut() {
        invincible.timer.tick(dt);
        sprite.color.set_a(f32::sin(
            invincible.timer.elapsed_secs() * Invincible::IFRAMES_FREQUENCY * TAU,
        ));

        if invincible.timer.just_finished() {
            cmds.entity(id).remove::<Invincible>();
            sprite.color.set_a(1.);
        }
    }
}

fn collect_initial_y_values(
    mut y0s: ResMut<InitialYValues>,
    is_grounded_qry: Query<(Entity, &Transform), (With<RigidBody>, With<IsGrounded>)>,
) {
    y0s.extend(
        is_grounded_qry
            .iter()
            .map(|(id, xform)| (id, xform.translation.y)),
    );
}

fn update_is_grounded(
    mut is_grounded_qry: Query<(Entity, &Transform, &mut IsGrounded), With<RigidBody>>,
    mut y0s: ResMut<InitialYValues>,
) {
    for (id, xform, mut is_grounded) in is_grounded_qry.iter_mut() {
        is_grounded.0 = y0s.get(&id).is_some_and(|&y0| y0 == xform.translation.y);
    }
    y0s.clear();
}

pub fn status_effects_plugin(app: &mut App) {
    app.insert_resource(InitialYValues::default())
        .add_systems(
            Update,
            update_invincibility.run_if(in_state(GameState::Playing)),
        )
        .add_systems(FixedPreUpdate, collect_initial_y_values)
        .add_systems(FixedPostUpdate, update_is_grounded);
}
