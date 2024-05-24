use {
    super::game_state::GameState,
    bevy::prelude::*,
    bevy_ecs_tilemap::prelude::*,
    bevy_rapier2d::prelude::*,
};

const TILE_Z: f32 = 2.;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (spawn_tilemap, add_colliders_to_blocks).chain(),
        );
    }
}

#[derive(Component)]
struct Tile;

fn spawn_tilemap(mut cmds: Commands, asset_server: Res<AssetServer>) {
    let tilemap_id = cmds.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(TilemapSize {
        x: WORLD_SIZE.x as u32,
        y: WORLD_SIZE.y as u32,
    });

    for y in 0..WORLD_SIZE.y as u32 {
        for x in 0..WORLD_SIZE.x as u32 {
            let perlin_val = perlin_map.get_value(x as usize, y as usize);
            if perlin_val > 0.007 || y > 100 {
                continue;
            };

            let block_pos = TilePos { x, y };
            let block_id = cmds
                .spawn((
                    Block,
                    TileBundle {
                        texture_index: TileTextureIndex(if y == 100 { 0 } else if y < 50 { 2 } else { 1 }),
                        position: block_pos,
                        tilemap_id: TilemapId(blockmap_id),
                        ..default()
                    },
                ))
                .id();

            block_storage.set(&block_pos, block_id);
        }
    }

    let block_size = TilemapTileSize::new(16., 16.);
    cmds.entity(blockmap_id).insert(TilemapBundle {
        grid_size: block_size.into(),
        map_type: TilemapType::Square,
        size: TilemapSize {
            x: WORLD_SIZE.x as u32,
            y: WORLD_SIZE.y as u32,
        },
        storage: block_storage,
        texture: TilemapTexture::Single(asset_server.load("tile.png")),
        tile_size: block_size,
        transform: get_tilemap_center_transform(
            &TilemapSize {
                x: WORLD_SIZE.x as u32,
                y: WORLD_SIZE.y as u32,
            },
            &block_size.into(),
            &TilemapType::Square,
            1.,
        ),
        ..default()
    });
}

fn add_colliders_to_blocks(
    mut cmds: Commands,
    block_tilemap_qry: Query<(&TileStorage, &Transform), With<BlockTilemap>>,
    block_pos_qry: Query<&TilePos, (With<Block>, Without<Collider>)>,
) {
    let (block_storage, &block_tilemap_xform) = block_tilemap_qry.single();

    for &tile_id in block_storage.iter().flatten() {
        let Ok(tile_pos) = block_pos_qry.get(tile_id) else {
            continue;
        };
        let Vec2 { x, y } =
            tile_pos.center_in_world(&Vec2::splat(16.).into(), &TilemapType::Square);

        cmds.entity(tile_id).insert((
            TransformBundle::from_transform(block_tilemap_xform * Transform::from_xyz(x, y, BLOCK_Z)),
            Collider::cuboid(8., 8.),
        ));
    }
}
