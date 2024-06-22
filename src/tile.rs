use {
    super::{game_state::GameState, level},
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
};

const TILE_Z: f32 = 1.;
pub const TILE_ID: u8 = TILE_Z as u8;
pub const TILE_SIZE: Vec2 = Vec2::splat(128.);

#[derive(Event)]
pub struct TileSpawnEvent {
    pub pos: Vec2,
    pub tex_idx: usize,
    pub has_collider: bool,
}

fn on_tile_spawn(
    mut tile_spawn_evr: EventReader<TileSpawnEvent>,
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut tex_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for &TileSpawnEvent {
        pos,
        tex_idx,
        has_collider,
    } in tile_spawn_evr.read()
    {
        let tile_id = cmds
            .spawn(SpriteSheetBundle {
                transform: Transform::from_translation(pos.extend(TILE_Z)),
                texture: asset_server.load("tile.png"),
                atlas: TextureAtlas {
                    layout: tex_atlas_layouts.add(TextureAtlasLayout::from_grid(
                        Vec2::splat(128.),
                        14,
                        7,
                        None,
                        None,
                    )),
                    index: tex_idx,
                },
                ..default()
            })
            .id();

        if has_collider {
            cmds.entity(tile_id)
                .insert(Collider::cuboid(TILE_SIZE.x / 2., TILE_SIZE.y / 2.));
        }
    }
}

pub fn tile_plugin(app: &mut App) {
    app.add_event::<TileSpawnEvent>().add_systems(
        OnEnter(GameState::Playing),
        (on_tile_spawn.after(level::signal_entity_spawns),).chain(),
    );
}
