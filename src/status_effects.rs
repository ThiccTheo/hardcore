use {
    super::game_state::GameState,
    bevy::prelude::*,
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

pub struct Grounded;

fn update_invincibility(
    time: Res<Time>,
    mut iframes_qry: Query<(Entity, &mut Invincible, &mut Sprite)>,
    mut cmds: Commands,
) {
    let dt = time.delta();

    for (id, mut iframes, mut sprite) in iframes_qry.iter_mut() {
        iframes.timer.tick(dt);
        sprite.color.set_a(f32::sin(
            iframes.timer.elapsed_secs() * Invincible::IFRAMES_FREQUENCY * TAU,
        ));

        if iframes.timer.just_finished() {
            cmds.entity(id).remove::<Invincible>();
            sprite.color.set_a(1.);
        }
    }
}

fn update_grounded_ness(mut cmds: Commands) {

}

pub fn status_effects_plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_invincibility.run_if(in_state(GameState::Playing)),
    );
}
