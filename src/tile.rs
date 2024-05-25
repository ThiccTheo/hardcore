use {
    super::{game_state::GameState, level::LEVEL_SIZE},
    bevy::prelude::*,
    bevy_ecs_tilemap::prelude::*,
    bevy_rapier2d::prelude::*,
};

const TILE_Z: f32 = 2.;
const TILE_SIZE: Vec2 = Vec2::splat(16.);

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TileSpawnEvent>()
            .add_systems(OnEnter(GameState::Playing), spawn_tilemap)
            .add_systems(Update, on_tile_spawn.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Event)]
pub struct TileSpawnEvent {
    pub pos: Vec2,
    pub tex_idx: usize,
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
        texture: TilemapTexture::Single(asset_server.load("images/tile.png")),
        tile_size: TILE_SIZE.into(),
        transform: get_tilemap_center_transform(
            &TilemapSize::new(LEVEL_SIZE.x as u32, LEVEL_SIZE.y as u32),
            &TILE_SIZE.into(),
            &TilemapType::Square,
            1.,
        ),
        ..default()
    });
}

fn on_tile_spawn(
    mut tile_spawn_evr: EventReader<TileSpawnEvent>,
    mut cmds: Commands,
    mut tilemap_qry: Query<(Entity, &mut TileStorage, &Transform)>,
) {
    let (tilemap_id, mut tile_storage, tilemap_xform) = tilemap_qry.single_mut();

    for &TileSpawnEvent { pos, tex_idx } in tile_spawn_evr.read() {
        let tile_pos = TilePos::new(pos.x as u32, pos.y as u32);
        let tile_id = cmds
            .spawn((TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_id),
                texture_index: TileTextureIndex(tex_idx as u32),
                ..default()
            },))
            .id();
        tile_storage.set(&tile_pos, tile_id);

        let Vec2 { x, y } = tile_pos.center_in_world(&TILE_SIZE.into(), &TilemapType::Square);
        cmds.entity(tile_id).insert((
            TransformBundle::from_transform(*tilemap_xform * Transform::from_xyz(x, y, TILE_Z)),
            Collider::cuboid(TILE_SIZE.x / 2., TILE_SIZE.y / 2.),
        ));
    }
}
