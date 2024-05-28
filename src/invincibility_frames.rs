use {super::game_state::GameState, bevy::prelude::*, std::{f32::consts::TAU, time::Duration}};

const FLICKER_FREQUENCY: f32 = 5.;

pub struct InvincibilityFramesPlugin;

impl Plugin for InvincibilityFramesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_iframes.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct InvincibilityFrames {
    timer: Timer,
}

impl InvincibilityFrames {
    pub fn new(duration: Duration) -> Self {
        Self {
            timer: Timer::new(duration, TimerMode::Once),
        }
    }
}

fn update_iframes(
    time: Res<Time>,
    mut iframes_qry: Query<(Entity, &mut InvincibilityFrames, &mut Sprite)>,
    mut cmds: Commands,
) {
    let dt = time.delta();

    for (id, mut iframes, mut sprite) in iframes_qry.iter_mut() {
        iframes.timer.tick(dt);
        sprite.color.set_a(f32::sin(iframes.timer.elapsed_secs() * FLICKER_FREQUENCY * TAU));

        if iframes.timer.just_finished() {
            cmds.entity(id).remove::<InvincibilityFrames>();
            sprite.color.set_a(1.);
        }
    }
}
