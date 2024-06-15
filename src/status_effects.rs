use {
    super::game_state::GameState,
    bevy::prelude::*,
    bevy_ecs_tilemap::prelude::*,
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

#[derive(Component, Deref, DerefMut, Default)]
pub struct IsGrounded(pub bool);

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

fn update_is_grounded(
    mut is_grounded_qry: Query<(&Transform, &Collider, &mut IsGrounded), With<RigidBody>>,
    tile_qry: Query<(), (With<TilePos>, With<Collider>)>,
    rapier_ctx: Res<RapierContext>,
) {
    for (xform, collider, mut is_grounded) in is_grounded_qry.iter_mut() {
        let mut collider = collider.clone();
        collider.set_scale(collider.scale() * Vec2::new(0.9, 0.9), 100);

        is_grounded.0 = rapier_ctx
            .cast_shape(
                xform.translation.truncate(),
                xform.rotation.z,
                -Vec2::Y,
                &collider,
                1.,
                true,
                QueryFilter::new().predicate(&|id| tile_qry.contains(id)),
            )
            .is_some_and(|(_, toi)| toi.details.is_some_and(|deets| deets.normal1.y == 1.));
    }
}

pub fn status_effects_plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_invincibility.run_if(in_state(GameState::Playing)),
    )
    .add_systems(
        FixedPostUpdate,
        update_is_grounded.run_if(in_state(GameState::Playing)),
    );
}
