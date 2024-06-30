use {
    super::{
        combat::Damage,
        game_state::{GameState, PlayingEntity},
        level,
        texture_atlas_owner::TextureAtlasOwner,
        tile::{Tile, TILE_SIZE, TILE_Z},
    },
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
};

const SPIKE_Z: f32 = TILE_Z + 1.;
const SPIKE_COLLIDER_SIZE: Vec2 = Vec2::new(TILE_SIZE.x * 2. / 3., TILE_SIZE.y / 3.);

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
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(SPIKE_COLLIDER_SIZE.x / 2., SPIKE_COLLIDER_SIZE.y / 2.),
                Sensor,
                Damage::Kill,
                SpatialBundle::from_transform(Transform::from_xyz(
                    0.,
                    -(TILE_SIZE.y - SPIKE_COLLIDER_SIZE.y) / 2.,
                    0.,
                )),
            ));
        });
    }
}

pub fn spike_plugin(app: &mut App) {
    app.add_event::<SpikeSpawnEvent>().add_systems(
        OnEnter(GameState::Playing),
        on_spike_spawn.after(level::signal_level_object_spawns),
    );
}
