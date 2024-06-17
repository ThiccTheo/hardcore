use {
    super::{
        game_state::GameState,
        level::{self, LEVEL_SIZE},
    },
    bevy::prelude::*,
    bevy_ecs_tilemap::prelude::*,
    bevy_rapier2d::prelude::*,
};

const TILE_Z: f32 = 1.;
pub const TILE_ID: u8 = TILE_Z as u8;
pub const TILE_SIZE: Vec2 = Vec2::splat(128.);

#[derive(Event)]
pub struct TileSpawnEvent {
    pub tile_pos: TilePos,
    pub world_pos: Vec2,
    pub tex_idx: TileTextureIndex,
}

fn spawn_tilemap(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.spawn(TilemapBundle {
        grid_size: TILE_SIZE.into(),
        map_type: TilemapType::Square,
        size: TilemapSize {
            x: LEVEL_SIZE.x as u32,
            y: LEVEL_SIZE.y as u32,
        },
        storage: TileStorage::empty(TilemapSize {
            x: LEVEL_SIZE.x as u32,
            y: LEVEL_SIZE.y as u32,
        }),
        texture: TilemapTexture::Single(asset_server.load("bruh.png")),
        tile_size: TILE_SIZE.into(),
        transform: get_tilemap_center_transform(
            &TilemapSize::new(LEVEL_SIZE.x as u32, LEVEL_SIZE.y as u32),
            &TILE_SIZE.into(),
            &TilemapType::Square,
            TILE_Z,
        ),
        ..default()
    });
}

fn on_tile_spawn(
    mut tile_spawn_evr: EventReader<TileSpawnEvent>,
    mut cmds: Commands,
    mut tilemap_qry: Query<(Entity, &mut TileStorage)>,
) {
    let (tilemap_id, mut tile_storage) = tilemap_qry.single_mut();

    for &TileSpawnEvent {
        tile_pos,
        world_pos,
        tex_idx,
    } in tile_spawn_evr.read()
    {
        let tile_id = cmds
            .spawn((
                TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_id),
                    texture_index: tex_idx,
                    ..default()
                },
                TransformBundle::from_transform(Transform::from_translation(
                    world_pos.extend(TILE_Z),
                )),
            ))
            .id();
        tile_storage.set(&tile_pos, tile_id);

        if tex_idx.0 == 10 {
            cmds.entity(tile_id)
                .insert(Collider::cuboid(TILE_SIZE.x / 2., TILE_SIZE.y / 2.));
        }
    }
}

pub fn tile_plugin(app: &mut App) {
    app.add_event::<TileSpawnEvent>().add_systems(
        OnEnter(GameState::Playing),
        (
            spawn_tilemap.before(level::signal_entity_spawns),
            on_tile_spawn.after(level::signal_entity_spawns),
        )
            .chain(),
    );
}
