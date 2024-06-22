use {
    super::{game_state::GameState, level, tile::TileAssets},
    bevy::prelude::*,
};

const SPIKE_Z: f32 = 2.;
pub const SPIKE_ID: u8 = SPIKE_Z as u8;

#[derive(Event)]
pub struct SpikeSpawnEvent {
    pub pos: Vec2,
}

fn on_spike_spawn(
    mut spike_spawn_evr: EventReader<SpikeSpawnEvent>,
    mut cmds: Commands,
    tile_assets: Res<TileAssets>,
) {
    for &SpikeSpawnEvent { pos } in spike_spawn_evr.read() {
        cmds.spawn(SpriteSheetBundle {
            transform: Transform::from_translation(pos.extend(SPIKE_Z)),
            texture: tile_assets.tex.clone_weak(),
            atlas: TextureAtlas {
                layout: tile_assets.layout.clone_weak(),
                index: 70,
            },
            ..default()
        });
    }
}

pub fn spike_plugin(app: &mut App) {
    app.add_event::<SpikeSpawnEvent>().add_systems(
        OnEnter(GameState::Playing),
        on_spike_spawn.after(level::signal_entity_spawns),
    );
}
