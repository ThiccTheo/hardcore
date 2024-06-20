use {
    super::{game_state::GameState, level},
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
    asset_server: Res<AssetServer>,
    mut tex_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for &SpikeSpawnEvent { pos } in spike_spawn_evr.read() {
        cmds.spawn(SpriteSheetBundle {
            transform: Transform::from_translation(pos.extend(SPIKE_Z)),
            texture: asset_server.load("tile.png"),
            atlas: TextureAtlas {
                layout: tex_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    Vec2::splat(128.),
                    14,
                    7,
                    None,
                    None,
                )),
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
