use {super::game_state::GameState, bevy::prelude::*};

#[derive(Component)]
pub struct HitPoints(pub u8);

#[derive(Event)]
pub struct DamageDealtEvent {
    pub target: Entity,
    pub damage: u8,
}

fn on_damage_dealt(
    mut dmg_dealt_evr: EventReader<DamageDealtEvent>,
    mut hp_qry: Query<&mut HitPoints>,
) {
    for &DamageDealtEvent {
        target,
        damage: dmg,
    } in dmg_dealt_evr.read()
    {
        let Ok(mut hp) = hp_qry.get_mut(target) else {
            continue;
        };
        hp.0 = (hp.0 as i8 - dmg as i8).max(0) as u8;
    }
}

pub fn combat_plugin(app: &mut App) {
    app.add_event::<DamageDealtEvent>()
        .add_systems(Update, on_damage_dealt.run_if(in_state(GameState::Playing)));
}
