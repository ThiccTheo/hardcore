use {super::game_state::GameState, bevy::prelude::*, bevy_rapier2d::prelude::*};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (process_collisions, apply_forces)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct TerminalVelocity(pub Vec2);

#[derive(Component, Default, Deref, DerefMut)]
pub struct Acceleration(pub Vec2);

#[derive(Component, Default)]
pub struct NetDirection {
    pub x: i8,
    pub y: i8,
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct Grounded(pub bool);

fn is_colliding(normal: Vec2, threshold: f32, axis: Vec2) -> bool {
    let dot_prod = normal.normalize().dot(axis);
    dot_prod > threshold || dot_prod < -threshold
}

pub fn apply_forces(
    mut physics_qry: Query<(
        &mut KinematicCharacterController,
        &mut Velocity,
        &TerminalVelocity,
        &Friction,
        &Acceleration,
        &NetDirection,
    )>,
    time: Res<Time<Fixed>>,
) {
    let dt = time.timestep().as_secs_f32();

    for (mut kcc, mut vel, terminal_vel, friction, acc, net_dir) in physics_qry.iter_mut() {
        vel.linvel.x += acc.x * net_dir.x as f32 * dt;
        vel.linvel.y += acc.y * net_dir.y as f32 * dt;

        let dir = vel.linvel.normalize_or_zero();
        if dir.x > 0. {
            vel.linvel.x = f32::max(0., vel.linvel.x - friction.coefficient);
        } else if dir.x < 0. {
            vel.linvel.x = f32::min(vel.linvel.x + friction.coefficient, 0.);
        }
        vel.linvel.x = vel.linvel.x.clamp(-terminal_vel.x, terminal_vel.x);
        vel.linvel.y = vel.linvel.y.clamp(-terminal_vel.y, terminal_vel.y);

        let mut pos = Vec2::ZERO;
        pos += vel.linvel * dt;
        kcc.translation = Some(pos);
    }
}

pub fn process_collisions(
    mut physics_qry: Query<
        (
            &KinematicCharacterControllerOutput,
            &mut Velocity,
            Option<&mut Grounded>,
        ),
        With<Collider>,
    >,
) {
    for (kcc_out, mut vel, mut grounded) in physics_qry.iter_mut() {
        for collision in kcc_out.collisions.iter() {
            let threshold = 0.8;
            let Some(deets) = collision.toi.details else {
                continue;
            };
            if is_colliding(deets.normal2, threshold, Vec2::X) {
                vel.linvel.x = 0.;
            }
            if is_colliding(deets.normal2, threshold, Vec2::Y) {
                vel.linvel.y = 0.;
            }
        }
        let Some(grounded) = grounded.as_mut() else {
            continue;
        };
        grounded.0 = vel.linvel.y == 0.;
    }
}
