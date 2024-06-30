use {
    super::{
        game_state::{GameState, PlayingEntity},
        level,
        texture_atlas_owner::TextureAtlasOwner,
        tile::Tile,
    },
    bevy::prelude::*,
};

const SPIKE_Z: f32 = 2.;

#[derive(Component)]
pub struct Spike;

#[derive(Event)]
pub struct SpikeSpawnEvent {
    pub pos: Vec2,
}

fn on_spike_spawn(
    mut spike_spawn_evr: EventReader<SpikeSpawnEvent>,
    mut cmds: Commands,
    tile_assets: Res<TextureAtlasOwner<Tile>>,
) {
    for &SpikeSpawnEvent { pos } in spike_spawn_evr.read() {
        cmds.spawn((
            Spike,
            PlayingEntity,
            SpriteSheetBundle {
                transform: Transform::from_translation(pos.extend(SPIKE_Z)),
                texture: tile_assets.tex.clone_weak(),
                atlas: TextureAtlas {
                    layout: tile_assets.layout.clone_weak(),
                    index: 70,
                },
                ..default()
            },
        ));
    }
}

pub fn spike_plugin(app: &mut App) {
    app.add_event::<SpikeSpawnEvent>().add_systems(
        OnEnter(GameState::Playing),
        on_spike_spawn.after(level::signal_level_object_spawns),
    );
}
