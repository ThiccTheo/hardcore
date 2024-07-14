use {
    super::{asset_owner::TextureAtlasOwner, level},
    crate::GameState,
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
};

pub const TILE_Z: f32 = 1.;
pub const TILE_SIZE: Vec2 = Vec2::splat(128.);

#[derive(Component)]
pub struct Tile;

#[derive(Event)]
pub struct TileSpawnEvent {
    pub pos: Vec2,
    pub tex_idx: usize,
}

fn on_tile_spawn(
    mut tile_spawn_evr: EventReader<TileSpawnEvent>,
    mut cmds: Commands,
    tile_assets: Res<TextureAtlasOwner<Tile>>,
) {
    for &TileSpawnEvent { pos, tex_idx } in tile_spawn_evr.read() {
        cmds.spawn((
            Tile,
            StateScoped(GameState::Playing),
            SpriteBundle {
                transform: Transform::from_translation(pos.extend(TILE_Z)),
                texture: tile_assets.texture(),
                ..default()
            },
            TextureAtlas {
                layout: tile_assets.layout(),
                index: tex_idx,
            },
            Collider::cuboid(TILE_SIZE.x / 2., TILE_SIZE.y / 2.),
        ));
    }
}

pub fn tile_plugin(app: &mut App) {
    app.add_event::<TileSpawnEvent>()
        .add_systems(
            OnEnter(GameState::Setup),
            |mut cmds: Commands,
             asset_server: Res<AssetServer>,
             mut tex_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>| {
                cmds.insert_resource(TextureAtlasOwner::<Tile>::new(
                    asset_server.load("tile.png"),
                    tex_atlas_layouts.add(TextureAtlasLayout::from_grid(
                        UVec2::splat(128),
                        14,
                        7,
                        None,
                        None,
                    )),
                ));
            },
        )
        .add_systems(
            OnEnter(GameState::Playing),
            (on_tile_spawn.after(level::signal_level_object_spawns),).chain(),
        );
}
