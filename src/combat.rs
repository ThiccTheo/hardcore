use {
    super::GameState,
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
    std::{f32::consts::TAU, time::Duration},
};

#[derive(Component)]
pub struct Health(pub i8);

#[derive(Component)]
pub enum Damage {
    Kill,
    Fixed(i8),
}

#[derive(Component)]
pub struct Iframes {
    timer: Timer,
}

impl Iframes {
    const FREQUENCY: f32 = 5.;
    const SECONDS_PER_DAMAGE: f32 = 0.5;

    pub fn new(duration: Duration) -> Self {
        Self {
            timer: Timer::new(duration, TimerMode::Once),
        }
    }
}

fn deal_damage(
    mut hp_qry: Query<(Entity, &mut Health, Has<Sensor>), (With<Collider>, Without<Iframes>)>,
    dmg_qry: Query<(Entity, &Damage, Has<Sensor>), With<Collider>>,
    rapier_ctx: Res<RapierContext>,
    mut cmds: Commands,
) {
    for (hp_id, mut hp, hp_has_sensor) in &mut hp_qry {
        for (dmg_id, dmg, dmg_has_sensor) in &dmg_qry {
            if (hp_id != dmg_id)
                && (hp_has_sensor || dmg_has_sensor)
                && (rapier_ctx.intersection_pair(hp_id, dmg_id) == Some(true))
            {
                match dmg {
                    Damage::Kill => hp.0 = 0,
                    &Damage::Fixed(dmg) => {
                        hp.0 -= dmg;
                        if dmg > 0 {
                            cmds.entity(hp_id)
                                .insert(Iframes::new(Duration::from_secs_f32(
                                    Iframes::SECONDS_PER_DAMAGE * dmg as f32,
                                )));
                        }
                    }
                }
            }
        }
        if hp.0 <= 0 {
            cmds.entity(hp_id).despawn_recursive();
        }
    }
}

fn update_iframes(
    time: Res<Time>,
    mut iframes_qry: Query<(Entity, &mut Iframes, &mut Sprite)>,
    mut cmds: Commands,
) {
    let dt = time.delta();

    for (id, mut invincible, mut sprite) in &mut iframes_qry {
        invincible.timer.tick(dt);
        sprite.color.set_a(f32::sin(
            invincible.timer.elapsed_secs() * Iframes::FREQUENCY * TAU,
        ));

        if invincible.timer.just_finished() {
            cmds.entity(id).remove::<Iframes>();
            sprite.color.set_a(1.);
        }
    }
}

pub fn combat_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (deal_damage, update_iframes)
            .chain()
            .run_if(in_state(GameState::Playing)),
    );
}
